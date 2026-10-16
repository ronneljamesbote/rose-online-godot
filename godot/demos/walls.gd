extends "res://demos/demo.gd"

## Runs 20 m (--wall-reach) out in eight (--wall-directions) directions from the start, coming back each time, to test
## collision with zone objects.
func run() -> void:
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
