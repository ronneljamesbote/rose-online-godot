extends "res://demos/demo.gd"

## Walks into the nearest warp gate, then (in the zone it leads to) into that zone's
## nearest one, three times.
func run() -> void:
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
		print("rose net demo: zone %d, walk to the warp gate at (%.1f, %.1f), %.0f m away" % [main.loaded_zone, best.x, best.z, best.distance_to(me.position)])
		var zone_before := main.loaded_zone
		online.move_to(Vector3(best.x, 0, best.z))
		var waited := 0.0
		while main.loaded_zone == zone_before and waited < 120.0:
			await get_tree().create_timer(0.5).timeout
			waited += 0.5
		print("rose net demo: now in zone %d" % main.loaded_zone)
		trips += 1
