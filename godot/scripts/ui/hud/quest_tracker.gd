## Our quests on screen, under the minimap: each quest's name, the quest items collected
## so far and the time left. Click a quest to open the quest window.
extends VBoxContainer

signal open_quests

var _last := []


func _ready() -> void:
	mouse_filter = Control.MOUSE_FILTER_IGNORE
	add_theme_constant_override("separation", 6)
	custom_minimum_size.x = 240


## quests: RoseNet.get_quests().
func refresh(quests: Array) -> void:
	var key := []
	for q in quests:
		key.append([q.get("name", ""), q.get("items", []).map(func(i): return [i.get("name", ""), i.get("quantity", 1)]), int(q.get("time_left", -1)) / 10])
	if key == _last:
		return
	_last = key
	for child in get_children():
		child.queue_free()
	if quests.is_empty():
		return
	var header := Label.new()
	header.text = "Quests"
	header.theme_type_variation = "AccentLabel"
	_outline(header)
	add_child(header)
	for q in quests.slice(0, 5):
		var entry := VBoxContainer.new()
		entry.add_theme_constant_override("separation", 0)
		entry.mouse_filter = Control.MOUSE_FILTER_STOP
		entry.mouse_default_cursor_shape = Control.CURSOR_POINTING_HAND
		entry.gui_input.connect(func(e):
			if e is InputEventMouseButton and e.pressed and e.button_index == MOUSE_BUTTON_LEFT:
				open_quests.emit())
		add_child(entry)
		var name_label := Label.new()
		name_label.text = "◆ " + str(q.get("name", ""))
		name_label.theme_type_variation = "HeaderLabel"
		name_label.add_theme_font_size_override("font_size", 14)
		name_label.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
		name_label.custom_minimum_size.x = 240
		_outline(name_label)
		entry.add_child(name_label)
		for item in q.get("items", []):
			var line := Label.new()
			line.text = "    %s  %d" % [item.get("name", "?"), item.get("quantity", 1)]
			line.theme_type_variation = "SmallLabel"
			_outline(line)
			entry.add_child(line)
		var left := int(q.get("time_left", -1))
		if left >= 0:
			var time := Label.new()
			time.text = "    %d:%02d left" % [left / 60, left % 60]
			time.theme_type_variation = "SmallLabel"
			_outline(time)
			entry.add_child(time)


## Quest text floats over the world, so it gets a dark outline in every theme.
static func _outline(label: Label) -> void:
	label.add_theme_color_override("font_color", Color.WHITE if label.theme_type_variation != "AccentLabel" else UI.color("colors.accent"))
	label.add_theme_color_override("font_outline_color", Color(0, 0, 0, 0.8))
	label.add_theme_constant_override("outline_size", 5)
	label.mouse_filter = Control.MOUSE_FILTER_IGNORE
