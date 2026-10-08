## Sound window (O): music and sound effect volumes, kept for the next start.
extends PanelContainer

var fx: Node
var _music: HSlider
var _effects: HSlider


func _ready() -> void:
	custom_minimum_size = Vector2(260, 0)
	var margin := MarginContainer.new()
	for side in ["left", "right", "top", "bottom"]:
		margin.add_theme_constant_override("margin_" + side, 12)
	add_child(margin)
	var box := VBoxContainer.new()
	box.add_theme_constant_override("separation", 6)
	margin.add_child(box)
	var header := Label.new()
	header.text = "Sound"
	header.add_theme_font_size_override("font_size", 18)
	box.add_child(header)
	box.add_child(HSeparator.new())
	_music = _slider(box, "Music", fx.music_volume)
	_effects = _slider(box, "Sound effects", fx.effects_volume)


func _slider(box: VBoxContainer, text: String, value: float) -> HSlider:
	var label := Label.new()
	label.text = text
	box.add_child(label)
	var slider := HSlider.new()
	slider.min_value = 0.0
	slider.max_value = 1.0
	slider.step = 0.05
	slider.value = value
	slider.custom_minimum_size.x = 220
	slider.value_changed.connect(func(_v): fx.set_volumes(_music.value, _effects.value))
	box.add_child(slider)
	return slider


func _unhandled_key_input(event: InputEvent) -> void:
	if visible and event is InputEventKey and event.pressed and event.keycode == KEY_ESCAPE:
		visible = false
		get_viewport().set_input_as_handled()
