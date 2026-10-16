extends "res://demos/demo.gd"

## Puts the first two active skills on the hotbar, uses the first (a buff) and then the
## second on the nearest monster, over and over, between normal attacks.
func run() -> void:
	await get_tree().create_timer(4.0).timeout
	var active: Array = online.net.get_skills()[1]
	# The first active skill (a buff on ourselves) and the last one learned (used on monsters).
	var slots := []
	for i in active.size():
		if active[i] != null:
			slots.append(i)
	if slots.size() > 2:
		slots = [slots[0], slots[-1]]
	var names := []
	for n in slots.size():
		online.net.set_hotbar(n, "skill", 1, slots[n])
		names.append(active[slots[n]]["name"])
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
