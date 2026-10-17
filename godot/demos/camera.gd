extends "res://demos/demo.gd"

## Offline, --demo=camera: stands at --at=X,Z, or else next to the steepest slope near the
## zone's start, zooms all the way out and turns the camera round at its lowest angle.
## Prints where the camera is against the terrain and where a click on the screen lands,
## then clicks the ground three times and walks there. --shots=DIR saves a picture at every
## eighth of the turn.

const STEPS := 32


func run() -> void:
	var tree := get_tree()
	var camera = main.camera
	var zone = main.zone
	var spot := _below_cliff(zone, main.player.position)
	if options.has("at"):
		var v: PackedFloat64Array = str(options["at"]).split_floats(",")
		spot = Vector3(v[0], 0.0, v[1])
	main.player.place(spot)
	_say("standing at %.1f, %.1f, %.1f" % [spot.x, main.player.position.y, spot.z])
	camera.distance = camera.MAX_DISTANCE if "MAX_DISTANCE" in camera else 60.0  # the wheel's furthest
	camera.pitch = camera.MAX_PITCH if "MAX_PITCH" in camera else deg_to_rad(10.0)  # lowest the right button allows
	await tree.create_timer(1.5).timeout
	var under := 0
	var picks_under := 0
	var shots: String = options.get("shots", "")
	for i in STEPS:
		camera.yaw = TAU * i / STEPS
		await tree.process_frame
		await tree.process_frame
		var c: Vector3 = camera.global_position
		var ground: float = zone.get_terrain_height(c.x, c.z)
		var pick = main._pick_ground(main.get_viewport().get_visible_rect().size * Vector2(0.5, 0.7))
		var pick_text := "nothing"
		if pick != null:
			# A pick from below the terrain lands under the ground you see.
			var top: float = zone.get_terrain_height(pick.x, pick.z)
			pick_text = "%.1f, %.1f (%.1f m above the terrain)" % [pick.x, pick.z, pick.y - top]
			if pick.y < top - 0.5:
				picks_under += 1
		if c.y < ground:
			under += 1
		_say("turn %3d deg: camera %.1f m from the character, %.1f m above the terrain; click lands at %s" % [
			roundi(rad_to_deg(camera.yaw)), c.distance_to(main.player.position), c.y - ground, pick_text])
		if shots != "" and i % (STEPS / 8) == 0:
			await RenderingServer.frame_post_draw
			main.get_viewport().get_texture().get_image().save_png("%s/camera-%03d.png" % [shots, roundi(rad_to_deg(camera.yaw))])
		await tree.create_timer(0.3).timeout
	_say("camera under the terrain %d of %d turns, clicks landing under it %d" % [under, STEPS, picks_under])
	# Then click the ground the way a player does and walk there, three times.
	for at in [Vector2(0.7, 0.75), Vector2(0.3, 0.7), Vector2(0.5, 0.85)]:
		var screen: Vector2 = main.get_viewport().get_visible_rect().size * at
		var pick = main._pick_ground(screen)
		for pressed in [true, false]:
			var click := InputEventMouseButton.new()
			click.button_index = MOUSE_BUTTON_LEFT
			click.pressed = pressed
			click.position = screen
			Input.parse_input_event(click)
		_say("clicked the ground at %s" % (str(pick) if pick != null else "nothing"))
		await tree.create_timer(2.5).timeout
	_say("walked to %.1f, %.1f, %.1f" % [main.player.position.x, main.player.position.y, main.player.position.z])
	tree.quit()


## The point on the map within 60 m of around whose ground rises most steeply within 12 m.
func _below_cliff(zone: Node, around: Vector3) -> Vector3:
	var best := around
	var best_rise := -INF
	for gx in range(-60, 61, 4):
		for gz in range(-60, 61, 4):
			var p := around + Vector3(gx, 0.0, gz)
			var h: float = zone.get_terrain_height(p.x, p.z)
			if h == 0.0:
				continue  # off the map (no terrain there)
			for a in 8:
				var q := p + Vector3(cos(a * TAU / 8), 0.0, sin(a * TAU / 8)) * 12.0
				var rise: float = zone.get_terrain_height(q.x, q.z) - h
				if rise > best_rise:
					best_rise = rise
					best = p
	_say("steepest rise near the start: %.1f m within 12 m" % best_rise)
	return best


func _say(text: String) -> void:
	print("rose demo: ", text)
