## Proof of concept: loads Canyon City of Zant from the original data files and puts an
## animated character on the terrain.
##
## Command-line options (after "--"):
##   --data-idx=PATH            ROSE data.idx (or set ROSE_DATA_IDX; otherwise the last one
##                              picked, a ROSE folder next to the game, or ask)
##   --zone=ID                  zone to load (default 1, Canyon City of Zant)
##   --free-camera=x,y,z,yaw,pitch   fixed camera, degrees, same convention as the Bevy zone viewer
##   --screenshot=PATH          save one frame and quit
##   --time=morning|day|evening|night|TICKS   fixed time of day (default: day, then the clock runs)
##   --demo                     scripted run / attack / cancel sequence (for --write-movie)
##   --server=URI               play online on this SpacetimeDB server, skipping the start screen
##   --name=NAME                character name for --server
##   --weapon=bow               equip the bow and arrows from the bag after signing in
##   --profile=NAME             identity file to use (user://identity-NAME.token), so two
##                              clients on one PC are two players
##   --offline                  skip the start screen and play offline
##   --net-demo[=square|line|fight|walls|warp|shop|skills|talk|party|craft]   online: scripted routes, fights,
##                              wall tests, warp gates, buying and selling at the nearest
##                              store (--buy=TEXT: what to buy), the first two active
##                              skills (a buff, then an attack), or a talk with the NPC
##                              --npc=NAME, answering --answers=1,2,... (q: list quests;
##                              deposit/withdraw: the bank; pick=NAME puts a bag item in
##                              the refine/disassemble window, work presses its button,
##                              gem=NAME sets a gem, skill=NAME uses a skill), or a party (--invite=NAME
##                              invites, else accept; --rules=XP,ITEMS) that then fights,
##                              or crafting with the first craft skill (--craft=NAME,
##                              --times=N), or a trade (--trade-with=NAME asks, else
##                              accept; --offer=ITEM puts a bag item up, --zuly=N money,
##                              --hold=SECONDS waits before pressing Trade), or a PvP
##                              fight with --fight=NAME (--leave-party first)
##   --net-log                  online: print every player's position once a second
##   --open=inventory,character,skills,quests   online: open these windows at the start (for screenshots)
##   --quit-after=SECONDS       quit after this long
extends Node3D

const START := Vector3(5210.5, 0.0, -5136.7)  # zone start position from LIST_ZONE.STB
const SETTINGS := "user://settings.cfg"

var zone: RoseZone
var player: Node3D  # offline character
var online: Node3D  # online world, when connected
var camera_anchor: Node3D
var camera: Camera3D
var options := {}
var world_ticks := 0.0  # one world tick is 10 seconds
var loaded_zone := 0


var _joined_before := false

func _ready() -> void:
	for arg in OS.get_cmdline_user_args():
		var parts := arg.trim_prefix("--").split("=", true, 1)
		options[parts[0]] = parts[1] if parts.size() > 1 else "true"

	var data_idx := _find_data_idx()
	if data_idx == "":
		_ask_for_data_idx()
	else:
		_start(data_idx)


## data.idx from --data-idx, ROSE_DATA_IDX, the last one picked, or a ROSE folder next to
## the game. Empty if none of them exists.
func _find_data_idx() -> String:
	var settings := ConfigFile.new()
	settings.load(SETTINGS)
	var exe_dir := OS.get_executable_path().get_base_dir()
	var candidates := [
		options.get("data-idx", ""),
		OS.get_environment("ROSE_DATA_IDX"),
		settings.get_value("data", "data_idx", ""),
		exe_dir.path_join("iRose_129_129/data.idx"),
		exe_dir.path_join("data/data.idx"),
		exe_dir.path_join("data.idx"),
	]
	for candidate in candidates:
		if candidate != "" and FileAccess.file_exists(candidate):
			return candidate
	if options.has("data-idx"):
		push_error("rose: no data.idx at " + options["data-idx"])
	return ""


## First start on a new PC: ask where the ROSE client's data.idx is, and remember it.
func _ask_for_data_idx() -> void:
	var dialog := FileDialog.new()
	dialog.file_mode = FileDialog.FILE_MODE_OPEN_FILE
	dialog.title = "Where is the ROSE client? Pick its data.idx"
	dialog.access = FileDialog.ACCESS_FILESYSTEM
	dialog.filters = PackedStringArray(["data.idx ; ROSE data index"])
	dialog.use_native_dialog = true
	dialog.file_selected.connect(func(path: String):
		var settings := ConfigFile.new()
		settings.load(SETTINGS)
		settings.set_value("data", "data_idx", path)
		settings.save(SETTINGS)
		_start(path))
	dialog.canceled.connect(get_tree().quit)
	add_child(dialog)
	dialog.popup_centered_ratio(0.6)


