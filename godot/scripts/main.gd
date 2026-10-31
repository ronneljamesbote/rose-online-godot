## Proof of concept: loads Canyon City of Zant from the original data files and puts an
## animated character on the terrain.
##
## Command-line options (after "--"):
##   --data-idx=PATH            ROSE data.idx (or set ROSE_DATA_IDX; otherwise the last one
##                              picked, a ROSE folder next to the game, or ask)
##   --zone=ID                  zone to load (default 1, Canyon City of Zant)
##   --zone-files               build zones from the ROSE map files even when they have scenes
##                              in res://zones/ (to compare the two)
##   --free-camera=x,y,z,yaw,pitch   fixed camera, degrees, same convention as the Bevy zone viewer
##   --screenshot=PATH          save one frame and quit
##   --time=morning|day|evening|night|TICKS   fixed time of day (default: day, then the clock runs)
##   --server=URI               play online on this SpacetimeDB server, skipping the start screen
##   --name=NAME                character name for --server (servers without accounts)
##   --email=EMAIL --password=PASSWORD   sign in to this account (or set ROSE_PASSWORD)
##   --website=URL              the account website (default http://127.0.0.1:3001)
##   --create=NAME,female,FACE,HAIR   fill in the character creation screen (1-based),
##                              --create-after=SECONDS then presses Create
##   --weapon=bow               equip the bow and arrows from the bag after signing in
##   --profile=NAME             identity file to use (user://identity-NAME-HOST-PORT.token), so two
##                              clients on one PC are two players
##   --offline                  skip the start screen and play offline
##   --net-demo[=NAME], --demo, --effect=PATH|ID,...   scripted runs for tests and recordings,
##                              see demos/README.md (not in the Windows build)
##   --net-log                  online: print every player's position once a second
##   --perf-log                 print how long each zone load took, part by part; online also the
##                              frame rate, entities drawn and frame times every 2 seconds
##   --open=inventory,character,skills,quests   online: open these windows at the start (for screenshots)
##   --mute                     no music or sound (or --music=0..1 --effects=0..1 for this run;
##                              the O key opens the sound window, which saves them)
##   --sound-log                print each sound effect played
##   --quit-after=SECONDS       quit after this long
##   --import-rose[=force]      convert the ROSE models, textures, sounds and effects into res://assets/rose/
##                              and quit (run headless; force redoes files already there)
##   --import-zone=ID[,force]   write the scenes of zone ID into res://zones/ from the ROSE map
##                              files and quit (run headless; force replaces scenes already there)
##   --convert-data[=force]     write the data resources in res://data/ from the iROSE tables
##                              and quit (run once, headless; they are the source after that, so it
##                              skips each kind already there; force writes them all again)
##   --export-data[=check]      write ../export/ for the server and the game and quit: the game
##                              data from the resources in res://data/ (export/data/), the spawns,
##                              NPCs and named spots of the zone scenes in res://zones/
##                              (export/zones/) and the hit times from the hit markers of the
##                              imported animations (export/motions.toml); check only compares and
##                              fails when a file is out of date. Run headless, after --import-rose
##   --merge-zone-blocks=ID|all  move a zone's block_X_Y.tscn files into its zone.tscn (zones
##                              converted before 2026-10-18) and quit (run headless, after --import-rose)
##   --strip-lightmaps=ID|all   take the baked lightmaps out of a zone scene made before real-time
##                              shadows (painting their colour into the ground's colour map and the models' part_tints) and quit
##                              (run headless)
##   --paint-ground-tint=ID|all  paint the colour the client's ground lightmaps gave a zone scene
##                              into its ground's colour map again (replaces colour painted by
##                              hand) and quit (run headless)
##   --paint-object-tints=ID|all  write the colour each placed building and decoration had from its
##                              client lightmap into its part_tints and quit (run headless, after
##                              --import-rose)
##   --shadows=off|low|medium|high   shadow setting for this run (Options > Graphics saves one)
##   --frame-time               with --screenshot: print the average frame time and draw calls
extends Node3D

