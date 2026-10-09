## The interface's root: covers the screen and applies the Interface size setting by
## scaling everything inside it.
class_name UiRoot
extends Control


func _ready() -> void:
	mouse_filter = Control.MOUSE_FILTER_IGNORE
	get_viewport().size_changed.connect(_fit)
	UI.settings_changed.connect(_fit)
	_fit()


func _fit() -> void:
	var s := clampf(float(UI.settings.get("scale", 1.0)), 0.75, 1.5)
	var screen := get_viewport().get_visible_rect().size
	scale = Vector2(s, s)
	position = Vector2.ZERO
	size = screen / s