func _start(data_idx: String) -> void:
	var started := Time.get_ticks_msec()
	if not RoseData.open(data_idx):
		get_tree().quit(1)
		return
	print("rose: data tables loaded in %d ms" % (Time.get_ticks_msec() - started))

	if not _load_zone(int(options.get("zone", "1"))):
		get_tree().quit(1)
		return

	camera_anchor = Node3D.new()
	add_child(camera_anchor)
	camera_anchor.position = Vector3(START.x, zone.get_terrain_height(START.x, START.z), START.z)

	if options.has("free-camera"):
		var v: PackedFloat64Array = options["free-camera"].split_floats(",")
		camera = Camera3D.new()
		camera.position = Vector3(v[0], v[1], v[2])
		camera.rotation = Vector3(deg_to_rad(v[4]), deg_to_rad(v[3]), 0.0)
	else:
		camera = preload("res://scripts/orbit_camera.gd").new()
		camera.target = camera_anchor
	camera.fov = float(options.get("fov", "45"))
	camera.near = 0.1
	camera.far = 1000.0
	add_child(camera)
	camera.make_current()

	# Colours already match rose-offline-client without it, so glow (its bloom) is opt-in.
	var environment := Environment.new()
	environment.background_mode = Environment.BG_COLOR
	environment.background_color = Color(0.2, 0.2, 0.2)
	environment.tonemap_mode = Environment.TONE_MAPPER_LINEAR
	environment.glow_enabled = options.get("glow", "false") == "true"
	environment.glow_normalized = true
	environment.glow_intensity = float(options.get("glow-intensity", "0.6"))
	environment.glow_bloom = float(options.get("glow-bloom", "0.15"))
	environment.glow_blend_mode = Environment.GLOW_BLEND_MODE_SOFTLIGHT
	var world_environment := WorldEnvironment.new()
	world_environment.environment = environment
	add_child(world_environment)

	if options.has("quit-after"):
		get_tree().create_timer(float(options["quit-after"])).timeout.connect(get_tree().quit)
	if options.has("server"):
		_go_online(options["server"], options.get("name", ""), options.get("weapon", "sword") == "bow")
	elif options.has("offline") or options.has("demo") or options.has("screenshot") or options.has("free-camera"):
		_go_offline()
	else:
		var panel = preload("res://scripts/connect_panel.gd").new()
		panel.connect_requested.connect(_go_online)
		panel.offline_requested.connect(_go_offline)
		add_child(panel)

	if options.has("demo"):
		_run_demo()
	if options.has("screenshot"):
		_take_screenshot(options["screenshot"])


## Loads a zone in place of the current one, with its lighting at the current time of day.
func _load_zone(zone_id: int) -> bool:
	if zone:
		zone.queue_free()
		remove_child(zone)
	zone = RoseZone.new()
	zone.name = "Zone"
	add_child(zone)
	move_child(zone, 0)
	if not zone.load_zone(zone_id):
		return false
	loaded_zone = zone_id
	zone.set_meta("zone_id", zone_id)
	print("rose: zone ", zone_id, " loaded ", zone.get_stats(), " day cycle ", zone.get_day_cycle(), " morning/day/evening/night start ", [zone.get_state_start("morning"), zone.get_state_start("day"), zone.get_state_start("evening"), zone.get_state_start("night")])
	if world_ticks == 0.0:
		var time: String = options.get("time", "day")
		world_ticks = float(time) if time.is_valid_int() else float(zone.get_state_start(time)) + 1.0
	_apply_lighting(zone.get_lighting_at(int(world_ticks), 0.0))
	return true


## The server has our character in another zone: load it and show that zone's entities.
func _on_zone_needed(zone_id: int) -> void:
	if zone_id == loaded_zone:
		return
	if _load_zone(zone_id):
		online.use_zone(zone)


func _go_offline() -> void:
	player = preload("res://scripts/player.gd").new()
	player.name = "Player"
	add_child(player)
	player.setup(zone, true, {"face": 1, "hair": 0, "body": 1, "hands": 1, "feet": 1, "weapon": 2})
	player.place(START)
	if camera.get("target") != null:
		camera.target = player


func _go_online(uri: String, name_text: String, use_bow: bool) -> void:
	var profile: String = options.get("profile", "default")
	var token_path := ProjectSettings.globalize_path("user://identity-%s.token" % profile)
	online = preload("res://scripts/online.gd").new()
	online.name = "Online"
	add_child(online)
	online.joined.connect(_on_joined)
	online.zone_needed.connect(_on_zone_needed)
	online.log_positions = options.has("net-log")
	online.log_damage = options.has("net-log") or options.has("net-demo")
	online.start(zone, uri, token_path, name_text, use_bow)


func _on_joined(me: Node3D) -> void:
	if camera.get("target") != null:
		camera.target = me
	# Joining again after a warp keeps the windows and demo of the first join.
	if _joined_before:
		return
	_joined_before = true
	var windows: String = options.get("open", "")
	if "inventory" in windows:
		online.toggle_inventory_window()
	if "character" in windows:
		online.toggle_character_window()
	if "skills" in windows:
		online.toggle_skill_window()
	if "quests" in windows:
		online.toggle_quest_window()
	if options.has("net-demo"):
		_run_net_demo(me)