const START := Vector3(5210.5, 0.0, -5136.7)  # the zone's start position
const STEER_EVERY_MS := 200  # while the left button is held, look for a new spot this often
const STEER_MIN_MOVE := 0.5  # metres the spot under the cursor must move to send a new one
const SETTINGS := "user://settings.cfg"

var zone: RoseZone
var player: Node3D  # offline character
var online: Node3D  # online world, when connected
var camera_anchor: Node3D
var camera: Camera3D
var options := {}
var world_ticks := 0.0  # one world tick is 10 seconds
var loaded_zone := 0
var loading: CanvasLayer  # loading_screen.gd
var fx: Node  # fx.gd: music, sounds and effects
var sun: RoseSun  # the sun (or moon), which casts the shadows
var _loading_zone := 0
var zone_preload: Node  # zone_preload.gd: reads the zone behind a nearby warp gate ahead


var _joined_before := false
var _steering := false  # the left button went down on the ground and is still held
var _steer_last := Vector3.ZERO  # where the held button last sent us
var _steer_next_ms := 0

func _ready() -> void:
	for arg in OS.get_cmdline_user_args():
		var parts := arg.trim_prefix("--").split("=", true, 1)
		options[parts[0]] = parts[1] if parts.size() > 1 else "true"
	# --theme=clear, --ui-scale=1.25 and --ui-opacity=0.5 set the interface options (for screenshots).
	if options.has("theme"):
		UI.set_setting("theme", options["theme"])
	if options.has("ui-scale"):
		UI.set_setting("scale", float(options["ui-scale"]))
	if options.has("ui-opacity"):
		UI.set_setting("opacity", float(options["ui-opacity"]))

	if options.has("merge-zone-blocks"):
		_merge_zone_blocks(options["merge-zone-blocks"])
		return
	for old in ["export-motions", "export-zones"]:
		if options.has(old):
			push_error("rose: --%s is part of --export-data now (it writes the zones and motions too)" % old)
			get_tree().quit(1)
			return
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


## The ROSE client could not be read (usually a file missing from an incomplete unzip):
## say which file, then let the player pick another data.idx or quit.
func _show_data_error(data_idx: String, error: String) -> void:
	var dialog := ConfirmationDialog.new()
	dialog.title = "ROSE client files not found"
	dialog.dialog_text = ("Could not load the ROSE client at\n%s\n\n%s\n\n"
		+ "Unpack the full iROSE 129_129 client again (all .VFS files next to data.idx),\n"
		+ "or pick a different data.idx.") % [data_idx, error]
	dialog.ok_button_text = "Pick data.idx"
	dialog.cancel_button_text = "Quit"
	dialog.confirmed.connect(func():
		dialog.queue_free()
		_ask_for_data_idx())
	dialog.canceled.connect(func(): get_tree().quit(1))
	add_child(dialog)
	dialog.popup_centered()


## The import command: writes the converted assets, prints what it made and quits.
func _import_rose(force: bool) -> void:
	var result: Dictionary = RoseImport.run(force)
	if result.is_empty():
		get_tree().quit(1)
		return
	print("rose: imported %d meshes, %d textures, %d sounds and %d effects and particle systems (%d already there, %d failed) in %.0f s" % [
		result.meshes, result.textures, result.sounds, result.effects, result.kept, result.failed, result.seconds])
	if result.empty_effects > 0:
		print("rose: %d effect files show nothing in the client data either; left out" % result.empty_effects)
	var folders: Dictionary = result.folders
	for folder in folders:
		print("rose:   %-24s %8.1f MB" % [folder, folders[folder] / 1048576.0])
	print("rose: characters, monsters, NPCs and carts: %d files made, %d already there, %.1f MB" % [
		result.avatar_files, result.avatar_kept, result.avatar_bytes / 1048576.0])
	print("rose: zone models and textures: %d files made, %d already there, %.1f MB" % [
		result.zone_files, result.zone_kept, result.zone_bytes / 1048576.0])
	print("rose: assets/rose is %.1f MB" % ((result.bytes + result.avatar_bytes + result.zone_bytes) / 1048576.0))
	get_tree().quit(1 if result.failed > 0 else 0)


