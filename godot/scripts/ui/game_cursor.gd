## The mouse cursor: ROSE's own cursors (the interface settings link them), switched by what the mouse is
## over like the original client (CGameStateMain::UpdateCheckFrame): crossed blades over a
## monster or an enemy player, a speech bubble over a town NPC, a hand over an item on the
## ground. Over windows it is the normal sword, with the gold hand over items that can be
## dragged and the blue hand while dragging them; title bars and window corners keep their
## move and resize arrows.
## Made by the UI autoload; Online sets `online` while we are in the game.
extends Node

## The cursor states. Each one's picture and click point are in the interface settings
## (`res://data/interface/interface.tres`, the client's own cursor files by default).
const CURSORS := ["default", "attack", "talk", "player", "pick_up", "drag", "dragging", "repair", "camera"]
## Cursor size setting -> how many times the original 32 pixels. Whole steps keep the pixel
## art sharp.
const SIZES := {"normal": 1, "large": 2, "huge": 3}
## Godot shows these shapes while something is dragged; they get the blue hand.
const DRAG_SHAPES := [Input.CURSOR_CAN_DROP, Input.CURSOR_FORBIDDEN]

var online: Node
var _images := {}  # state -> [Image at normal size, click point]
var _cursors := {}  # state -> [Image at the chosen size, click point]
var _shown := ""  # the state on the arrow shape now, "" when the system cursor is in use
var _next_try := 0


func _ready() -> void:
	UI.settings_changed.connect(_on_settings)


func _process(_delta: float) -> void:
	if not UI.settings.get("game_cursor", true):
		return
	if _images.is_empty():
		# The client data opens after the interface starts; try until it is there.
		if Time.get_ticks_msec() < _next_try:
			return
		_next_try = Time.get_ticks_msec() + 1000
		if not _load():
			return
	var state := state_at(get_viewport().get_mouse_position())
	if state != _shown:
		_show(state)


## Which cursor fits this screen position now: one of the CURSORS.
func state_at(mouse: Vector2) -> String:
	var viewport := get_viewport()
	if not is_instance_valid(online):
		online = null
	if online and online.is_repairing():
		return "repair"
	if viewport.gui_is_dragging():
		return "dragging"
	var hovered := viewport.gui_get_hovered_control()
	if hovered != null:
		return "drag" if hovered.has_method("is_draggable") and hovered.is_draggable() else "default"
	if online == null or online.me == null:
		return "default"
	if Input.is_mouse_button_pressed(MOUSE_BUTTON_RIGHT):
		return "camera"
	var camera := viewport.get_camera_3d()
	if camera == null:
		return "default"
	match online.under_mouse(camera, mouse).get("kind", ""):
		"monster", "enemy":
			return "attack"
		"npc":
			return "talk"
		"player":
			return "player"
		"item":
			return "pick_up"
	return "default"


func _load() -> bool:
	for state in CURSORS:
		var cursor: Dictionary = RoseData.cursor(state)
		if cursor.is_empty():
			# Without the cursor pictures the system cursor stays.
			_images.clear()
			return false
		_images[state] = [cursor["image"], cursor["click"]]
	_scale()
	return true


## Enlarges every cursor to the chosen size, keeping the pixels sharp.
func _scale() -> void:
	_cursors.clear()
	var times: int = SIZES.get(UI.settings.get("cursor_size", "large"), 2)
	for state in _images:
		var image: Image = _images[state][0].duplicate()
		image.resize(image.get_width() * times, image.get_height() * times, Image.INTERPOLATE_NEAREST)
		_cursors[state] = [image, _images[state][1] * times + Vector2(times - 1, times - 1) * 0.5]


func _show(state: String) -> void:
	if _shown == "":
		Input.set_custom_mouse_cursor(_cursors["drag"][0], Input.CURSOR_POINTING_HAND, _cursors["drag"][1])
		for shape in DRAG_SHAPES:
			Input.set_custom_mouse_cursor(_cursors["dragging"][0], shape, _cursors["dragging"][1])
	Input.set_custom_mouse_cursor(_cursors[state][0], Input.CURSOR_ARROW, _cursors[state][1])
	_shown = state
	_refresh()


## Back to the system cursor.
func _clear() -> void:
	for shape in DRAG_SHAPES + [Input.CURSOR_ARROW, Input.CURSOR_POINTING_HAND]:
		Input.set_custom_mouse_cursor(null, shape)
	_shown = ""
	_refresh()


## A new picture for the shape on screen only shows once the shape changes (on Linux, at
## least), so switch away and back.
func _refresh() -> void:
	var shape := DisplayServer.cursor_get_shape()
	DisplayServer.cursor_set_shape(DisplayServer.CURSOR_HELP if shape != DisplayServer.CURSOR_HELP else DisplayServer.CURSOR_ARROW)
	DisplayServer.cursor_set_shape(shape)


func _on_settings() -> void:
	if not UI.settings.get("game_cursor", true):
		if _shown != "":
			_clear()
		return
	if not _images.is_empty():
		_scale()
		_shown = ""  # sets every shape again at the new size next frame