## Walks a route from where the character joined, forever: "square" (default) goes round
## the start point 4 m out, "line" goes back and forth 5 m east.
func _run_net_demo(me: Node3D) -> void:
	var origin := me.position
	var route := [Vector3(4, 0, 2), Vector3(4, 0, -6), Vector3(-4, 0, -6), Vector3(-4, 0, 2)]
	var pause := 2.6
	if options["net-demo"] == "fight":
		_run_fight_demo()
		return
	if options["net-demo"] == "walls":
		_run_walls_demo(origin)
		return
	if options["net-demo"] == "warp":
		_run_warp_demo()
		return
	if options["net-demo"] == "shop":
		_run_shop_demo()
		return
	if options["net-demo"] == "skills":
		_run_skills_demo()
		return
	if options["net-demo"] == "talk":
		_run_talk_demo()
		return
	if options["net-demo"] == "party":
		_run_party_demo()
		return
	if options["net-demo"] == "craft":
		_run_craft_demo()
	if options["net-demo"] == "trade":
		_run_trade_demo()
	if options["net-demo"] == "pvp":
		_run_pvp_demo()
		return
	if options["net-demo"] == "line":
		route = [Vector3(5, 0, 0), Vector3(0, 0, 0)]
		pause = 3.5
	var i := 0
	while true:
		await get_tree().create_timer(pause).timeout
		var target: Vector3 = origin + route[i % route.size()]
		print("rose net demo: move to ", target)
		online.move_to(target)
		i += 1


func _apply_lighting(lighting: Dictionary) -> void:
	RenderingServer.global_shader_parameter_set("rose_map_ambient", lighting["map_ambient"])
	RenderingServer.global_shader_parameter_set("rose_character_ambient", lighting["character_ambient"])
	RenderingServer.global_shader_parameter_set("rose_character_diffuse", lighting["character_diffuse"])
	RenderingServer.global_shader_parameter_set("rose_light_direction", lighting["light_direction"])
	RenderingServer.global_shader_parameter_set("rose_fog_color", lighting["fog_color"])
	RenderingServer.global_shader_parameter_set("rose_fog_density", lighting["fog_density"])
	RenderingServer.global_shader_parameter_set("rose_sky_day_weight", lighting["day_weight"])


func _process(delta: float) -> void:
	if zone == null or options.has("screenshot"):
		return
	if online and options.has("net-log") and Engine.get_process_frames() % 60 == 0:
		# For click tests: the monster drawn nearest the middle of the screen.
		var middle := get_viewport().get_visible_rect().size / 2.0
		var best := ""
		var best_distance := INF
		for id in online.entities:
			var monster: Node3D = online.entities[id]
			var centre := monster.global_position + Vector3(0, monster.height * 0.5, 0)
			if monster.is_monster and not camera.is_position_behind(centre):
				var at := camera.unproject_position(centre)
				if at.distance_to(middle) < best_distance:
					best_distance = at.distance_to(middle)
					best = "rose net: monster %s (entity %d) on screen at %s" % [monster.label.text, id, at]
		if best != "":
			print(best)
	world_ticks += delta / 10.0
	_apply_lighting(zone.get_lighting_at(int(world_ticks), fmod(world_ticks, 1.0)))


func _take_screenshot(path: String) -> void:
	for i in 10:
		await get_tree().process_frame
	await RenderingServer.frame_post_draw
	get_viewport().get_texture().get_image().save_png(path)
	print("rose: saved ", path, " (draw calls %d, objects %d, primitives %d)" % [
		Performance.get_monitor(Performance.RENDER_TOTAL_DRAW_CALLS_IN_FRAME),
		Performance.get_monitor(Performance.RENDER_TOTAL_OBJECTS_IN_FRAME),
		Performance.get_monitor(Performance.RENDER_TOTAL_PRIMITIVES_IN_FRAME)])
	get_tree().quit()


func _unhandled_input(event: InputEvent) -> void:
	if online and event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_RIGHT:
		var other: int = online.pick_player(camera, event.position)
		if other >= 0:
			online.open_player_menu(other, event.position)
			return
	if event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_LEFT:
		if online:
			var monster: int = online.pick_monster(camera, event.position)
			if monster >= 0:
				online.attack(monster)
				return
			var enemy: int = online.pick_enemy_player(camera, event.position)
			if enemy >= 0:
				online.attack(enemy)
				return
			var npc: int = online.pick_npc(camera, event.position)
			if npc >= 0:
				online.talk_to(npc)
				return
			var item: int = online.pick_item(camera, event.position)
			if item >= 0:
				online.pickup(item)
				return
		var hit = _pick_ground(event.position)
		if hit != null:
			if online:
				online.move_to(hit)
			elif player:
				player.move_to(hit)
	elif event is InputEventKey and event.pressed and not event.echo:
		if event.keycode == KEY_SPACE:
			if online:
				online.attack(online.nearest_monster())
			elif player:
				player.attack()
		elif event.keycode == KEY_S:
			if online:
				online.stop()
			elif player:
				player.stop()
		elif event.keycode == KEY_C and online:
			online.toggle_character_window()
		elif event.keycode == KEY_I and online:
			online.toggle_inventory_window()
		elif event.keycode == KEY_K and online:
			online.toggle_skill_window()
		elif event.keycode == KEY_Q and online:
			online.toggle_quest_window()
		elif event.keycode >= KEY_1 and event.keycode <= KEY_8 and online:
			online.hotbar.use_slot(event.keycode - KEY_1)
		elif event.keycode == KEY_Z and online:
			online.pickup(online.nearest_item())


