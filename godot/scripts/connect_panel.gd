## Start screen: server address, character name and weapon, or play offline.
## The last values are kept in user://settings.cfg.
extends CanvasLayer

signal connect_requested(uri: String, name_text: String, use_bow: bool)
signal offline_requested

const SETTINGS := "user://settings.cfg"

var server_edit: LineEdit
var name_edit: LineEdit
var weapon_button: OptionButton


func _ready() -> void:
	var settings := ConfigFile.new()
	settings.load(SETTINGS)

	var center := CenterContainer.new()
	center.set_anchors_preset(Control.PRESET_FULL_RECT)
	add_child(center)
	var panel := PanelContainer.new()
	panel.custom_minimum_size = Vector2(360, 0)
	center.add_child(panel)
	var margin := MarginContainer.new()
	for side in ["left", "right", "top", "bottom"]:
		margin.add_theme_constant_override("margin_" + side, 16)
	panel.add_child(margin)
	var box := VBoxContainer.new()
	box.add_theme_constant_override("separation", 8)
	margin.add_child(box)

	var title := Label.new()
	title.text = "ROSE"
	title.horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER
	title.add_theme_font_size_override("font_size", 28)
	box.add_child(title)

	server_edit = _field(box, "Server", settings.get_value("net", "server", "ws://127.0.0.1:3000"))
	name_edit = _field(box, "Name", settings.get_value("net", "name", ""))
	name_edit.max_length = 20
	name_edit.placeholder_text = "1-20 characters"

	var row := HBoxContainer.new()
	box.add_child(row)
	var weapon_label := Label.new()
	weapon_label.text = "Weapon"
	weapon_label.custom_minimum_size.x = 70
	row.add_child(weapon_label)
	weapon_button = OptionButton.new()
	weapon_button.add_item("Short Sword")
	weapon_button.add_item("Short Bow")
	weapon_button.selected = 1 if settings.get_value("net", "bow", false) else 0
	weapon_button.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	row.add_child(weapon_button)

	var buttons := HBoxContainer.new()
	buttons.alignment = BoxContainer.ALIGNMENT_CENTER
	buttons.add_theme_constant_override("separation", 12)
	box.add_child(buttons)
	var connect_button := Button.new()
	connect_button.text = "Connect"
	connect_button.pressed.connect(_on_connect)
	buttons.add_child(connect_button)
	var offline_button := Button.new()
	offline_button.text = "Play offline"
	offline_button.pressed.connect(func(): offline_requested.emit(); queue_free())
	buttons.add_child(offline_button)
	name_edit.text_submitted.connect(func(_t): _on_connect())
	name_edit.grab_focus()


func _field(box: VBoxContainer, caption: String, value: String) -> LineEdit:
	var row := HBoxContainer.new()
	box.add_child(row)
	var label := Label.new()
	label.text = caption
	label.custom_minimum_size.x = 70
	row.add_child(label)
	var edit := LineEdit.new()
	edit.text = value
	edit.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	row.add_child(edit)
	return edit


func _on_connect() -> void:
	var name_text := name_edit.text.strip_edges()
	var use_bow := weapon_button.selected == 1
	var settings := ConfigFile.new()
	settings.set_value("net", "server", server_edit.text.strip_edges())
	settings.set_value("net", "name", name_text)
	settings.set_value("net", "bow", use_bow)
	settings.save(SETTINGS)
	connect_requested.emit(server_edit.text.strip_edges(), name_text, use_bow)
	queue_free()
