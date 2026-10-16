extends "res://demos/demo.gd"

## Plays EFT effects in a row 10 m in front of the camera, each again once it ends.
func run() -> void:
	var paths: PackedStringArray = String(options["effect"]).split(",")
	for i in 3:
		await get_tree().process_frame
	var forward := -main.camera.global_basis.z
	forward.y = 0.0
	forward = forward.normalized()
	var side := forward.cross(Vector3.UP)
	var centre := main.camera.global_position + forward * 10.0
	var target = main.camera.get("target")
	if target is Node3D:
		centre = target.global_position + forward * 3.0
	var spots := []
	for i in paths.size():
		var at := centre + side * (i - (paths.size() - 1) / 2.0) * 3.0
		at.y = main.zone.get_terrain_height(at.x, at.z)
		spots.append(at)
	var playing := {}
	while true:
		for i in paths.size():
			if not is_instance_valid(playing.get(i)):
				playing[i] = main.fx.spawn_effect(paths[i], spots[i])
				if playing[i] == null:
					push_warning("rose: effect %s shows nothing" % paths[i])
		await get_tree().create_timer(0.5).timeout