## Runs 20 m (--wall-reach) out in eight (--wall-directions) directions from the start, coming back each time, to test
## collision with zone objects.
func _run_walls_demo(origin: Vector3) -> void:
	var count := int(options.get("wall-directions", "8"))
	var reach := float(options.get("wall-reach", "20"))
	for i in count:
		var angle := i * TAU / count
		var target := origin + Vector3(cos(angle), 0, sin(angle)) * reach
		print("rose net demo: run toward (%.1f, %.1f)" % [target.x, target.z])
		online.move_to(target)
		await get_tree().create_timer(reach / 4.5 + 1.0).timeout
		print("rose net demo: stopped at (%.2f, %.2f), %.1f m out" % [online.me.position.x, online.me.position.z, Vector2(online.me.position.x - origin.x, online.me.position.z - origin.z).length()])
		online.move_to(origin)
		await get_tree().create_timer(reach / 4.5 + 1.0).timeout
	get_tree().quit()


## Walks into the nearest warp gate, then (in the zone it leads to) into that zone's
## nearest one, three times.
func _run_warp_demo() -> void:
	var trips := 0
	while trips < 3:
		await get_tree().create_timer(4.0).timeout
		while online.me == null:
			await get_tree().create_timer(0.5).timeout
		var me: Node3D = online.me
		var best = null
		for warp in online._warps:
			var centre: Vector3 = warp["position"] + warp["size"] / 2.0
			if best == null or centre.distance_to(me.position) < best.distance_to(me.position):
				best = centre
		if best == null:
			print("rose net demo: no warp gate here")
			continue
		print("rose net demo: zone %d, walk to the warp gate at (%.1f, %.1f), %.0f m away" % [loaded_zone, best.x, best.z, best.distance_to(me.position)])
		var zone_before := loaded_zone
		online.move_to(Vector3(best.x, 0, best.z))
		var waited := 0.0
		while loaded_zone == zone_before and waited < 120.0:
			await get_tree().create_timer(0.5).timeout
			waited += 0.5
		print("rose net demo: now in zone %d" % loaded_zone)
		trips += 1


## Puts the first two active skills on the hotbar, uses the first (a buff) and then the
## second on the nearest monster, over and over, between normal attacks.
func _run_skills_demo() -> void:
	await get_tree().create_timer(4.0).timeout
	var active: Array = online.net.get_skills()[1]
	var names := []
	for i in 2:
		if active[i] != null:
			online.net.set_hotbar(i, "skill", 1, i)
			names.append(active[i]["name"])
	print("rose net demo: hotbar skills ", names)
	while true:
		await get_tree().create_timer(1.0).timeout
		online.hotbar.use_slot(0)
		print("rose net demo: use %s, MP %d" % [names[0] if names.size() > 0 else "?", online.me.mp])
		await get_tree().create_timer(3.0).timeout
		var effects: Array = online.net.get_status_effects(online.my_id)
		print("rose net demo: status effects ", effects.map(func(e): return "%s %.0f s" % [e["name"], e["seconds"]]))
		for round in 3:
			var target: int = online.nearest_monster()
			if target < 0:
				break
			online.attack(target)
			await get_tree().create_timer(0.3).timeout
			online.hotbar.use_slot(1)
			print("rose net demo: use %s on %s, MP %d" % [names[1] if names.size() > 1 else "?", online.entities[target].label.text if online.entities.has(target) else "?", online.me.mp])
			await get_tree().create_timer(3.0).timeout


