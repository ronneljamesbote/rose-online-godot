## Crafting window, opened by using a craft skill (Sword Craft, Armor Craft, Gem Cutting, ...):
## the items the skill can make at its level on the left, the chosen item's materials on
## the right with what the bag holds, and Craft. Each material is a step that can fail;
## the server rolls them.
extends PanelContainer

var net: RoseNet
var skill_page := -1
var skill_index := -1
var title: Label
var list: ItemList
var item_label: Label
var materials_box: VBoxContainer
var craft_button: Button
var entries: Array = []
var selected := -1
var _slots: Array = []
var _refresh_in := 0.0


func _ready() -> void:
	custom_minimum_size = Vector2(560, 360)
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
	title.add_theme_font_size_override("font_size", 18)
	title.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	top.add_child(title)
	var close := Button.new()
	close.text = "Close"
	close.focus_mode = Control.FOCUS_NONE
	close.pressed.connect(func(): visible = false)
	top.add_child(close)

	var columns := HBoxContainer.new()
	columns.size_flags_vertical = Control.SIZE_EXPAND_FILL
	columns.add_theme_constant_override("separation", 10)
	box.add_child(columns)
	list = ItemList.new()
	list.custom_minimum_size = Vector2(250, 300)
	list.fixed_icon_size = Vector2i(32, 32)
	list.focus_mode = Control.FOCUS_NONE
	list.item_selected.connect(_select)
	columns.add_child(list)

	var right := VBoxContainer.new()
	right.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	right.add_theme_constant_override("separation", 6)
	columns.add_child(right)
	item_label = Label.new()
	item_label.autowrap_mode = TextServer.AUTOWRAP_WORD
	right.add_child(item_label)
	materials_box = VBoxContainer.new()
	right.add_child(materials_box)
	var spacer := Control.new()
	spacer.size_flags_vertical = Control.SIZE_EXPAND_FILL
	right.add_child(spacer)
	craft_button = Button.new()
	craft_button.text = "Craft"
	craft_button.focus_mode = Control.FOCUS_NONE
	craft_button.disabled = true
	craft_button.pressed.connect(craft)
	right.add_child(craft_button)


## Open for the craft skill in this skill slot. False when it doesn't craft.
func open_skill(page: int, index: int, skill: Dictionary) -> bool:
	entries = net.get_craft_items(page, index)
	if entries.is_empty():
		return false
	skill_page = page
	skill_index = index
	title.text = "%s   Level %d" % [skill.get("name", "Craft"), skill.get("level", 1)]
	list.clear()
	for e in entries:
		var i := list.add_item("%s   (Lv %d)" % [e["item"].get("name", "?"), e["level"]], e["item"].get("icon"))
		list.set_item_tooltip(i, e["item"].get("tooltip", ""))
	visible = true
	list.select(0)
	_select(0)
	return true


func _select(index: int) -> void:
	selected = index
	_refresh_in = 0.0


func _show_materials() -> void:
	for child in materials_box.get_children():
		child.queue_free()
	if selected < 0 or selected >= entries.size():
		craft_button.disabled = true
		return
	var e: Dictionary = entries[selected]
	item_label.text = "%s\n%s" % [e["item"].get("name", "?"), e["item"].get("tooltip", "")]
	_slots = net.find_craft_materials(e["type"], e["number"])
	var ready := not _slots.is_empty()
	for i in e["materials"].size():
		var m: Dictionary = e["materials"][i]
		var have: bool = i < _slots.size() and _slots[i][0] >= 0
		ready = ready and have
		var row := HBoxContainer.new()
		var icon := TextureRect.new()
		icon.custom_minimum_size = Vector2(28, 28)
		icon.expand_mode = TextureRect.EXPAND_IGNORE_SIZE
		icon.texture = m.get("icon")
		row.add_child(icon)
		var label := Label.new()
		label.text = "Step %d: %s x%d%s" % [i + 1, m["name"], m["quantity"], "" if have else "   (missing)"]
		if not have:
			label.modulate = Color(1, 0.55, 0.5)
		row.add_child(label)
		materials_box.add_child(row)
	craft_button.disabled = not ready


func craft() -> void:
	if selected < 0 or selected >= entries.size():
		return
	var e: Dictionary = entries[selected]
	net.craft_item(skill_page, skill_index, e["type"], e["number"], _slots)
	_refresh_in = 0.5


func _process(delta: float) -> void:
	if not visible:
		return
	_refresh_in -= delta
	if _refresh_in <= 0.0:
		_refresh_in = 1.0
		_show_materials()


func _unhandled_key_input(event: InputEvent) -> void:
	if visible and event is InputEventKey and event.pressed and event.keycode == KEY_ESCAPE:
		visible = false
