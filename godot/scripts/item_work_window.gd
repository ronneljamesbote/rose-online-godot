## Refining and disassembly, at an NPC whose dialog offers it (paid in Zuly) or with the
## Item Refining / Item Disassembly skills (paid in MP). Right-click a bag item, or drag it
## onto the slot, to pick it; the window shows the materials and cost. A disassembled item
## with a set gem gives the gem back instead.
extends PanelContainer

const InventoryWindow := preload("res://scripts/inventory_window.gd")
const NPC_RANGE := 15.0  # metres; walking further away closes the window

var net: RoseNet
var online: Node  # online.gd, for our character
var inventory_window: Control
var mode := "refine"  # or "disassemble"
var npc_id := -1  # the NPC's entity id, or -1 when a skill does the work
var npc: Node3D
var skill_page := -1
var skill_index := -1
var target_page := -1
var target_index := -1
var title: Label
var slot
var info: Label
var materials_box: VBoxContainer
var cost: Label
var button: Button
var _info := {}
var _refresh_in := 0.0


func _ready() -> void:
	custom_minimum_size = Vector2(340, 0)
	var margin := MarginContainer.new()
	for side in ["left", "right", "top", "bottom"]:
		margin.add_theme_constant_override("margin_" + side, 10)
	add_child(margin)
	var box := VBoxContainer.new()
	box.add_theme_constant_override("separation", 6)
	margin.add_child(box)

	var top := HBoxContainer.new()
	box.add_child(top)
	title = Label.new()
	title.theme_type_variation = "HeaderLabel"
	title.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	top.add_child(title)
	var close := Button.new()
	close.text = "Close"
	close.focus_mode = Control.FOCUS_NONE
	close.pressed.connect(close_window)
	top.add_child(close)

	var row := HBoxContainer.new()
	row.add_theme_constant_override("separation", 8)
	box.add_child(row)
	slot = InventoryWindow.Slot.new(self, "work", 0)
	row.add_child(slot)
	info = Label.new()
	info.autowrap_mode = TextServer.AUTOWRAP_WORD
	info.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	row.add_child(info)
	materials_box = VBoxContainer.new()
	box.add_child(materials_box)
	cost = Label.new()
	box.add_child(cost)
	button = Button.new()
	button.focus_mode = Control.FOCUS_NONE
	button.pressed.connect(work)
	box.add_child(button)


## Open for an NPC's service.
func open_npc(kind: String, entity_id: int, npc_node: Node3D) -> void:
	npc_id = entity_id
	npc = npc_node
	skill_page = -1
	skill_index = -1
	_open(kind, npc_node.label.text if npc_node.get("label") else "")


## Open for our Item Refining or Item Disassembly skill in this skill slot.
func open_skill(kind: String, page: int, index: int, skill: Dictionary) -> void:
	npc_id = -1
	npc = null
	skill_page = page
	skill_index = index
	_open(kind, skill.get("name", ""))


func _open(kind: String, by: String) -> void:
	mode = kind
	title.text = ("Refine" if mode == "refine" else "Disassemble") + ("   " + by if by != "" else "")
	button.text = "Refine" if mode == "refine" else "Disassemble"
	target_page = -1
	target_index = -1
	visible = true
	if inventory_window:
		inventory_window.visible = true
		# Just left of the inventory window.
		offset_right = inventory_window.offset_right - inventory_window.get_combined_minimum_size().x - 12
	_refresh_in = 0.0


func close_window() -> void:
	visible = false
	npc_id = -1
	npc = null


func set_target(page: int, index: int) -> void:
	target_page = page
	target_index = index
	_refresh_in = 0.0


func _show() -> void:
	for child in materials_box.get_children():
		child.queue_free()
	_info = {}
	if target_page >= 0:
		_info = net.get_refine_info(target_page, target_index) if mode == "refine" else net.get_disassemble_info(target_page, target_index)
		if _info.is_empty():  # the item is gone (taken apart, moved)
			target_page = -1
	slot.show_item(_info.get("item"))
	var ready := false
	if _info.is_empty():
		info.text = "Right-click an item in your bag to %s it." % mode
		cost.text = ""
	elif _info.has("error"):
		info.text = _info["error"]
		cost.text = ""
	elif mode == "refine":
		info.text = "%s\nGrade %d → %d. Failing can lower the grade." % [_name(), _info["grade"], _info["grade"] + 1]
		ready = _info["ready"]
		for i in _info["materials"].size():
			var m: Dictionary = _info["materials"][i]
			var have: bool = i < _info["slots"].size() and _info["slots"][i][0] >= 0
			materials_box.add_child(_material_row(m.get("icon"), "%s x%d%s" % [m["name"], m["quantity"], "" if have else "   (missing)"], have))
	elif _info.has("gem"):
		info.text = "%s\nTakes out %s. It can lose a grade or break." % [_name(), _info["gem"]]
		ready = true
	else:
		info.text = "%s\nTakes it apart into some of:" % _name()
		for name in _info["outputs"]:
			materials_box.add_child(_material_row(null, name, true))
		ready = true
	if not _info.is_empty() and not _info.has("error"):
		cost.text = "Costs %d MP" % _info["mp"] if npc_id < 0 else "Costs %d Zuly" % _info["zuly"]
	button.disabled = not ready


## The item's name with its grade, like "Bushido +5".
func _name() -> String:
	var item: Dictionary = _info["item"]
	var grade: int = item.get("grade", 0)
	return item.get("name", "?") + (" +%d" % grade if grade > 0 else "")


func _material_row(texture, text: String, have: bool) -> Control:
	var row := HBoxContainer.new()
	var icon := TextureRect.new()
	icon.custom_minimum_size = Vector2(24, 24)
	icon.expand_mode = TextureRect.EXPAND_IGNORE_SIZE
	icon.texture = texture
	row.add_child(icon)
	var label := Label.new()
	label.text = text
	if not have:
		label.modulate = Color(1, 0.55, 0.5)
	row.add_child(label)
	return row


func work() -> void:
	if target_page < 0 or button.disabled:
		return
	if mode == "refine":
		net.refine_item(npc_id, skill_page, skill_index, target_page, target_index, _info.get("slots", []))
	else:
		net.disassemble_item(npc_id, skill_page, skill_index, target_page, target_index)
	_refresh_in = 0.5


## Slot callbacks (see inventory_window.gd's Slot).
func activate(_slot) -> void:
	set_target(-1, -1)


func select(_slot) -> void:
	pass


func dropped(from, _to) -> void:
	set_target(inventory_window.page, from.index)


func _process(delta: float) -> void:
	if not visible:
		return
	if npc_id >= 0:
		var me: Node3D = online.me if online else null
		if npc == null or not is_instance_valid(npc) or me == null or me.position.distance_to(npc.position) > NPC_RANGE:
			close_window()
			return
	_refresh_in -= delta
	if _refresh_in <= 0.0:
		_refresh_in = 1.0
		_show()


func _unhandled_key_input(event: InputEvent) -> void:
	if visible and event is InputEventKey and event.pressed and event.keycode == KEY_ESCAPE:
		close_window()
