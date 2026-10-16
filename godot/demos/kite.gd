extends "res://demos/demo.gd"

## Hits the nearest monster once, then keeps stepping away from it so it chases and swings
## at a moving target (every swing it starts should land, not restart).
func run() -> void:
	await get_tree().create_timer(float(options.get("demo-wait", "1.5"))).timeout
	# --walk-to=X,Z;X,Z... (zone positions) first, to fight somewhere open.
	for spot in String(options.get("walk-to", "")).split(";", false):
		var to: PackedFloat64Array = spot.split_floats(",")
		online.move_to(Vector3(to[0], online.me.position.y, to[1]))
		await get_tree().create_timer(0.5).timeout
		while online.me.is_moving:
			await get_tree().create_timer(0.25).timeout
	await get_tree().create_timer(1.0).timeout
	# The nearest monster that fights back (butterflies, npc 316 and up, don't).
	var target := -1
	for id in online.entities:
		var e: Node3D = online.entities[id]
		if e.is_monster and not e.dead and e.npc_id < 316 and (target < 0
				or e.position.distance_to(online.me.position) < online.entities[target].position.distance_to(online.me.position)):
			target = id
	if target < 0:
		return
	print("rose net demo: hit ", online.entities[target].label.text, " once, then walk away")
	online.attack(target)
	while online.me and not online.me.was_swinging:
		await get_tree().process_frame
	await get_tree().create_timer(0.7).timeout
	# Count the monster's wind-ups; its hits show in the damage log.
	var windups := 0
	print("rose net demo: kiting starts at %.1f s" % (Time.get_ticks_msec() / 1000.0))
	for i in 20:
		if not online.entities.has(target) or online.me == null or online.me.dead:
			break
		var away: Vector3 = online.me.position - online.entities[target].position
		away.y = 0
		online.move_to(online.me.position + away.normalized() * float(options.get("kite-step", "2")))
		var waited := 0.0
		var was: bool = online.entities[target].was_swinging
		while waited < 1.2 and online.entities.has(target):
			await get_tree().process_frame
			waited += get_process_delta_time()
			var now: bool = online.entities[target].was_swinging
			if now and not was:
				windups += 1
			was = now
	print("rose net demo: monster wind-ups %d" % windups)
