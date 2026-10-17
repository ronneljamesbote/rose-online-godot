extends "res://demos/demo.gd"

## Puts the first two bag consumables on hotbar keys 1 and 2, moves the first one to the
## 11th bag slot, then drops it. Prints the hotbar after each step: key 1 should keep the
## same item after the move and be empty after the drop, while key 2 never changes.
func run() -> void:
	await get_tree().create_timer(3.0).timeout
	online.inventory_window.visible = true
	online.inventory_window.tabs.current_tab = 1
	var page := 1
	var items: Array = online.net.get_inventory()["pages"][page]
	var filled: Array = []
	for i in items.size():
		if items[i] != null:
			filled.append(i)
	if filled.size() < 2:
		print("rose net demo: needs two consumables in the bag")
		return
	online.net.set_hotbar(0, "item", page, filled[0])
	online.net.set_hotbar(1, "item", page, filled[1])
	await _show("on the hotbar")
	online.net.move_item(page, filled[0], 10)
	await _show("moved bag slot %d to 10" % filled[0])
	online.net.drop_item(page, 10, items[filled[0]].get("quantity", 1))
	await _show("dropped bag slot 10")


func _show(what: String) -> void:
	await get_tree().create_timer(3.0).timeout
	var names: Array = online.net.get_hotbar().slice(0, 2).map(
		func(e): return "-" if e == null else "%s (bag %d)" % [e["name"], e["index"]])
	print("rose net demo: %s: key 1 %s, key 2 %s" % [what, names[0], names[1]])