## Walks to the NPC named --npc (part of the name), prints what it says, and answers with
## --answers (1-based, comma separated; "q" prints the quest list), then prints our quests.
func _run_talk_demo() -> void:
	await get_tree().create_timer(3.0).timeout
	var wanted := String(options.get("npc", "")).to_lower()
	var npc := -1
	var nearest := INF
	for id in online.entities:
		var entity: Node3D = online.entities[id]
		var d: float = entity.position.distance_to(online.me.position)
		if entity.is_npc and wanted in String(entity.label.text).to_lower() and d < nearest:
			npc = id
			nearest = d
	if npc < 0:
		var names := []
		for id in online.entities:
			if online.entities[id].is_npc:
				names.append(online.entities[id].label.text)
		print("rose net demo: no NPC called ", wanted, " here, only ", names)
		return
	print("rose net demo: walk to %s, %.0f m away" % [online.entities[npc].label.text, online.entities[npc].position.distance_to(online.me.position)])
	online.talk_to(npc)
	var waited := 0.0
	while not online.conversation_window.visible and not online.store_window.visible and waited < 60.0:
		await get_tree().create_timer(0.5).timeout
		waited += 0.5
	_print_conversation()
	var answers := String(options.get("answers", ""))
	for answer in answers.split(",", false):
		await get_tree().create_timer(1.5).timeout
		if answer == "q":
			_print_quests()
			continue
		if answer == "deposit":
			# The first item in the bag's consumables page, whole stack.
			var bag: Array = online.net.get_inventory()["pages"][1]
			for i in bag.size():
				if bag[i] != null:
					print("rose net demo: deposit %s x%d" % [bag[i]["name"], bag[i].get("quantity", 1)])
					online.bank_window.deposit(1, i, bag[i].get("quantity", 1))
					break
			await get_tree().create_timer(1.0).timeout
			_print_bank()
			continue
		if answer == "withdraw":
			var bank: Array = online.net.get_bank()
			for i in bank.size():
				if bank[i] != null:
					print("rose net demo: withdraw %s x%d" % [bank[i]["name"], bank[i].get("quantity", 1)])
					online.bank_window.withdraw(i, bank[i].get("quantity", 1))
					break
			await get_tree().create_timer(1.0).timeout
			_print_bank()
			continue
		if answer.begins_with("pick="):
			var found := _find_bag_item(answer.trim_prefix("pick="))
			if found.is_empty():
				print("rose net demo: no ", answer.trim_prefix("pick="), " in the bag")
			else:
				online.work_window.set_target(found[0], found[1])
				await get_tree().create_timer(1.5).timeout
				print("rose net demo: %s: %s | %s" % [online.work_window.title.text, online.work_window.info.text.replace("\n", " / "), online.work_window.cost.text])
			continue
		if answer == "work":
			online.work_window.work()
			await get_tree().create_timer(2.5).timeout
			print("rose net demo: now: %s" % online.work_window.info.text.replace("\n", " / "))
			continue
		if answer.begins_with("skill="):
			# Use our skill with this name (e.g. Item Refining opens the refine window).
			var pages: Array = online.net.get_skills()
			for page in pages.size():
				for index in pages[page].size():
					var skill = pages[page][index]
					if skill != null and answer.trim_prefix("skill=").to_lower() in String(skill["name"]).to_lower():
						online.use_skill(page, index, skill)
			await get_tree().create_timer(1.0).timeout
			continue
		if answer == "wait":
			await get_tree().create_timer(6.0).timeout
			continue
		if answer.begins_with("equip="):
			var found := _find_bag_item(answer.trim_prefix("equip="))
			if not found.is_empty():
				online.net.equip_item(found[0], found[1])
				await get_tree().create_timer(1.0).timeout
			continue
		if answer.begins_with("use="):
			var found := _find_bag_item(answer.trim_prefix("use="))
			if not found.is_empty():
				online.net.use_item(found[0], found[1])
				await get_tree().create_timer(1.5).timeout
			continue
		if answer == "unequip":
			online.net.unequip_item(6)
			await get_tree().create_timer(1.0).timeout
			continue
		if answer.begins_with("gem="):
			var found := _find_bag_item(answer.trim_prefix("gem="))
			if not found.is_empty():
				online.net.insert_gem(found[0], found[1])
				await get_tree().create_timer(1.5).timeout
				var weapon = online.net.get_inventory()["equipped"][6]
				print("rose net demo: weapon now: ", weapon["tooltip"].replace("\n", " / ") if weapon else "none")
			continue
		if answer == "talk":
			online.talk_to(npc)
			await get_tree().create_timer(1.0).timeout
			_print_conversation()
			continue
		print("rose net demo: answer ", answer)
		online.conversation_window.choose(int(answer) - 1)
		await get_tree().create_timer(0.5).timeout
		_print_conversation()
	await get_tree().create_timer(2.0).timeout
	_print_quests()


## [page, index] of the first bag item whose name has this text, else [].
func _find_bag_item(text: String) -> Array:
	var pages: Array = online.net.get_inventory()["pages"]
	for page in pages.size():
		for i in pages[page].size():
			var item = pages[page][i]
			if item != null and text.to_lower() in String(item["name"]).to_lower():
				return [page, i]
	return []


func _print_conversation() -> void:
	var d: Dictionary = online.net.get_conversation()
	if not d.get("open", false):
		var open := " store open" if online.store_window.visible else (" bank open" if online.bank_window.visible else (" %s open" % online.work_window.mode if online.work_window.visible else ""))
		print("rose net demo: (no conversation)", open)
		return
	print("rose net demo: %s says: %s" % [d["title"], d["message"]])
	var responses: Array = d["responses"]
	for i in responses.size():
		print("rose net demo:   %d. %s" % [i + 1, responses[i]])


