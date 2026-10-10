## Options (O): the interface theme (Arumic by default; every file in the themes folder is
## a card), interface size, window look, layout, labels over the world, and sound.
extends PanelContainer

const Fx := preload("res://scripts/fx.gd")

var tabs: TabBar
var pages: Array[Control] = []
var cards: HFlowContainer
var problems: Label
var _music: HSlider
var _effects: HSlider


func _ready() -> void:
	custom_minimum_size = Vector2(560, 0)
	var column := VBoxContainer.new()
	column.add_theme_constant_override("separation", 10)
	add_child(column)
	tabs = TabBar.new()
	tabs.focus_mode = Control.FOCUS_NONE
	for name in ["Interface", "World labels", "Sound"]:
		tabs.add_tab(name)
	tabs.tab_changed.connect(_show_page)
	column.add_child(tabs)
	pages.append(_interface_page())
	pages.append(_labels_page())
	pages.append(_sound_page())
	for page in pages:
		column.add_child(page)
	_show_page(0)
	UI.theme_changed.connect(_fill_cards)
	_fill_cards()


func _show_page(index: int) -> void:
	for i in pages.size():
		pages[i].visible = i == index


func _section(box: Container, text: String) -> void:
	var label := Label.new()
	label.text = text
	label.theme_type_variation = "HeaderLabel"
	box.add_child(label)


func _interface_page() -> Control:
	var box := VBoxContainer.new()
	box.add_theme_constant_override("separation", 8)
	_section(box, "Theme")
	cards = HFlowContainer.new()
	cards.add_theme_constant_override("h_separation", 8)
	cards.add_theme_constant_override("v_separation", 8)
	box.add_child(cards)
	problems = Label.new()
	problems.theme_type_variation = "BadLabel"
	problems.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
	problems.custom_minimum_size.x = 520
	box.add_child(problems)
	var files := HBoxContainer.new()
	files.add_theme_constant_override("separation", 8)
	box.add_child(files)
	var hint := Label.new()
	hint.text = "Make your own: copy a file in the themes folder and change it."
	hint.theme_type_variation = "MutedLabel"
	hint.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	hint.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
	files.add_child(hint)
	files.add_child(_button("Open themes folder", func(): OS.shell_open(UI.themes_dir())))
	files.add_child(_button("Reload themes", func(): UI.reload_themes()))
	box.add_child(HSeparator.new())

	_section(box, "Size and look")
	var grid := GridContainer.new()
	grid.columns = 2
	grid.add_theme_constant_override("h_separation", 16)
	grid.add_theme_constant_override("v_separation", 8)
	box.add_child(grid)
	_slider(grid, "Interface size", "scale", 0.75, 1.5, 0.05, true)
	_slider(grid, "Window opacity", "opacity", 0.4, 1.0, 0.05, true)
	_check(box, "Blur the world behind windows", "blur")
	box.add_child(HSeparator.new())

	_section(box, "Layout")
	var row := HBoxContainer.new()
	row.add_theme_constant_override("separation", 12)
	box.add_child(row)
	_check(row, "Lock windows and HUD in place", "locked")
	var spacer := Control.new()
	spacer.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	row.add_child(spacer)
	row.add_child(_button("Reset layout", func(): UI.reset_layout()))
	var tip := Label.new()
	tip.text = "Drag windows by their title bar and HUD pieces by the handle that shows on hover. Drag a window's corner to resize it."
	tip.theme_type_variation = "MutedLabel"
	tip.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
	tip.custom_minimum_size.x = 520
	box.add_child(tip)
	return box


func _labels_page() -> Control:
	var box := VBoxContainer.new()
	box.add_theme_constant_override("separation", 8)
	_section(box, "Name tags")
	_check(box, "Show my own name tag", "own_tag")
	_check(box, "Show other players' name tags", "name_tags")
	_check(box, "Chat bubbles", "bubbles")
	_check(box, "Damage numbers", "damage_numbers")
	box.add_child(HSeparator.new())
	_section(box, "Items on the ground")
	var mode := OptionButton.new()
	mode.focus_mode = Control.FOCUS_NONE
	mode.add_item("Always show labels (hold Alt to hide)")
	mode.add_item("Only while Alt is held")
	mode.selected = 1 if UI.settings.get("item_labels", "always") == "alt" else 0
	mode.item_selected.connect(func(i): UI.set_setting("item_labels", "alt" if i == 1 else "always"))
	box.add_child(mode)
	var tip := Label.new()
	tip.text = "Click a label to pick the item up. Faded labels are someone else's drop."
	tip.theme_type_variation = "MutedLabel"
	box.add_child(tip)
	return box


