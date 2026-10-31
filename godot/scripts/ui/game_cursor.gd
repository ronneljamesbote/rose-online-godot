## The mouse cursor: our painted cursors (the interface settings link them), or the client's
## own pixel cursors with the Classic style, switched by what the mouse is
## over like the original client (CGameStateMain::UpdateCheckFrame): crossed blades over a
## monster or an enemy player, a speech bubble over a town NPC, a hand over an item on the
## ground. Over windows it is the normal sword, with the gold hand over items that can be
## dragged and the blue hand while dragging them; title bars and window corners keep their
## move and resize arrows.
## Made by the UI autoload; Online sets `online` while we are in the game.
extends Node

## The cursor states. Each one's picture and click point are in the interface settings
## (`res://data/interface/interface.tres`, our pictures in `res://assets/ours/cursors/`).
const CURSORS := ["default", "attack", "talk", "player", "pick_up", "drag", "dragging", "repair", "camera"]
## Cursor size setting -> how wide the cursor is drawn, in pixels.
const SIZES := {"small": 32, "normal": 40, "large": 48, "huge": 64}
## Godot shows these shapes while something is dragged; they get the blue hand.
const DRAG_SHAPES := [Input.CURSOR_CAN_DROP, Input.CURSOR_FORBIDDEN]

var online: Node
var _images := {}  # state -> [Image as the picture is, click point in its pixels]
var _style := ""  # the Cursor Style _images were loaded for: "new" or "classic"
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


## Reads the nine pictures once (and again when the Cursor Style changes).
func _load() -> bool:
	_style = UI.settings.get("cursor_style", "new")
	_images.clear()
	for state in CURSORS:
		var cursor: Dictionary = RoseData.classic_cursor(state) if _style == "classic" else RoseData.cursor(state)
		if cursor.is_empty():
			# Without the cursor pictures the system cursor stays.
			_images.clear()
			if _shown != "":
				_clear()
			return false
		var image: Image = cursor["image"]
		# See-through pixels take their neighbours' colour, so smoothing leaves no dark rim.
		image.fix_alpha_edges()
		_images[state] = [image, cursor["click"]]
	_scale()
	_shown = ""
	return true


## Draws every cursor at the chosen size: our pictures smoothly scaled, the Classic pixel
## art enlarged in whole steps so it stays sharp.
func _scale() -> void:
	_cursors.clear()
	var size: int = SIZES.get(UI.settings.get("cursor_size", "large"), 48)
	for state in _images:
		var image: Image = _images[state][0].duplicate()
		var width := image.get_width()
		var click: Vector2 = _images[state][1]
		if _style == "classic":
			var times := maxi(1, roundi(float(size) / width))
			image.resize(width * times, image.get_height() * times, Image.INTERPOLATE_NEAREST)
			_cursors[state] = [image, click * times + Vector2(times - 1, times - 1) * 0.5]
		else:
			var scale := float(size) / width
			image.resize(size, maxi(1, roundi(image.get_height() * scale)), Image.INTERPOLATE_LANCZOS)
			_cursors[state] = [image, ((click + Vector2(0.5, 0.5)) * scale).floor()]


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
	if _style != UI.settings.get("cursor_style", "new"):
		_images.clear()  # the other style's pictures load next frame
		_next_try = 0
	elif not _images.is_empty():
		_scale()
		_shown = ""  # sets every shape again at the new size next frame
