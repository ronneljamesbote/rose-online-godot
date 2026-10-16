extends "res://demos/demo.gd"

## Walks a route from where the character joined, forever: "square" (default) goes round
## the start point 4 m out, "line" goes back and forth 5 m east.
func run() -> void:
	var route := [Vector3(4, 0, 2), Vector3(4, 0, -6), Vector3(-4, 0, -6), Vector3(-4, 0, 2)]
	var pause := 2.6
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