func _sound_page() -> Control:
	var box := VBoxContainer.new()
	box.add_theme_constant_override("separation", 8)
	_section(box, "Volume")
	var grid := GridContainer.new()
	grid.columns = 2
	grid.add_theme_constant_override("h_separation", 16)
	grid.add_theme_constant_override("v_separation", 8)
	box.add_child(grid)
	var fx := Fx.find(self) if is_inside_tree() else null
	_music = _plain_slider(grid, "Music", fx.music_volume if fx else 1.0)
	_effects = _plain_slider(grid, "Sound effects", fx.effects_volume if fx else 1.0)
	for slider in [_music, _effects]:
		slider.value_changed.connect(func(_v):
			var f := Fx.find(self)
			if f:
				f.set_volumes(_music.value, _effects.value))
	return box


func _button(text: String, action: Callable) -> Button:
	var b := Button.new()
	b.text = text
	b.focus_mode = Control.FOCUS_NONE
	b.pressed.connect(action)
	return b


func _check(box: Container, text: String, key: String) -> void:
	var check := CheckBox.new()
	check.text = text
	check.focus_mode = Control.FOCUS_NONE
	check.button_pressed = bool(UI.settings.get(key, false))
	check.toggled.connect(func(on): UI.set_setting(key, on))
	box.add_child(check)


func _slider(grid: GridContainer, text: String, key: String, low: float, high: float, step: float, percent: bool) -> void:
	var label := Label.new()
	label.text = text
	grid.add_child(label)
	var row := HBoxContainer.new()
	row.add_theme_constant_override("separation", 8)
	grid.add_child(row)
	var slider := HSlider.new()
	slider.min_value = low
	slider.max_value = high
	slider.step = step
	slider.value = float(UI.settings.get(key, 1.0))
	slider.custom_minimum_size.x = 260
	slider.size_flags_vertical = Control.SIZE_SHRINK_CENTER
	slider.focus_mode = Control.FOCUS_NONE
	row.add_child(slider)
	var value := Label.new()
	value.custom_minimum_size.x = 48
	value.text = "%d%%" % roundi(slider.value * 100.0) if percent else str(slider.value)
	row.add_child(value)
	# Interface size is applied when the slider is let go, so the window doesn't jump while
	# dragging; opacity follows the slider. A click on the track or the mouse wheel moves
	# the slider without a drag, so those apply at once.
	var dragging := [false]
	slider.drag_started.connect(func(): dragging[0] = true)
	slider.drag_ended.connect(func(_changed):
		dragging[0] = false
		if UI.settings.get(key) != slider.value:
			UI.set_setting(key, slider.value))
	slider.value_changed.connect(func(v):
		value.text = "%d%%" % roundi(v * 100.0) if percent else str(v)
		if key == "opacity" or not dragging[0]:
			UI.set_setting(key, v))


func _plain_slider(grid: GridContainer, text: String, at: float) -> HSlider:
	var label := Label.new()
	label.text = text
	grid.add_child(label)
	var slider := HSlider.new()
	slider.min_value = 0.0
	slider.max_value = 1.0
	slider.step = 0.05
	slider.value = at
	slider.custom_minimum_size.x = 260
	slider.size_flags_vertical = Control.SIZE_SHRINK_CENTER
	slider.focus_mode = Control.FOCUS_NONE
	grid.add_child(slider)
	return slider


## One card per theme: a small preview in its own colours, its name and description.
func _fill_cards() -> void:
	for child in cards.get_children():
		child.queue_free()
	var ids: Array = UI.themes.keys()
	# Built-in themes first, in their usual order, then the player's own.
	ids.sort_custom(func(a, b):
		var ia: int = UI.BUILTIN.find(a)
		var ib: int = UI.BUILTIN.find(b)
		if ia >= 0 and ib >= 0:
			return ia < ib
		if ia >= 0 or ib >= 0:
			return ia >= 0
		return a < b)
	var lines: Array[String] = []
	for id in ids:
		var info: Dictionary = UI.themes[id]
		var card := _Card.new()
		card.info = info
		card.selected = id == UI.settings["theme"]
		card.pressed.connect(func(): UI.set_setting("theme", id))
		cards.add_child(card)
		for e in info["errors"]:
			lines.append("%s: %s" % [info["name"], e])
	problems.text = "\n".join(lines) if not lines.is_empty() else ""
	problems.visible = not lines.is_empty()


