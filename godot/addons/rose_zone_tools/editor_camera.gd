## Moves the 3D editor's camera. Godot has no API for it, so this does what the editor's own
## View menu does: Focus Selection on a helper node at the spot, then the mouse wheel to set
## how far away the camera is.
extends RefCounted

## Godot's mouse wheel zooms by 8 % per step (ZOOM_FREELOOK_MULTIPLIER in node_3d_editor_plugin.cpp).
const WHEEL_STEP := 0.08


## Puts the camera of the first 3D view `distance` metres from `target` (in world space),
## keeping its angle. `select` is selected afterwards (nothing when null).
static func look_at(target: Vector3, distance: float, select: Node = null) -> void:
	var root := EditorInterface.get_edited_scene_root()
	var view := _view(0)
	if root == null or view.is_empty():
		return
	EditorInterface.set_main_screen_editor("3D")
	var helper := Node3D.new()
	helper.name = "RoseGoToHelper"
	root.add_child(helper, false, Node.INTERNAL_MODE_BACK)
	helper.global_position = target
	var selection := EditorInterface.get_selection()
	selection.clear()
	selection.add_node(helper)
	var menu: PopupMenu = view.menu
	menu.id_pressed.emit(view.focus_id)
	selection.clear()
	root.remove_child(helper)
	helper.free()
	if select != null:
		selection.add_node(select)
	_set_distance(view, distance)


## The editor's 3D view number `index`: {surface, menu, focus_id, camera}, or {} when the
## editor's layout is not what this expects.
static func _view(index: int) -> Dictionary:
	var sub_viewport := EditorInterface.get_editor_viewport_3d(index)
	if sub_viewport == null:
		return {}
	var view_control := sub_viewport.get_parent().get_parent()  # Node3DEditorViewport
	var surface: Control = null
	for child in view_control.get_children():
		if child.get_class() == "Control":
			surface = child
	if surface == null:
		return {}
	for button in surface.find_children("*", "MenuButton", true, false):
		var popup: PopupMenu = button.get_popup()
		for i in popup.item_count:
			if _is_focus_selection(popup, i):
				return {"surface": surface, "menu": popup, "focus_id": popup.get_item_id(i), "camera": sub_viewport.get_camera_3d()}
	return {}


static func _is_focus_selection(popup: PopupMenu, i: int) -> bool:
	if popup.get_item_text(i) == "Focus Selection":
		return true
	var shortcut := popup.get_item_shortcut(i)
	if shortcut == null:
		return false
	for event in shortcut.events:
		if event is InputEventKey and event.keycode == KEY_F and not (event.ctrl_pressed or event.alt_pressed or event.shift_pressed or event.meta_pressed):
			return true
	return false


## The camera orbits its focus point at a distance only the mouse wheel changes: zoom all the
## way in (a known distance), then out by the right amount in one wheel event.
static func _set_distance(view: Dictionary, distance: float) -> void:
	var camera: Camera3D = view.camera
	var nearest := maxf(camera.near * 4.0, 0.01)
	var wheel := func(button: MouseButton, factor: float) -> void:
		var event := InputEventMouseButton.new()
		event.button_index = button
		event.pressed = true
		event.factor = factor
		view.surface.gui_input.emit(event)
	wheel.call(MOUSE_BUTTON_WHEEL_UP, 1.0e9)
	wheel.call(MOUSE_BUTTON_WHEEL_DOWN, (distance / nearest - 1.0) / WHEEL_STEP)


## The camera of the 3D view the mouse is over (or the first one).
static func camera() -> Camera3D:
	var view := _view(0)
	return view.camera if not view.is_empty() else null