## Writes a zone's scenes, prints what it made and quits.
func _import_zone(zone_id: int, force: bool) -> void:
	await get_tree().process_frame  # the import adds a Terrain3D node to the tree for a moment
	var result: Dictionary = RoseImport.import_zone(zone_id, force)
	if result.has("error"):
		push_error("rose: %s" % result.error)
		get_tree().quit(1)
		return
	print("rose: %s: %d blocks, %d objects, %d markers" % [result.folder, result.blocks, result.objects, result.markers])
	get_tree().quit(0)


## Moves the block scenes of one zone ("all": every zone) into its zone.tscn and quits.
func _merge_zone_blocks(which: String) -> void:
	var result: Dictionary = RoseImport.merge_zone_blocks(0 if which == "all" else int(which))
	if result.has("error"):
		push_error("rose: %s" % result.error)
		get_tree().quit(1)
		return
	for zone in result.zones:
		if zone.blocks > 0:
			print("rose: %s: %d blocks moved into zone.tscn" % [zone.folder, zone.blocks])
		else:
			print("rose: %s: already one scene" % zone.folder)
	get_tree().quit(0)


## Takes the baked lightmaps out of one zone scene ("all": every zone) and quits.
func _strip_lightmaps(which: String) -> void:
	await get_tree().process_frame  # it adds a Terrain3D node to the tree for a moment
	var result: Dictionary = RoseImport.strip_lightmaps(0 if which == "all" else int(which))
	if result.has("error"):
		push_error("rose: %s" % result.error)
		get_tree().quit(1)
		return
	for zone in result.zones:
		if zone.lines > 0:
			print("rose: %s: %d lightmap lines taken out, ground colour painted for %d blocks" % [zone.folder, zone.lines, zone.blocks])
		else:
			print("rose: %s: no lightmaps (already done)" % zone.folder)
	get_tree().quit(0)


## Paints the lightmap colours of one zone scene ("all": every zone) and quits: the ground's
## ("ground") or the placed models' ("objects").
func _paint(what: String, which: String) -> void:
	await get_tree().process_frame  # the ground adds a Terrain3D node to the tree for a moment
	var id := 0 if which == "all" else int(which)
	var result: Dictionary = RoseImport.paint_ground_tint(id) if what == "ground" else RoseImport.paint_object_tints(id)
	if result.has("error"):
		push_error("rose: %s" % result.error)
		get_tree().quit(1)
		return
	for zone in result.zones:
		print("rose: %s: %s coloured" % [zone.folder, ("%d blocks" if what == "ground" else "%d models") % zone.count])
	get_tree().quit(0)


## Writes the data resources (items, monsters, NPCs ...) from the iROSE tables, prints how
## many and quits.
func _convert_data(force: bool) -> void:
	var result: Dictionary = RoseDataExport.convert(force)
	if result.has("error"):
		push_error("rose: %s" % result.error)
		get_tree().quit(1)
		return
	for kind in result.skipped:
		print("rose: %s already made in Godot, left as they are (--convert-data=force writes them again)" % kind)
	for line in result.lost:
		print("rose:   not kept: %s" % line)
	print("rose: %d resources written to %s" % [result.resources, result.folder])
	get_tree().quit(0)


## Writes or checks export/ from the resources, the zone scenes and the animations, prints
## what differs and quits (1 when a check fails).
func _export_data(check: bool) -> void:
	var result: Dictionary = RoseDataExport.export(check)
	if result.has("error"):
		push_error("rose: %s" % result.error)
		get_tree().quit(1)
		return
	print("rose: %d items, %d monsters, %d NPCs, %d skills, %d zones, %d sounds, %d effects, %d attack effects and %d skies from %d resources (read in %d ms)" % [result.items, result.monsters, result.npcs, result.skills, result.zones, result.sounds, result.effects, result.attack_effects, result.skies, result.resources, result.read_ms])
	print("rose: %d quests, %d quest triggers" % [result.quests, result.quest_triggers])
	print("rose: %d NPC dialogs, %d monster AIs" % [result.dialogs, result.ai])
	print("rose: %d zone scenes, %d motions" % [result.zone_scenes, result.motions])
	var changed: Array = result.changed
	for path in changed:
		print("rose:   %s %s" % ["differs:" if check else "updated:", path])
	if check:
		print("rose: export %s" % ("matches the resources, zone scenes and animations" if changed.is_empty() else "out of date; run --export-data"))
	else:
		print("rose: %d files in the export, %d changed" % [result.files.size(), changed.size()])
	get_tree().quit(1 if check and not changed.is_empty() else 0)