## Party test with two clients: with --invite=NAME, invite that player once they are near;
## without, accept the first invitation. Then both fight, printing the party now and then.
func _run_party_demo() -> void:
	var wanted := String(options.get("invite", ""))
	if not online.net.get_party().is_empty():
		# Start from scratch: leave the party of an earlier run.
		print("rose net demo: leave the old party")
		online.net.party_leave()
		await get_tree().create_timer(3.0).timeout
	var waited := 0.0
	while waited < 90.0 and not _party_has(wanted):
		await get_tree().create_timer(1.0).timeout
		waited += 1.0
		if wanted != "":
			for id in online.entities:
				if online.entities[id].label.text == wanted:
					print("rose net demo: invite ", wanted)
					online.net.party_invite(id)
					await get_tree().create_timer(4.0).timeout
					break
		elif not online.net.get_party_invites().is_empty():
			# --accept-after=SECONDS leaves the invitation up for a screenshot.
			await get_tree().create_timer(float(options.get("accept-after", "0"))).timeout
			print("rose net demo: accept invitation from ", online.net.get_party_invites()[0][1])
			online.party_window.answer_invite(true)
	_print_party()
	if options.has("rules"):
		var r := String(options["rules"]).split(",")
		online.net.party_set_rules(int(r[0]), int(r[1]))
	get_tree().create_timer(20.0).timeout.connect(_print_party)
	get_tree().create_timer(50.0).timeout.connect(_print_party)
	_run_fight_demo()


## Opens the first craft skill, picks the item matching --craft=NAME and crafts it
## --times=N times (default 3), a few seconds apart.
func _run_craft_demo() -> void:
	await get_tree().create_timer(4.0).timeout
	var pages: Array = online.net.get_skills()
	for page in pages.size():
		for index in pages[page].size():
			var skill = pages[page][index]
			if skill == null or skill["type"] != "Create Window":
				continue
			online.use_skill(page, index, skill)
			await get_tree().create_timer(1.0).timeout
			if not online.craft_window.visible:
				continue
			var wanted := String(options.get("craft", "")).to_lower()
			var w = online.craft_window
			print("rose net demo: %s makes %d items" % [skill["name"], w.entries.size()])
			for i in w.entries.size():
				if wanted in String(w.entries[i]["item"]["name"]).to_lower():
					w.list.select(i)
					w._select(i)
					break
			await get_tree().create_timer(1.5).timeout
			var e: Dictionary = w.entries[w.selected]
			print("rose net demo: craft %s from %s" % [e["item"]["name"], e["materials"].map(func(m): return "%s x%d" % [m["name"], m["quantity"]])])
			for n in int(options.get("times", "3")):
				print("rose net demo: materials ", w._slots)
				w.craft()
				await get_tree().create_timer(4.0).timeout
			return
	print("rose net demo: no craft skill")


## Trade test with two clients: with --trade-with=NAME ask that player, without accept the
## first request; then offer --offer=ITEM and --zuly=N, lock, and trade.
func _run_trade_demo() -> void:
	var wanted := String(options.get("trade-with", ""))
	if not online.net.get_trade().is_empty():
		online.net.trade_cancel()
		await get_tree().create_timer(2.0).timeout
	var waited := 0.0
	while waited < 90.0 and online.net.get_trade().is_empty():
		await get_tree().create_timer(1.0).timeout
		waited += 1.0
		if wanted != "":
			for id in online.entities:
				if online.entities[id].label.text == wanted:
					print("rose net demo: ask ", wanted, " to trade")
					online.net.trade_ask(id)
					await get_tree().create_timer(4.0).timeout
					break
		elif not online.net.get_trade_requests().is_empty():
			print("rose net demo: accept trade from ", online.net.get_trade_requests()[0][1])
			online.trade_window.answer_request(true)
	if online.net.get_trade().is_empty():
		print("rose net demo: no trade")
		return
	await get_tree().create_timer(2.0).timeout
	var offer := String(options.get("offer", ""))
	if offer != "":
		var found := _find_bag_item(offer)
		if not found.is_empty():
			var item = online.net.get_inventory()["pages"][found[0]][found[1]]
			online.trade_window.add(found[0], found[1], item.get("quantity", 1))
	await get_tree().create_timer(1.0).timeout
	if options.has("zuly"):
		online.trade_window.my_money.value = int(options["zuly"])
	await get_tree().create_timer(3.0).timeout
	_print_trade()
	online.net.trade_lock(true)
	waited = 0.0
	while waited < 30.0 and not online.net.get_trade().is_empty() and not online.net.get_trade()["theirs"]["locked"]:
		await get_tree().create_timer(1.0).timeout
		waited += 1.0
	_print_trade()
	await get_tree().create_timer(float(options.get("hold", "2"))).timeout
	var before: int = online.net.get_inventory()["money"]
	online.net.trade_accept()
	await get_tree().create_timer(4.0).timeout
	print("rose net demo: Zuly %d -> %d" % [before, online.net.get_inventory()["money"]])
	var names := []
	for page in online.net.get_inventory()["pages"]:
		for item in page:
			if item != null:
				names.append("%s x%d" % [item["name"], item.get("quantity", 1)])
	print("rose net demo: bag now ", names)


