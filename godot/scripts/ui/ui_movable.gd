## Something on the interface the player can drag around: windows and HUD widgets.
## Remembers where it was put (relative to the nearest screen side, so it stays there when
## the game window is resized), snaps to the screen edges and stays on screen.
class_name UiMovable
extends Container

const SNAP := 14.0  # pixels from a screen edge that snap to it
const EDGE := 8.0  # gap kept to the screen edge when snapped

## Name in the saved layout.
var id := ""
## Where it goes until the player moves it: anchor (0 to 1 on each axis; the same point of
## this and of the screen line up) plus an offset in pixels.
var default_anchor := Vector2.ZERO
var default_offset := Vector2(EDGE, EDGE)
var default_scale := 1.0
var min_scale := 0.75
var max_scale := 1.5
var _anchor := Vector2.ZERO
var _offset := Vector2.ZERO
var _dragging := false
var _grab := Vector2.ZERO
var _placed := false


func _init() -> void:
	add_to_group("ui_movable")
	mouse_filter = Control.MOUSE_FILTER_IGNORE


func _ready() -> void:
	minimum_size_changed.connect(_fit_size)
	item_rect_changed.connect(_keep_on_screen)
	if get_parent_control():
		get_parent_control().resized.connect(_place_from_anchor)
	else:
		get_viewport().size_changed.connect(_place_from_anchor)
	_load_place()
	_fit_size()


func _fit_size() -> void:
	var wanted := get_combined_minimum_size()
	if size != wanted:
		size = wanted
	if _placed:
		_place_from_anchor.call_deferred()


func _area() -> Vector2:
	var parent := get_parent_control()
	return parent.size if parent else get_viewport_rect().size


func _load_place() -> void:
	var place := UI.layout_get(id) if id != "" else {}
	_anchor = place.get("anchor", default_anchor)
	_offset = place.get("offset", default_offset)
	scale = Vector2.ONE * clampf(float(place.get("scale", default_scale)), min_scale, max_scale)
	_placed = true
	_after_load(place)
	_place_from_anchor.call_deferred()


## Subclasses read their extra saved values here.
func _after_load(_place: Dictionary) -> void:
	pass


## Back to the default place (Options: Reset layout).
func reset_place() -> void:
	_load_place()


func _place_from_anchor() -> void:
	var scaled := size * scale
	position = ((_area() - scaled) * _anchor + _offset).round()
	_keep_on_screen()


func _keep_on_screen() -> void:
	var area := _area()
	var scaled := size * scale
	var p := position
	p.x = clampf(p.x, 0.0, maxf(0.0, area.x - scaled.x))
	p.y = clampf(p.y, 0.0, maxf(0.0, area.y - scaled.y))
	if p != position:
		position = p


## Starts dragging from a press at this point (in our parent's coordinates).
func start_drag() -> bool:
	if UI.settings.get("locked", false):
		return false
	_dragging = true
	_grab = _parent_mouse() - position
	move_to_front()
	return true


func _parent_mouse() -> Vector2:
	var parent := get_parent_control()
	return parent.get_local_mouse_position() if parent else get_viewport().get_mouse_position()


func _input(event: InputEvent) -> void:
	if not _dragging:
		return
	if event is InputEventMouseMotion:
		var area := _area()
		var scaled := size * scale
		var p := _parent_mouse() - _grab
		# Snap to the screen edges.
		if absf(p.x - EDGE) < SNAP:
			p.x = EDGE
		elif absf(area.x - scaled.x - EDGE - p.x) < SNAP:
			p.x = area.x - scaled.x - EDGE
		if absf(p.y - EDGE) < SNAP:
			p.y = EDGE
		elif absf(area.y - scaled.y - EDGE - p.y) < SNAP:
			p.y = area.y - scaled.y - EDGE
		position = p.round()
		_keep_on_screen()
		get_viewport().set_input_as_handled()
	elif event is InputEventMouseButton and event.button_index == MOUSE_BUTTON_LEFT and not event.pressed:
		_dragging = false
		save_place()
		get_viewport().set_input_as_handled()


## Remembers the current place, measured from the nearest side of the screen.
func save_place() -> void:
	var area := _area()
	var scaled := size * scale
	var free := area - scaled
	var centre := position + scaled * 0.5
	_anchor = Vector2(_side(centre.x, area.x), _side(centre.y, area.y))
	_offset = position - free * _anchor
	if id == "":
		return
	var place := UI.layout_get(id)
	place["anchor"] = _anchor
	place["offset"] = _offset
	place["scale"] = scale.x
	_before_save(place)
	UI.layout_set(id, place)


func _before_save(_place: Dictionary) -> void:
	pass


static func _side(at: float, length: float) -> float:
	if at < length / 3.0:
		return 0.0
	if at > length * 2.0 / 3.0:
		return 1.0
	return 0.5


func _notification(what: int) -> void:
	if what == NOTIFICATION_SORT_CHILDREN:
		for child in get_children():
			if child is Control and not child.top_level and child.has_meta("ui_fill"):
				fit_child_in_rect(child, Rect2(Vector2.ZERO, size))
		_sort_extra()


func _sort_extra() -> void:
	pass


func _get_minimum_size() -> Vector2:
	var wanted := Vector2.ZERO
	for child in get_children():
		if child is Control and child.visible and child.has_meta("ui_fill"):
			wanted = wanted.max(child.get_combined_minimum_size())
	return wanted