func _start(data_idx: String) -> void:
	var started := Time.get_ticks_msec()
	if not RoseData.open(data_idx):
		_show_data_error(data_idx, RoseData.open_error())
		return
	print("rose: data tables loaded in %d ms" % (Time.get_ticks_msec() - started))
	if options.has("import-rose"):
		_import_rose(options["import-rose"] == "force")
		return
	if options.has("export-data"):
		_export_data(options["export-data"] == "check")
		return
	if options.has("convert-data"):
		_convert_data(options["convert-data"] == "force")
		return
	if options.has("import-zone"):
		var parts: PackedStringArray = str(options["import-zone"]).split(",")
		_import_zone(int(parts[0]), parts.size() > 1 and parts[1] == "force")
		return
	if options.has("strip-lightmaps"):
		_strip_lightmaps(options["strip-lightmaps"])
		return
	if options.has("paint-ground-tint"):
		_paint("ground", options["paint-ground-tint"])
		return
	if options.has("paint-object-tints"):
		_paint("objects", options["paint-object-tints"])
		return

	fx = preload("res://scripts/fx.gd").new()
	fx.name = "Fx"
	add_child(fx)
	fx.log_sounds = options.has("sound-log")
	if options.has("mute"):
		fx.set_volumes(0.0, 0.0, false)
	elif options.has("music") or options.has("effects"):
		fx.set_volumes(float(options.get("music", fx.music_volume)), float(options.get("effects", fx.effects_volume)), false)
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
		camera.zone = zone
	camera.fov = float(options.get("fov", "45"))
	camera.near = 0.1
	camera.far = 1000.0
	add_child(camera)
	camera.make_current()
	fx.camera = camera

	sun = RoseSun.new()
	sun.name = "Sun"
	add_child(sun)
	_apply_shadow_setting()
	UI.settings_changed.connect(_apply_shadow_setting)
	if zone:
		sun.apply(zone.get_lighting_at(int(world_ticks), 0.0))

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
	if options.has("server") and options.has("email"):
		_sign_in_from_options()
	elif options.has("server"):
		_go_online(options["server"], options.get("name", ""), options.get("weapon", "sword") == "bow")
	elif options.has("offline") or options.has("demo") or options.has("screenshot") or options.has("free-camera"):
		_go_offline()
	else:
		_show_connect_panel("")

	if options.has("demo"):
		_start_demo("demo")
	if options.has("effect"):
		_start_demo("effect")
	if options.has("screenshot"):
		_take_screenshot(options["screenshot"])


## Milliseconds of zone building per frame while the loading screen shows.
const LOAD_STEP_MS := 8.0

## Loads a zone in place of the current one in one go (the first zone, screenshots).
func _load_zone(zone_id: int) -> bool:
	if zone:
		zone.queue_free()
		remove_child(zone)
	zone = RoseZone.new()
	zone.name = "Zone"
	zone.files_only = options.has("zone-files")
	add_child(zone)
	move_child(zone, 0)
	if not zone.load_zone(zone_id):
		return false
	_use_zone(zone_id, 0.0)
	return true