## PvP test: attack the player --fight=NAME (in a PvP zone), printing both HPs until one
## falls. --leave-party leaves our party a few seconds in (party members can't fight in
## "all except party" zones).
func _run_pvp_demo() -> void:
	var wanted := String(options.get("fight", ""))
	await get_tree().create_timer(4.0).timeout
	print("rose net demo: zone PvP state ", online.net.zone_pvp())
	var target := -1
	var waited := 0.0
	while waited < 60.0:
		for id in online.entities:
			if online.entities[id].label.text == wanted:
				target = id
		if target >= 0:
			break
		await get_tree().create_timer(1.0).timeout
		waited += 1.0
	if target < 0:
		print("rose net demo: no ", wanted)
		return
	print("rose net demo: %s is an enemy: %s" % [wanted, online.net.is_enemy_player(target)])
	online.attack(target)
	await get_tree().create_timer(2.0).timeout
	if options.has("leave-party") and not online.net.get_party().is_empty():
		print("rose net demo: leave the party")
		online.net.party_leave()
		await get_tree().create_timer(2.0).timeout
		print("rose net demo: %s is an enemy: %s" % [wanted, online.net.is_enemy_player(target)])
		online.attack(target)
	for i in 20:
		await get_tree().create_timer(1.5).timeout
		if online.my_target != target and online.net.is_enemy_player(target):
			online.attack(target)
		var them: Node3D = online.entities.get(target)
		print("rose net demo: me HP %d/%d, %s HP %s" % [online.me.hp, online.me.max_hp, wanted, "%d/%d" % [them.hp, them.max_hp] if them else "?"])
		if them == null or them.dead or online.me.dead:
			break


func _print_trade() -> void:
	var t: Dictionary = online.net.get_trade()
	if t.is_empty():
		print("rose net demo: not trading")
		return
	var side := func(s: Dictionary) -> String:
		return "%s + %d Zuly%s" % [s["items"].map(func(i): return "%s x%d" % [i["name"], i.get("quantity", 1)]), s["money"], " (locked)" if s["locked"] else ""]
	print("rose net demo: trade with %s: mine %s, theirs %s" % [t["with"], side.call(t["mine"]), side.call(t["theirs"])])


## In a party, with this member when a name is given.
func _party_has(member: String) -> bool:
	var party: Dictionary = online.net.get_party()
	if party.is_empty():
		return false
	if member == "":
		return true
	return party["members"].any(func(m): return m["name"] == member)


func _print_party() -> void:
	var party: Dictionary = online.net.get_party()
	if party.is_empty():
		print("rose net demo: no party")
		return
	var members := []
	for m in party["members"]:
		members.append("%s%s Lv %d HP %d/%d" % ["*" if m["leader"] else "", m["name"], m["level"], m["hp"], m["max_hp"]])
	print("rose net demo: party xp %d items %d: %s" % [party["xp_sharing"], party["item_sharing"], members])


func _print_bank() -> void:
	var items := []
	for item in online.net.get_bank():
		if item != null:
			items.append("%s x%d" % [item["name"], item.get("quantity", 1)])
	print("rose net demo: bank ", items)


func _print_quests() -> void:
	var quests: Array = online.net.get_quests()
	print("rose net demo: %d quests" % quests.size())
	for q in quests:
		var items: Array = q["items"].map(func(i): return "%s x%d" % [i["name"], i["quantity"]])
		print("rose net demo:   %s (slot %d, id %d) items %s" % [q["name"], q["slot"], q["id"], items])


## Walks to the nearest store, buys the first item of each tab, then sells one of them back.
func _run_shop_demo() -> void:
	await get_tree().create_timer(3.0).timeout
	var npc: int = online.nearest_store()
	if npc < 0:
		print("rose net demo: no store in this zone")
		return
	print("rose net demo: walk to %s, %.0f m away" % [online.entities[npc].label.text, online.entities[npc].position.distance_to(online.me.position)])
	online.talk_to(npc)
	var waited := 0.0
	while not online.store_window.visible and waited < 60.0:
		await get_tree().create_timer(0.5).timeout
		waited += 0.5
		# The NPC talks first: pick the answer that opens its store.
		if online.conversation_window.visible:
			var responses: Array = online.net.get_conversation().get("responses", [])
			for i in responses.size():
				var text := String(responses[i]).to_lower()
				if "trade" in text or "store" in text or "shop" in text or "buy" in text:
					print("rose net demo: answer ", responses[i])
					online.conversation_window.choose(i)
					break
	if not online.store_window.visible:
		print("rose net demo: the store did not open")
		return
	var store: Dictionary = online.store_window.store
	for t in store["tabs"]:
		var names := []
		for entry in t["items"]:
			names.append("%s %d" % [entry["item"]["name"], entry["price"]])
		print("rose net demo: %s, tab %s: %s" % [store["name"], t["name"], ", ".join(names)])
	var money_before: int = online.net.get_inventory()["money"]
	var wanted: String = options.get("buy", "")
	for t in store["tabs"].size():
		online.store_window.tabs.current_tab = t
		await get_tree().create_timer(1.0).timeout
		for entry in store["tabs"][t]["items"]:
			if wanted != "" and not wanted.to_lower() in String(entry["item"]["name"]).to_lower():
				continue
			print("rose net demo: buy %s for %d Zuly" % [entry["item"]["name"], entry["price"]])
			online.store_window.buy(entry["index"], 1)
			await get_tree().create_timer(1.0).timeout
			break
	print("rose net demo: Zuly %d -> %d" % [money_before, online.net.get_inventory()["money"]])
	# Read what we bought: skill books teach their skill, scrolls cast theirs.
	var use_page: Array = online.net.get_inventory()["pages"][1]
	for i in use_page.size():
		var item = use_page[i]
		if item != null and (item.get("class", "") == "Skill Book" or item.get("class", "") == "Magic Item"):
			print("rose net demo: use ", item["name"])
			online.net.use_item(1, i)
			await get_tree().create_timer(4.0).timeout
	await get_tree().create_timer(2.0).timeout
	var pages: Array = online.net.get_inventory()["pages"]
	for i in pages[0].size():
		if pages[0][i] != null and online.net.sell_price(0, i) >= 0:
			print("rose net demo: sell %s for %d Zuly" % [pages[0][i]["name"], online.net.sell_price(0, i)])
			online.store_window.sell(0, i, 1)
			break
	await get_tree().create_timer(2.0).timeout
	print("rose net demo: Zuly now %d" % online.net.get_inventory()["money"])


