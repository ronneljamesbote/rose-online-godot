## The loading screen shown while a zone loads (signing in and walking through a warp
## gate): the client's LOADING.DDS picture with the zone's name.
extends CanvasLayer

const FADE := 0.35

var _root: Control
var _title: Label
var _bar: ProgressBar
var _tween: Tween


func _ready() -> void:
	layer = 100
	_root = Control.new()
	_root.set_anchors_preset(Control.PRESET_FULL_RECT)
	_root.mouse_filter = Control.MOUSE_FILTER_STOP
	add_child(_root)
	var background := ColorRect.new()
	background.color = Color(0.04, 0.03, 0.06)
	background.set_anchors_preset(Control.PRESET_FULL_RECT)
	_root.add_child(background)
	var picture := TextureRect.new()
	picture.texture = RoseData.texture("3DDATA/CONTROL/RES/LOADING.DDS")
	picture.set_anchors_preset(Control.PRESET_FULL_RECT)
	picture.expand_mode = TextureRect.EXPAND_IGNORE_SIZE
	picture.stretch_mode = TextureRect.STRETCH_KEEP_ASPECT_COVERED
	_root.add_child(picture)

	var bottom := VBoxContainer.new()
	bottom.set_anchors_and_offsets_preset(Control.PRESET_CENTER_BOTTOM)
	bottom.grow_horizontal = Control.GROW_DIRECTION_BOTH
	bottom.grow_vertical = Control.GROW_DIRECTION_BEGIN
	bottom.offset_bottom = -48
	bottom.custom_minimum_size.x = 520
	bottom.add_theme_constant_override("separation", 10)
	_root.add_child(bottom)
	_title = Label.new()
	_title.horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER
	_title.theme_type_variation = "HeaderLabel"
	_title.add_theme_font_size_override("font_size", 30)
	_title.add_theme_color_override("font_color", Color.WHITE)
	_title.add_theme_color_override("font_outline_color", Color(0, 0, 0, 0.8))
	_title.add_theme_constant_override("outline_size", 8)
	bottom.add_child(_title)
	_bar = ProgressBar.new()
	_bar.show_percentage = false
	_bar.custom_minimum_size.y = 10
	_bar.max_value = 1.0
	bottom.add_child(_bar)
	_root.visible = false


## Show the screen for this zone. Wait for the next frames before loading, so it is drawn.
func show_zone(zone_name: String) -> void:
	if _tween:
		_tween.kill()
	_title.text = "Loading %s..." % zone_name if zone_name != "" else "Loading..."
	_root.modulate.a = 1.0
	_root.visible = true
	_bar.value = 0.15
	_tween = create_tween()
	# The load itself blocks, so the bar only moves before and after it.
	_tween.tween_property(_bar, "value", 0.6, 0.25)


func hide_screen() -> void:
	if not _root.visible:
		return
	if _tween:
		_tween.kill()
	_tween = create_tween()
	_tween.tween_property(_bar, "value", 1.0, 0.15)
	_tween.tween_property(_root, "modulate:a", 0.0, FADE)
	_tween.tween_callback(func(): _root.visible = false)


func is_showing() -> bool:
	return _root.visible