## Loads a zone without stopping the game, so the loading screen keeps moving: its files are
## read on background threads and its nodes made a few milliseconds a frame, beside the old
## zone, which goes once the new one is ready. Effects near `focus` (our character's spot
## in the new zone, when known) are made first.
func _load_zone_in_steps(zone_id: int, focus: Dictionary) -> bool:
	var next := RoseZone.new()
	next.name = "NextZone"
	next.files_only = options.has("zone-files")
	add_child(next)
	move_child(next, 0)
	if focus.has("x"):
		next.set_focus(focus["x"], focus["z"])
	if not next.begin_load(zone_id):
		next.queue_free()
		return false
	# The loading screen covers the world, so the world isn't drawn meanwhile: the frames
	# stay short and the background threads get the time.
	var covered: bool = loading != null and loading.is_showing()
	get_viewport().disable_3d = covered
	var longest_frame := 0.0
	var last := Time.get_ticks_usec()
	while true:
		var progress := next.step_load(LOAD_STEP_MS)
		if loading:
			loading.set_progress(progress)
		if progress >= 1.0:
			break
		await get_tree().process_frame
		var now := Time.get_ticks_usec()
		longest_frame = maxf(longest_frame, (now - last) / 1000.0)
		last = now
	get_viewport().disable_3d = false
	if next.get_stats().is_empty():
		next.queue_free()
		return false
	if zone:
		zone.queue_free()
		remove_child(zone)
	zone = next
	zone.name = "Zone"
	zone.activate()
	_use_zone(zone_id, longest_frame)
	return true


## After a zone loaded: the camera, version line, lighting and sounds follow it.
## `longest_frame` is the longest frame while it loaded in steps (ms, 0 for one go).
func _use_zone(zone_id: int, longest_frame: float) -> void:
	loaded_zone = zone_id
	if zone_preload:
		zone_preload.let_go()
	if camera and "zone" in camera:
		camera.zone = zone
	zone.set_meta("zone_id", zone_id)
	VersionLine.zone = VersionLine.zone_line(zone_id, RoseData.zone_name(zone_id), zone.get_stats())
	if options.has("perf-log"):
		print(_load_times(zone_id, zone.get_stats(), longest_frame))
	print("rose: zone ", zone_id, " loaded ", zone.get_stats(), " day cycle ", zone.get_day_cycle(), " morning/day/evening/night start ", [zone.get_state_start("morning"), zone.get_state_start("day"), zone.get_state_start("evening"), zone.get_state_start("night")])
	if world_ticks == 0.0:
		var time: String = options.get("time", "day")
		world_ticks = float(time) if time.is_valid_int() else float(zone.get_state_start(time)) + 1.0
	var lighting := zone.get_lighting_at(int(world_ticks), 0.0)
	_apply_lighting(lighting)
	fx.use_zone(zone, zone_id)
	fx.set_time_state(lighting.get("state", "day"))


## The --perf-log line of a zone load: how long each part took.
func _load_times(zone_id: int, stats: Dictionary, longest_frame: float) -> String:
	var line := "rose perf: zone %d ready in %d ms: files read %d ms (background), instancing %.1f ms, adding models %.1f ms" % [
		zone_id, stats.get("load_ms", 0), stats.get("read_ms", 0), stats.get("instance_ms", 0.0), stats.get("attach_ms", 0.0)]
	line += ", reading nodes %.1f ms, collision %.1f ms (%s), sky %.1f ms, effects %.1f ms (%d of %d after the zone shows)" % [
		stats.get("gather_ms", 0.0), stats.get("collision_ms", 0.0), stats.get("collision", ""), stats.get("sky_ms", 0.0),
		stats.get("effects_ms", 0.0), stats.get("effects_later", 0), stats.get("effects", 0)]
	line += "; %d steps, longest step %.1f ms" % [stats.get("steps", 0), stats.get("longest_step_ms", 0.0)]
	if longest_frame > 0.0:
		line += ", longest frame %.1f ms" % longest_frame
	line += "; %d model parts fade at %d m" % [stats.get("faded_parts", 0), stats.get("fade_distance", 0)]
	return line