## Fights the nearest monster until it dies, then the next one. Every third fight starts
## with a swing cancelled by a step to the side, to show animation cancelling.
func _run_fight_demo() -> void:
	var fights := 0
	while true:
		await get_tree().create_timer(1.5).timeout
		var target: int = online.nearest_monster()
		if target < 0:
			continue
		print("rose net demo: attack ", online.entities[target].label.text, " (entity ", target, ")")
		online.attack(target)
		if fights % 3 == 0:
			# Wait for the first swing to start, then step away before it lands.
			var waited := 0.0
			while online.me and not online.me.was_swinging and waited < 6.0:
				await get_tree().process_frame
				waited += get_process_delta_time()
			await get_tree().create_timer(0.25).timeout
			print("rose net demo: cancel the swing by stepping aside")
			online.move_to(online.me.position + Vector3(1.0, 0, 0.5))
			await get_tree().create_timer(0.6).timeout
			online.attack(target)
		fights += 1
		while online.entities.has(target) and not online.entities[target].dying and online.me and not online.me.dead:
			await get_tree().create_timer(0.25).timeout
		# Pick up what it dropped (drops land a moment after the kill).
		await get_tree().create_timer(0.5).timeout
		var item: int = online.nearest_item()
		if item >= 0:
			print("rose net demo: pick up ", online.ground[item].get_node("Label").text)
			online.pickup(item)
			var waited := 0.0
			while online.ground.has(item) and waited < 5.0:
				await get_tree().create_timer(0.25).timeout
				waited += 0.25


## Where the mouse ray meets the ground: zone objects you can walk on (physics ray), or
## the terrain height field, whichever is nearer.
func _pick_ground(screen_position: Vector2):
	var origin := camera.project_ray_origin(screen_position)
	var direction := camera.project_ray_normal(screen_position)
	var terrain_hit = null
	var t := 0.0
	while t < 500.0:
		var p := origin + direction * t
		if p.y <= zone.get_terrain_height(p.x, p.z):
			terrain_hit = p
			break
		t += 0.25
	var query := PhysicsRayQueryParameters3D.create(origin, origin + direction * 500.0, 2)
	var hit := get_world_3d().direct_space_state.intersect_ray(query)
	if not hit.is_empty() and (terrain_hit == null or origin.distance_to(hit["position"]) < origin.distance_to(terrain_hit)):
		return hit["position"]
	return terrain_hit


func _say(text: String) -> void:
	print("rose demo: ", text)


## Run, swing, cancel the swing by moving, swing again and let it finish.
func _run_demo() -> void:
	var tree := get_tree()
	await tree.create_timer(1.0).timeout
	for name in ["attack", "attack2", "attack3"]:
		_say("%s: %d ms, hit at %d ms" % [name, player.anim.get_animation(name).length * 1000.0, player.hit_time(name) * 1000.0])
	_say("run")
	player.move_to(START + Vector3(8.0, 0.0, -6.0))
	await tree.create_timer(2.5).timeout
	_say("attack (full swing)")
	player.attack()
	await tree.create_timer(1.6).timeout
	_say("attack, cancelled by moving")
	player.attack()
	await tree.create_timer(0.35).timeout
	player.move_to(START + Vector3(2.0, 0.0, -10.0))
	await tree.create_timer(2.0).timeout
	_say("attack combo")
	player.attack()
	await tree.create_timer(1.3).timeout
	player.attack()
	await tree.create_timer(1.3).timeout
	player.attack()
	await tree.create_timer(1.6).timeout
	_say("done")
	tree.quit()
