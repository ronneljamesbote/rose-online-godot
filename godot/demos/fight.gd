extends "res://demos/demo.gd"

## Fights the nearest monster until it dies, then the next one. Every third fight starts
## with a swing cancelled by a step to the side, to show animation cancelling.
func run() -> void:
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
		# Pick up what it dropped (drops land a moment after the kill).
		await get_tree().create_timer(0.5).timeout
		var item: int = online.nearest_item() if not options.has("no-pickup") else -1
		if item >= 0:
			print("rose net demo: pick up ", online.ground[item].get_meta("item", {}).get("name", "?"))
			online.pickup(item)
			var waited := 0.0
			while online.ground.has(item) and waited < 5.0:
				await get_tree().create_timer(0.25).timeout
				waited += 0.25