## The server has our character in another zone: load it behind the loading screen and
## show that zone's entities. The screen goes once our character stands in the new zone.
func _on_zone_needed(zone_id: int) -> void:
	# One load at a time: when the server moved us on meanwhile, online.gd asks again after.
	if zone_id == loaded_zone or _loading_zone != 0:
		return
	_loading_zone = zone_id
	if loading == null:
		loading = preload("res://scripts/loading_screen.gd").new()
		add_child(loading)
	loading.show_zone(RoseData.zone_name(zone_id))
	await get_tree().process_frame
	if await _load_zone_in_steps(zone_id, online.net.get_character()):
		online.use_zone(zone)
	_loading_zone = 0
	# In case our character never shows up (the server moved us again).
	get_tree().create_timer(10.0).timeout.connect(func(): if loading: loading.hide_screen())


func _show_connect_panel(error: String) -> CanvasLayer:
	var panel = preload("res://scripts/connect_panel.gd").new()
	panel.connect_requested.connect(_go_online)
	panel.offline_requested.connect(_go_offline)
	add_child(panel)
	if error != "":
		panel.show_error(error)
	return panel


## --server with --email and --password (or ROSE_PASSWORD): sign in on --website first.
func _sign_in_from_options() -> void:
	var login = preload("res://scripts/account_login.gd").new()
	add_child(login)
	var password: String = options.get("password", OS.get_environment("ROSE_PASSWORD"))
	var answer: Dictionary = await login.login(options.get("website", "http://127.0.0.1:3001"), options["email"], password)
	login.queue_free()
	if answer.has("error"):
		print("rose net: signing in failed: ", answer["error"])
		_show_connect_panel(answer["error"]).email_edit.text = options["email"]
		return
	print("rose net: signed in on the account website")
	_go_online(options["server"], "", options.get("weapon", "sword") == "bow", answer["token"])


## The server turned us away (no account, say) or the connection dropped: back to the start.
func _on_connection_failed(reason: String) -> void:
	print("rose net: connection failed: ", reason)
	if options.has("server") and not options.has("email"):
		return
	_leave_online()
	_show_connect_panel("The server closed the connection: %s" % reason)


## Our logout finished: close the connection and go back to the start screen.
func _on_logged_out() -> void:
	online.net.disconnect_from()
	_leave_online()
	_show_connect_panel("")


## Drops the online game: the camera stays where our character was.
func _leave_online() -> void:
	if loading:
		loading.hide_screen()
	if camera and camera.get("target") != null and camera.target != camera_anchor:
		if is_instance_valid(camera.target):
			camera_anchor.global_position = camera.target.global_position
		camera.target = camera_anchor
	fx.online = null
	online.queue_free()
	online = null
	VersionLine.net = null
	_joined_before = false


func _go_offline() -> void:
	player = preload("res://scripts/player.gd").new()
	player.name = "Player"
	add_child(player)
	player.setup(zone, true, {"face": 1, "hair": 0, "body": 1, "hands": 1, "feet": 1, "weapon": 2})
	player.place(START)
	if camera.get("target") != null:
		camera.target = player


func _go_online(uri: String, name_text: String, use_bow: bool, token := "") -> void:
	var profile: String = options.get("profile", "default")
	# Each server signs its own tokens, so a token is kept per server address.
	var server_key := uri.get_slice("://", 1).trim_suffix("/").replace(":", "-").replace("/", "-")
	var token_path := ProjectSettings.globalize_path("user://identity-%s-%s.token" % [profile, server_key])
	online = preload("res://scripts/online.gd").new()
	online.name = "Online"
	add_child(online)
	fx.online = online
	online.joined.connect(_on_joined)
	online.zone_needed.connect(_on_zone_needed)
	if zone_preload == null:
		zone_preload = preload("res://scripts/zone_preload.gd").new()
		zone_preload.name = "ZonePreload"
		add_child(zone_preload)
	online.warp_gate_near.connect(zone_preload.want)
	online.connection_failed.connect(_on_connection_failed)
	online.logged_out.connect(_on_logged_out)
	online.create_preset = options.get("create", "")
	online.create_after = float(options.get("create-after", "-1"))
	online.log_positions = options.has("net-log")
	online.log_perf = options.has("perf-log")
	online.log_damage = options.has("net-log") or options.has("net-demo")
	online.start(zone, uri, token_path, name_text, use_bow, token)
	VersionLine.net = online.net


