## Quest list (Q): our active quests with what they ask for, their quest items and time
## left. Abandon gives a quest up.
extends PanelContainer

var net: RoseNet
var list: VBoxContainer
var empty: Label
var _last := []


func _ready() -> void:
	custom_minimum_size = Vector2(380, 0)
	var margin := MarginContainer.new()
	for side in ["left", "right", "top", "bottom"]:
		margin.add_theme_constant_override("margin_" + side, 10)
	add_child(margin)
	var box := VBoxContainer.new()
	box.add_theme_constant_override("separation", 8)
	margin.add_child(box)
	var title := Label.new()
	title.text = "Quests"
	title.add_theme_font_size_override("font_size", 18)
	box.add_child(title)
	empty = Label.new()
	empty.text = "No quests. Talk to people in town to find some."
	empty.modulate = Color(1, 1, 1, 0.6)
	box.add_child(empty)
	list = VBoxContainer.new()
	list.add_theme_constant_override("separation", 10)
	box.add_child(list)


func _process(_delta: float) -> void:
	if not visible or net == null:
		return
	var quests: Array = net.get_quests()
	if quests == _last:
		return
	_last = quests
	for child in list.get_children():
		child.queue_free()
	empty.visible = quests.is_empty()
	for quest in quests:
		list.add_child(_quest_entry(quest))
	reset_size()


func _quest_entry(quest: Dictionary) -> Control:
	var box := VBoxContainer.new()
	var top := HBoxContainer.new()
	box.add_child(top)
	var name := Label.new()
	name.text = quest["name"]
	name.add_theme_font_size_override("font_size", 16)
	name.add_theme_color_override("font_color", Color(1.0, 0.85, 0.5))
	name.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	top.add_child(name)
	if quest["time_left"] >= 0:
		var timer := Label.new()
		var left: int = quest["time_left"]
		timer.text = "%d:%02d left" % [left / 60, left % 60]
		top.add_child(timer)
	var abandon := Button.new()
	abandon.text = "Abandon"
	abandon.focus_mode = Control.FOCUS_NONE
	abandon.pressed.connect(func(): net.abandon_quest(quest["slot"], quest["id"]))
	top.add_child(abandon)
	if quest["description"] != "":
		var text := RichTextLabel.new()
		text.bbcode_enabled = true
		text.fit_content = true
		text.scroll_active = false
		text.custom_minimum_size = Vector2(356, 0)
		text.text = quest["description"]
		box.add_child(text)
	var items: Array = quest["items"]
	if not items.is_empty():
		var row := HBoxContainer.new()
		for item in items:
			var icon := TextureRect.new()
			icon.custom_minimum_size = Vector2(32, 32)
			icon.expand_mode = TextureRect.EXPAND_IGNORE_SIZE
			icon.texture = item.get("icon")
			icon.tooltip_text = item.get("tooltip", "")
			row.add_child(icon)
			var count := Label.new()
			count.text = "%s x%d" % [item.get("name", "?"), item.get("quantity", 1)]
			row.add_child(count)
		box.add_child(row)
	return box
