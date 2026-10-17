extends "res://demos/demo.gd"

## Puts two basic skills on hotbar keys 1 and 2 and the first bag item (a consumable if there is one) on key 3, then prints
## the hotbar every two seconds while the slots are dragged with the mouse: one dragged
## off the hotbar should empty, one dragged onto another slot should move there.
## Prints where each hotbar slot is on screen so a script (xdotool) can do the drags.
func run() -> void:
	await get_tree().create_timer(3.0).timeout
	var basic: Array = online.net.get_skills()[0]
	var key := 0
	for i in basic.size():
		if basic[i] != null and key < 2:
			online.net.set_hotbar(key, "skill", 0, i)
			key += 1
	var pages: Array = online.net.get_inventory()["pages"]
	for page in [1, 0, 2, 3]:
		var i: int = pages[page].find_custom(func(item): return item != null)
		if i >= 0:
			online.net.set_hotbar(2, "item", page, i)
			break
	await get_tree().create_timer(2.0).timeout
	for slot in online.hotbar.slots:
		var r: Rect2 = slot.get_global_rect()
		print("rose net demo: slot %d at %d,%d" % [slot.index + 1, r.get_center().x, r.get_center().y])
	var last := ""
	while true:
		var names: Array = online.net.get_hotbar().slice(0, 8).map(
			func(e): return "-" if e == null else String(e["name"]))
		var line := ", ".join(names)
		if line != last:
			print("rose net demo: hotbar 1-8: " + line)
			last = line
		await get_tree().create_timer(0.5).timeout