func _on_joined(me: Node3D) -> void:
	if loading:
		loading.hide_screen()
	if camera.get("target") != null:
		camera.target = me
	# Joining again after a warp keeps the windows and demo of the first join.
	if _joined_before:
		return
	_joined_before = true
	online.ui_sample = options.has("ui-sample")
	# --ui-screen=death|create|login|loading shows that screen over the game (screenshots).
	online.ui_sample_screen = options.get("ui-screen", "")
	match online.ui_sample_screen:
		"create":
			var create = preload("res://scripts/character_create.gd").new()
			add_child(create)
		"login":
			_show_connect_panel("")
		"loading":
			if loading:
				loading.show_zone("Junon Polis")
	var windows: String = options.get("open", "")
	for which in ["inventory", "character", "skill", "quest", "party", "friends", "map", "options"]:
		if which in windows:
			online.get(which + "_window").visible = true
	if options.has("net-demo"):
		_start_demo("net-demo", me)


## Scripted runs for tests and recordings (--net-demo, --demo, --effect) live in
## res://demos/; the Windows build leaves them out.
func _start_demo(which: String, me: Node3D = null) -> void:
	if not ResourceLoader.exists("res://demos/runner.gd"):
		push_warning("rose: demos aren't in this build")
		return
	load("res://demos/runner.gd").start(self, which, me)


func _apply_lighting(lighting: Dictionary) -> void:
	if sun:
		sun.apply(lighting)


## Options > Graphics > Shadows (--shadows for one run).
func _apply_shadow_setting() -> void:
	if sun:
		sun.set_quality(options.get("shadows", UI.settings.get("shadows", "medium")))


func _process(delta: float) -> void:
	if zone == null:
		return
	# Only models near the player cast shadows; this does nothing until they change block.
	var target = camera.get("target") if camera else null
	if target is Node3D:
		zone.update_shadows(target.global_position)
	elif camera:
		zone.update_shadows(camera.global_position)
	if options.has("screenshot"):
		return
	_steer()
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
	var lighting := zone.get_lighting_at(int(world_ticks), fmod(world_ticks, 1.0))
	_apply_lighting(lighting)
	fx.set_time_state(lighting.get("state", "day"))


func _take_screenshot(path: String) -> void:
	if options.has("screenshot-after"):
		await get_tree().create_timer(float(options["screenshot-after"])).timeout
	for i in 10:
		await get_tree().process_frame
	await RenderingServer.frame_post_draw
	get_viewport().get_texture().get_image().save_png(path)
	print("rose: saved ", path, " (draw calls %d, objects %d, primitives %d)" % [
		Performance.get_monitor(Performance.RENDER_TOTAL_DRAW_CALLS_IN_FRAME),
		Performance.get_monitor(Performance.RENDER_TOTAL_OBJECTS_IN_FRAME),
		Performance.get_monitor(Performance.RENDER_TOTAL_PRIMITIVES_IN_FRAME)])
	if options.has("frame-time"):
		# --frame-time: the average frame over the next 30, and the shadow pass's share.
		var viewport := get_viewport()
		var started := Time.get_ticks_usec()
		for i in 30:
			await RenderingServer.frame_post_draw
		print("rose: frame %.1f ms, draw calls %d, shadow draw calls %d" % [
			(Time.get_ticks_usec() - started) / 30000.0,
			viewport.get_render_info(Viewport.RENDER_INFO_TYPE_VISIBLE, Viewport.RENDER_INFO_DRAW_CALLS_IN_FRAME),
			viewport.get_render_info(Viewport.RENDER_INFO_TYPE_SHADOW, Viewport.RENDER_INFO_DRAW_CALLS_IN_FRAME)])
	get_tree().quit()


