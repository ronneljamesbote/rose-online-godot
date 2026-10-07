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
##   --name=NAME, --weapon=sword|bow   character for --server
##   --profile=NAME             identity file to use (user://identity-NAME.token), so two
##                              clients on one PC are two players
##   --offline                  skip the start screen and play offline
##   --net-demo[=square|line|fight|walls]   online: scripted routes, fights, wall tests
##   --net-log                  online: print every player's position once a second
##   --quit-after=SECONDS       quit after this long
extends Node3D

const DEFAULT_DATA_IDX := "data.idx"
const START := Vector3(5210.5, 0.0, -5136.7)  # zone start position from LIST_ZONE.STB
const SETTINGS := "user://settings.cfg"

var zone: RoseZone
var player: Node3D  # offline character
var online: Node3D  # online world, when connected
var camera_anchor: Node3D
var camera: Camera3D
var options := {}
var world_ticks := 0.0  # one world tick is 10 seconds


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
		DEFAULT_DATA_IDX,
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

	zone = RoseZone.new()
	zone.name = "Zone"
	add_child(zone)
	if not zone.load_zone(int(options.get("zone", "1"))):
		get_tree().quit(1)
		return
	print("rose: zone loaded ", zone.get_stats(), " day cycle ", zone.get_day_cycle(), " morning/day/evening/night start ", [zone.get_state_start("morning"), zone.get_state_start("day"), zone.get_state_start("evening"), zone.get_state_start("night")])
	var time: String = options.get("time", "day")
	world_ticks = float(time) if time.is_valid_int() else float(zone.get_state_start(time)) + 1.0
	_apply_lighting(zone.get_lighting_at(int(world_ticks), 0.0))
	print("rose: lighting ", zone.get_lighting_at(int(world_ticks), 0.0))

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
	online.log_positions = options.has("net-log")
	online.log_damage = options.has("net-log") or options.has("net-demo")
	online.start(zone, uri, token_path, name_text, use_bow)


func _on_joined(me: Node3D) -> void:
	if camera.get("target") != null:
		camera.target = me
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
	if event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_LEFT:
		if online:
			var monster: int = online.pick_monster(camera, event.position)
			if monster >= 0:
				online.attack(monster)
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