class _Card extends Button:
	var info: Dictionary
	var selected := false

	func _ready() -> void:
		custom_minimum_size = Vector2(124, 112)
		focus_mode = Control.FOCUS_NONE
		theme_type_variation = "FlatButton"
		tooltip_text = "%s\n%s" % [info["name"], info["description"]]
		if info["file"] != "":
			tooltip_text += "\n" + info["file"].get_file()

	func _draw() -> void:
		var t: Dictionary = info["tokens"]
		var col := func(path: String, fallback: Color) -> Color:
			var at = t
			for part in path.split("."):
				at = at.get(part) if at is Dictionary else null
			var c = UI.parse_color(at) if at is String else null
			return c if c != null else fallback
		var preview := Rect2(Vector2(6, 6), Vector2(size.x - 12, 64))
		# A tiny scene behind the glass so see-through themes read correctly.
		draw_style_box(UiBox.make(Color(0.47, 0.62, 0.86), Color(0.55, 0.66, 0.42), 8), preview)
		var panel := UiBox.make(col.call("panel.top", Color.WHITE), col.call("panel.bottom", Color.GRAY), 6)
		panel.border_width = 1
		panel.border_color = col.call("panel.border_color", Color.BLACK)
		panel.shadow_color = Color(0, 0, 0, 0.3)
		panel.shadow_size = 4
		panel.shadow_offset = 2
		var inner := Rect2(preview.position + Vector2(10, 8), preview.size - Vector2(20, 16))
		draw_style_box(panel, inner)
		var bar_rect := Rect2(inner.position + Vector2(8, 10), Vector2(inner.size.x - 16, 7))
		draw_style_box(UiBox.make(col.call("bars.back_top", Color.BLACK), col.call("bars.back_bottom", Color.BLACK), 3), bar_rect)
		draw_style_box(UiBox.make(col.call("bars.hp", Color.RED), col.call("bars.hp", Color.RED), 3), Rect2(bar_rect.position, Vector2(bar_rect.size.x * 0.7, bar_rect.size.y)))
		var mp_rect := Rect2(bar_rect.position + Vector2(0, 11), bar_rect.size)
		draw_style_box(UiBox.make(col.call("bars.back_top", Color.BLACK), col.call("bars.back_bottom", Color.BLACK), 3), mp_rect)
		draw_style_box(UiBox.make(col.call("bars.mp", Color.BLUE), col.call("bars.mp", Color.BLUE), 3), Rect2(mp_rect.position, Vector2(mp_rect.size.x * 0.45, mp_rect.size.y)))
		var btn := UiBox.make(col.call("button.top", Color.WHITE), col.call("button.bottom", Color.GRAY), 5)
		draw_style_box(btn, Rect2(inner.position + Vector2(inner.size.x - 34, inner.size.y - 13), Vector2(26, 8)))
		var font := get_theme_font("bold_font", "Fonts")
		var text_colour := get_theme_color("font_color", "Label")
		draw_string(font, Vector2(8, 88), str(info["name"]), HORIZONTAL_ALIGNMENT_LEFT, size.x - 16, 13, text_colour)
		draw_string(get_theme_font("font", "Label"), Vector2(8, 104), "Default" if info["id"] == UI.DEFAULT_THEME else ("Built in" if info["file"] == "" or UI.BUILTIN.has(info["id"]) else "Your theme"), HORIZONTAL_ALIGNMENT_LEFT, size.x - 16, 11, get_theme_color("font_color", "MutedLabel"))
		if selected:
			var ring := UiBox.new()
			ring.colors = PackedColorArray([Color(0, 0, 0, 0)])
			ring.radius = 10
			ring.border_width = 2
			ring.border_color = get_theme_color("accent", "Ui")
			draw_style_box(ring, Rect2(Vector2(1, 1), size - Vector2(2, 2)))
