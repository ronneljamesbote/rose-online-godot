extends "res://demos/demo.gd"

## Pulls the nearest monster that fights back (whose name has --monster=TEXT), after
## walking to --pull-from metres from it if given. Then keeps attacking from where it stands
## (--leash=stand), shoots once and just stands there (--leash=once), or runs --run-to metres
## straight away from it after the first hit (--leash=run). Prints once a second how far the
## monster is from the player and from where it was hit, and its HP, so the leash shows: a
## pulled monster fights, a dragged one gives up, heals and walks home.
func run() -> void:
	await get_tree().create_timer(float(options.get("demo-wait", "1.5"))).timeout
	var mode := String(options.get("leash", "stand"))
	var only := String(options.get("monster", "")).to_lower()
	var target := -1
	for id in online.entities:
		var e: Node3D = online.entities[id]
		if e.is_monster and not e.dead and e.npc_id < 316 and only in e.label.text.to_lower() and (target < 0
				or e.position.distance_to(online.me.position) < online.entities[target].position.distance_to(online.me.position)):
			target = id
	# --target-file=PATH holds "ENTITY HOME_X HOME_Y" (server units), written by a test script
	# meanwhile: pull that monster from the side away from its home.
	var home := Vector3.INF
	if options.has("target-file"):
		while not FileAccess.file_exists(String(options["target-file"])):
			await get_tree().create_timer(0.5).timeout
		await get_tree().create_timer(3.0).timeout
		var f: PackedStringArray = FileAccess.get_file_as_string(String(options["target-file"])).split(" ")
		target = int(f[0])
		home = Vector3(float(f[1]) / 100.0, 0, -float(f[2]) / 100.0)
	if not online.entities.has(target):
		print("rose net demo: no monster nearby")
		return
	var monster: Node3D = online.entities[target]
	# Stand --pull-from metres from it before the first shot.
	var away: Vector3 = online.me.position - monster.position
	if home != Vector3.INF:
		away = monster.position - home
	away.y = 0
	if options.has("pull-from"):
		online.move_to(monster.position + away.normalized() * float(options["pull-from"]))
		await get_tree().create_timer(0.5).timeout
		while online.me.is_moving:
			await get_tree().create_timer(0.25).timeout
	var hit_at: Vector3 = monster.position
	print("rose net demo: %s, pull it from %.1f m (%s)" % [monster.label.text, online.me.position.distance_to(hit_at), mode])
	# --overhead=METRES: for recordings, a fixed camera that high above the two of us.
	if options.has("overhead"):
		var cam := Camera3D.new()
		main.add_child(cam)
		var mid: Vector3 = (online.me.position + monster.position) / 2.0
		cam.position = mid + Vector3(0, float(options["overhead"]), float(options["overhead"]) * 0.35)
		cam.look_at(mid)
		cam.make_current()
	var hp_before: int = monster.hp
	online.attack(target)
	while online.entities.has(target) and monster.hp == hp_before:
		await get_tree().process_frame
	if not online.entities.has(target):
		print("rose net demo: monster gone (killed)")
		return
	if mode == "once":
		online.stop()
	if mode == "run":
		away = online.me.position - monster.position
		away.y = 0
		online.move_to(online.me.position + away.normalized() * float(options.get("run-to", "80")))
	var gave_up := false
	for i in int(options.get("leash-time", "25")):
		await get_tree().create_timer(1.0).timeout
		if not online.entities.has(target) or online.me == null:
			print("rose net demo: monster gone (killed or out of sight)")
			return
		var m: Node3D = online.entities[target]
		var flat := Vector3(m.position.x, hit_at.y, m.position.z)
		print("rose net demo: t=%2d s  monster %.1f m from me, %.1f m from where it was hit, HP %d/%d" % [
				i + 1, m.position.distance_to(online.me.position), flat.distance_to(hit_at), m.hp, m.max_hp])
		if not gave_up and i > 2 and m.hp == m.max_hp and m.position.distance_to(online.me.position) > 5.0:
			gave_up = true
			print("rose net demo: it gave up and walks home")
	if not gave_up:
		print("rose net demo: it never gave up")