func _unhandled_input(event: InputEvent) -> void:
	if online and event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_RIGHT:
		var other: int = online.pick_player(camera, event.position)
		if other >= 0:
			online.open_player_menu(other, event.position)
			return
	if event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_LEFT:
		if online:
			var under: Dictionary = online.under_mouse(camera, event.position)
			match under.get("kind", ""):
				"monster", "enemy":
					online.attack(under["id"])
					return
				"npc":
					online.talk_to(under["id"])
					return
				"player":
					online.select(under["id"])
					return
				"item":
					online.pickup(under["id"])
					return
		var hit = _pick_ground(event.position)
		if hit != null:
			if online:
				online.click_ground(hit)
				_steering = true
				_steer_last = hit
				_steer_next_ms = Time.get_ticks_msec() + STEER_EVERY_MS
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
		elif event.keycode == KEY_F and online:
			online.toggle_friends_window()
		elif event.keycode == KEY_P and online:
			online.toggle_party_window()
		elif event.keycode == KEY_M and online:
			online.toggle_map_window()
		elif event.keycode == KEY_ESCAPE and online:
			if not online.close_top_window():
				online.clear_target()
		elif event.keycode >= KEY_1 and event.keycode <= KEY_8 and online:
			online.hotbar.use_slot(event.keycode - KEY_1)
		elif event.keycode == KEY_Z and online:
			online.pickup(online.nearest_item())
		elif event.keycode == KEY_X and online:
			online.sit()
		elif event.keycode == KEY_O and online:
			online.toggle_options_window()
		elif event.keycode == KEY_O:
			_toggle_sound_window()
		elif (event.keycode == KEY_ENTER or event.keycode == KEY_KP_ENTER) and online:
			online.chat_window.focus_input()


var _sound_window: PanelContainer


func _toggle_sound_window() -> void:
	if _sound_window == null:
		var layer := CanvasLayer.new()
		layer.layer = 5
		add_child(layer)
		_sound_window = preload("res://scripts/sound_window.gd").new()
		_sound_window.fx = fx
		_sound_window.position = Vector2(12, 300)
		layer.add_child(_sound_window)
		return
	_sound_window.visible = not _sound_window.visible


## Holding the left button after clicking the ground keeps running toward the ground
## under the cursor, as in iROSE. Holding only steers: sweeping over a monster, NPC or
## item does not act on it. Letting go drops the marker where we will stop.
func _steer() -> void:
	if not _steering:
		return
	if online == null or not Input.is_mouse_button_pressed(MOUSE_BUTTON_LEFT):
		_steering = false
		if online:
			online.end_steer()
		return
	if Time.get_ticks_msec() < _steer_next_ms:
		return
	_steer_next_ms = Time.get_ticks_msec() + STEER_EVERY_MS
	var hit = _pick_ground(get_viewport().get_mouse_position())
	if hit == null or Vector2(hit.x - _steer_last.x, hit.z - _steer_last.z).length() < STEER_MIN_MOVE:
		return
	if online.steer_to(hit):
		_steer_last = hit


## Where the mouse ray meets the ground: zone objects you can walk on (physics ray), or
## the terrain height field, whichever is nearer. The terrain counts only where the ray
## comes down onto it from above, never from underneath.
func _pick_ground(screen_position: Vector2):
	var origin := camera.project_ray_origin(screen_position)
	var direction := camera.project_ray_normal(screen_position)
	var terrain_hit = null
	var t := 0.0
	var was_above := false
	while t < 500.0:
		var p := origin + direction * t
		if p.y > zone.get_terrain_height(p.x, p.z):
			was_above = true
		elif was_above:
			# Narrow down between the last step above and this one.
			var above := t - 0.25
			for i in 8:
				var mid := (above + t) * 0.5
				var m := origin + direction * mid
				if m.y > zone.get_terrain_height(m.x, m.z):
					above = mid
				else:
					t = mid
			terrain_hit = origin + direction * t
			break
		t += 0.25
	var query := PhysicsRayQueryParameters3D.create(origin, origin + direction * 500.0, 2)
	var hit := get_world_3d().direct_space_state.intersect_ray(query)
	if not hit.is_empty() and (terrain_hit == null or origin.distance_to(hit["position"]) < origin.distance_to(terrain_hit)):
		return hit["position"]
	return terrain_hit
