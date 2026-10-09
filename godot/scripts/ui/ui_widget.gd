## A HUD widget (player frame, minimap, hotbar ...) the player can drag: a small grip
## appears at its corner while the mouse is over it, unless the interface is locked.
class_name UiWidget
extends UiMovable

var content: Control
var _handle: Control
var _hover_until := 0


static func wrap(what: Control, widget_id: String, anchor: Vector2, offset: Vector2) -> UiWidget:
	var w := UiWidget.new()
	w.id = widget_id
	w.default_anchor = anchor
	w.default_offset = offset
	w.content = what
	w.name = widget_id.to_pascal_case() + "Widget"
	return w


func _ready() -> void:
	if content:
		if content.get_parent():
			content.get_parent().remove_child(content)
		content.set_anchors_preset(Control.PRESET_TOP_LEFT)
		content.position = Vector2.ZERO
		content.set_meta("ui_fill", true)
		add_child(content)
		visible = content.visible
		content.visibility_changed.connect(func(): visible = content.visible)
	_handle = _Handle.new()
	_handle.visible = false
	_handle.mouse_default_cursor_shape = Control.CURSOR_MOVE
	_handle.tooltip_text = "Drag to move"
	_handle.gui_input.connect(_on_handle_input)
	add_child(_handle)
	super._ready()


func _process(_delta: float) -> void:
	if not is_visible_in_tree():
		return
	var mouse := get_local_mouse_position()
	var over := Rect2(Vector2(-14, -14), size + Vector2(28, 28)).has_point(mouse)
	if over:
		_hover_until = Time.get_ticks_msec() + 600
	var show: bool = (_dragging or Time.get_ticks_msec() < _hover_until) and not UI.settings.get("locked", false)
	if _handle.visible != show:
		_handle.visible = show
		if show:
			move_to_front()


func _sort_extra() -> void:
	if _handle:
		_handle.size = Vector2(18, 18)
		_handle.position = Vector2(-9, -9)


func _on_handle_input(event: InputEvent) -> void:
	if event is InputEventMouseButton and event.button_index == MOUSE_BUTTON_LEFT and event.pressed:
		start_drag()
		_handle.accept_event()


## A round handle with a four-way arrow mark.
class _Handle extends Control:
	func _draw() -> void:
		var c := size * 0.5
		var accent := get_theme_color("accent", "Ui")
		draw_circle(c, 8.5, Color(0, 0, 0, 0.35), true, -1.0, true)
		draw_circle(c, 7.5, accent, true, -1.0, true)
		var dark := Color(0, 0, 0, 0.7)
		for dir in [Vector2.UP, Vector2.DOWN, Vector2.LEFT, Vector2.RIGHT]:
			var tip: Vector2 = c + dir * 5.0
			var side: Vector2 = Vector2(-dir.y, dir.x) * 2.0
			draw_colored_polygon(PackedVector2Array([tip, c + dir * 2.5 + side, c + dir * 2.5 - side]), dark)
		draw_circle(c, 1.2, dark, true, -1.0, true)
