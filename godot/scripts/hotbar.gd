## Hotbar along the bottom: eight slots for items and skills, used with keys 1-8 or a
## right-click. Drag an item from the inventory or a skill from the skill window onto a
## slot; Shift+right-click clears it.
extends PanelContainer

const InventoryWindow := preload("res://scripts/inventory_window.gd")
const SIZE := 8

var net: RoseNet
var online: Node
var slots: Array = []
var _last := []


func _ready() -> void:
	var margin := MarginContainer.new()
	for side in ["left", "right", "top", "bottom"]:
		margin.add_theme_constant_override("margin_" + side, 4)
	add_child(margin)
	var row := HBoxContainer.new()
	margin.add_child(row)
	for i in SIZE:
		var slot = InventoryWindow.Slot.new(self, "hotbar", i, str(i + 1))
		row.add_child(slot)
		slots.append(slot)


func _process(_delta: float) -> void:
	if net == null:
		return
	var entries: Array = net.get_hotbar()
	if entries == _last:
		return
	_last = entries
	for i in slots.size():
		slots[i].show_item(entries[i] if i < entries.size() else null)


## Key 1-8 or right-click: use what is on the slot.
func use_slot(index: int) -> void:
	if index < 0 or index >= slots.size():
		return
	var entry = slots[index].item
	if entry == null:
		return
	if entry["kind"] == "skill":
		online.use_skill(entry["page"], entry["index"], entry)
	elif entry["page"] == 1:
		net.use_item(entry["page"], entry["index"])
	else:
		net.equip_item(entry["page"], entry["index"])


## Slot callbacks (see inventory_window.gd's Slot).
func activate(slot) -> void:
	if Input.is_key_pressed(KEY_SHIFT):
		net.set_hotbar(slot.index, "", 0, 0)
	else:
		use_slot(slot.index)


func select(_slot) -> void:
	pass


func dropped(from, to) -> void:
	match from.kind:
		"page":
			net.set_hotbar(to.index, "item", from.window.page, from.index)
		"skill":
			net.set_hotbar(to.index, "skill", from.window.page, from.index)
		"hotbar":
			# Swap two hotbar slots.
			var a = from.item
			var b = to.item
			net.set_hotbar(to.index, a["kind"], a["page"], a["index"])
			if b == null:
				net.set_hotbar(from.index, "", 0, 0)
			else:
				net.set_hotbar(from.index, b["kind"], b["page"], b["index"])
