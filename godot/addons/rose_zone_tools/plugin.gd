## Zone scene tools for the 3D editor (zones in res://zones/, see assets/README.md):
## - a "Go to" menu (start spot, revive spots, blocks, NPCs, spawns, warp gates, search, a
##   ROSE x, y), and the camera jumps to the zone when one is opened far from it;
## - the point under the mouse in ROSE coordinates (the numbers /tp and the server use);
## - selecting a block (or F with a block selected) frames the block;
## - block outlines and names, and the 10 m ground tiles, drawn on the ground;
## - "Drop to ground" for the selected models and markers;
## - Terrain3D's toolbar without the tools our ROSE terrain shader ignores, unless
##   "Show all terrain tools" is on.
@tool
extends EditorPlugin

const EditorCamera := preload("res://addons/rose_zone_tools/editor_camera.gd")
const SETTINGS := "rose_zone_tools"

## Terrain3D tools that change nothing on a ROSE zone (our shader draws only the ROSE tiles).
const NO_EFFECT_TOOLS := ["PaintAutoshader", "DisableAutoshader", "PaintColor", "RemoveColor",
	"PaintWetness", "RemoveWetness", "PaintNavigableArea", "RemoveNavigableArea", "InstanceMeshes", "RemoveMeshes"]
## Their shortcut keys, which pick them even with the buttons hidden.
const NO_EFFECT_KEYS := [KEY_A, KEY_C, KEY_W, KEY_N, KEY_I]

enum { GO_START, GO_SEARCH, GO_COORDINATES }
enum { VIEW_OUTLINES, VIEW_TILES, VIEW_READOUT, VIEW_REDRAW, VIEW_ALL_TOOLS }

var toolbar: HBoxContainer
var go_menu: MenuButton
var view_menu: MenuButton
var search_dialog: ConfirmationDialog
var search_field: LineEdit
var search_list: ItemList
var coords_dialog: ConfirmationDialog
var coords_field: LineEdit

## Go-to entries of the open zone: {text, group, position, node, distance}.
var entries: Array[Dictionary] = []
var zone_root: Node3D
var mouse_text := ""
var opened_scenes := {}
var _moving_camera := false
var _last_selected_block: Node

var show_outlines := true
var show_tiles := false
var show_readout := true
var show_all_tools := false
var terrain_toolbar: Control
var hidden_tools: Array = []  # [node, index] taken out of Terrain3D's toolbar
var all_tools_button: Button
var all_tools_separator: HSeparator


func _enter_tree() -> void:
	var editor_settings := EditorInterface.get_editor_settings()
	show_outlines = editor_settings.get_project_metadata(SETTINGS, "outlines", true)
	show_tiles = editor_settings.get_project_metadata(SETTINGS, "tiles", false)
	show_readout = editor_settings.get_project_metadata(SETTINGS, "readout", true)
	show_all_tools = editor_settings.get_project_metadata(SETTINGS, "all_tools", false)

	toolbar = HBoxContainer.new()
	toolbar.add_child(VSeparator.new())
	go_menu = MenuButton.new()
	go_menu.text = "Go to"
	go_menu.tooltip_text = "Move the camera to a spot of this zone"
	go_menu.flat = false
	go_menu.about_to_popup.connect(_fill_go_menu)
	toolbar.add_child(go_menu)
	var drop := Button.new()
	drop.text = "Drop to ground"
	drop.tooltip_text = "Put the selected models and markers on the ground under them"
	drop.pressed.connect(_drop_to_ground)
	toolbar.add_child(drop)
	view_menu = MenuButton.new()
	view_menu.text = "Zone view"
	view_menu.flat = false
	var popup := view_menu.get_popup()
	popup.add_check_item("Block outlines and names", VIEW_OUTLINES)
	popup.add_check_item("Ground tiles (10 m)", VIEW_TILES)
	popup.add_check_item("ROSE coordinates under the mouse", VIEW_READOUT)
	popup.add_item("Redraw lines (after terrain edits)", VIEW_REDRAW)
	popup.add_separator()
	popup.add_check_item("Show all terrain tools", VIEW_ALL_TOOLS)
	popup.id_pressed.connect(_on_view_menu)
	toolbar.add_child(view_menu)
	add_control_to_container(CONTAINER_SPATIAL_EDITOR_MENU, toolbar)
	_update_view_menu()

	_make_dialogs()
	set_input_event_forwarding_always_enabled()
	set_force_draw_over_forwarding_enabled()
	scene_changed.connect(_on_scene_changed)
	EditorInterface.get_selection().selection_changed.connect(_on_selection_changed)
	get_undo_redo().version_changed.connect(_refresh_block_warnings)
	_on_scene_changed(EditorInterface.get_edited_scene_root())


func _exit_tree() -> void:
	_show_terrain_tools(true)
	for node in [all_tools_button, all_tools_separator]:
		if node != null and is_instance_valid(node):
			node.free()
	remove_control_from_container(CONTAINER_SPATIAL_EDITOR_MENU, toolbar)
	toolbar.free()
	search_dialog.free()
	coords_dialog.free()


# ---------------------------------------------------------------------------------------
# The open zone

static func is_zone(root: Node) -> bool:
	return root != null and (root.is_class("RoseZoneScene") or root.has_meta("zone_id"))


func zone_id() -> int:
	if zone_root == null:
		return 0
	return zone_root.get("zone_id") if zone_root.is_class("RoseZoneScene") else zone_root.get_meta("zone_id", 0)


## Block x, y of a zone child: a RoseBlock, or a Block_X_Y node of an unmerged zone.
static func block_xy(node: Node) -> Vector2i:
	if node.is_class("RoseBlock"):
		return Vector2i(node.get("block_x"), node.get("block_y"))
	var parts := String(node.name).split("_")
	if parts.size() == 3 and parts[0] == "Block" and parts[1].is_valid_int() and parts[2].is_valid_int():
		return Vector2i(int(parts[1]), int(parts[2]))
	return Vector2i(-1, -1)


static func block_center(xy: Vector2i) -> Vector3:
	return Vector3(xy.x * 160.0 + 80.0, 0.0, (xy.y - 65) * 160.0 + 80.0)


func terrain() -> Node3D:
	if zone_root == null:
		return null
	var node := zone_root.get_node_or_null("Terrain")
	return node if node != null and node.is_class("Terrain3D") else null


func ground_height(x: float, z: float, otherwise: float = 0.0) -> float:
	var ground := terrain()
	if ground == null:
		return otherwise
	var height: float = ground.data.get_height(Vector3(x, 0.0, z))
	return otherwise if is_nan(height) or is_inf(height) else height


func _on_scene_changed(root: Node) -> void:
	zone_root = root if is_zone(root) else null
	toolbar.visible = zone_root != null
	entries.clear()
	mouse_text = ""
	_apply_overlay()
	_show_terrain_tools(show_all_tools or zone_root == null)
	if zone_root == null or opened_scenes.has(root.scene_file_path):
		return
	opened_scenes[root.scene_file_path] = true
	# A zone holds thousands of nodes: show it as its list of blocks, unless some were folded
	# or unfolded by hand before (Godot remembers that).
	var blocks := root.get_children().filter(func(n: Node) -> bool: return block_xy(n).x >= 0)
	if not blocks.any(func(n: Node) -> bool: return n.is_displayed_folded()):
		for block in blocks:
			block.set_display_folded(true)
	# Godot puts back the camera it remembers for this scene a moment after opening it.
	get_tree().create_timer(0.3).timeout.connect(_jump_to_start.bind(root))


## On opening a zone with the camera far from it (at Godot's origin, 5 km away), go to its start.
func _jump_to_start(root: Node) -> void:
	if zone_root != root or not is_instance_valid(root):
		return
	var start := _start_spot()
	var camera := EditorCamera.camera()
	if start != null and camera != null and camera.global_position.distance_to(start.global_position) > 2000.0:
		_go(start.global_position, 80.0, null)


func _start_spot() -> Node3D:
	for node in zone_root.find_children("*", "RoseZoneEvent", true, false):
		if node.get("event_name") == "start":
			return node
	return null


# ---------------------------------------------------------------------------------------
# Go to

func _go(target: Vector3, distance: float, select: Node) -> void:
	_moving_camera = true
	EditorCamera.look_at(target, distance, select)
	_last_selected_block = select if select != null and block_xy(select).x >= 0 else null
	_moving_camera = false


func _go_to_entry(entry: Dictionary) -> void:
	var node: Node = entry.get("node")
	var target: Vector3 = entry.position
	if node is Node3D and node.is_inside_tree() and block_xy(node).x < 0:
		target = node.global_position
	_go(target, entry.distance, node if node != null and node.is_inside_tree() else null)


## The names of NPCs and spawned monsters, from the zone's export file's comments
## (overrides/zones/<id>-<name>.toml): {node path: name}.
func _export_names() -> Dictionary:
	var names := {}
	var folder := zone_root.scene_file_path.get_base_dir().get_file()
	var path := ProjectSettings.globalize_path("res://").path_join("../overrides/zones/%s.toml" % folder.replace("_", "-"))
	var file := FileAccess.open(path, FileAccess.READ)
	if file == null:
		return names
	var node_path := ""
	while not file.eof_reached():
		var line := file.get_line()
		if line.begins_with("[[") and line.contains("#"):
			node_path = line.get_slice("#", 1).strip_edges()
		elif node_path != "" and (line.begins_with("npc =") or line.begins_with("basic =")) and line.contains("#"):
			names[node_path] = line.get_slice("#", 1).strip_edges()
	return names


func _collect_entries() -> void:
	entries.clear()
	if zone_root == null:
		return
	var names := _export_names()
	var add := func(group: String, text: String, position: Vector3, node: Node, distance: float) -> void:
		entries.append({"group": group, "text": text, "position": position, "node": node, "distance": distance})
	for event in zone_root.find_children("*", "RoseZoneEvent", true, false):
		var name: String = event.get("event_name")
		var group := "Start" if name == "start" else ("Revive spots" if name == "restore" else "Named spots")
		add.call(group, "%s (%s)" % [name, event.name], event.global_position, event, 60.0)
	var blocks: Array[Node] = []
	for child in zone_root.get_children():
		if block_xy(child).x >= 0:
			blocks.append(child)
	blocks.sort_custom(func(a, b): return [block_xy(a).y, block_xy(a).x] < [block_xy(b).y, block_xy(b).x])
	for block in blocks:
		var xy := block_xy(block)
		var center := block_center(xy)
		center.y = ground_height(center.x, center.z)
		add.call("Blocks", "Block %d_%d" % [xy.x, xy.y], center, block, 230.0)
	for npc in zone_root.find_children("*", "RoseNpcSpot", true, false):
		var path := str(zone_root.get_path_to(npc))
		add.call("NPCs", "%s  (%s)" % [names.get(path, "NPC %d" % npc.get("npc_id")), _block_of(npc)], npc.global_position, npc, 30.0)
	for spawn in zone_root.find_children("*", "RoseSpawnSpot", true, false):
		var path := str(zone_root.get_path_to(spawn))
		var text: String = names.get(path, "Spawn %s" % spawn.name)
		if text.length() > 70:
			text = text.left(67) + "..."
		add.call("Spawns", "%s  (%s)" % [text, _block_of(spawn)], spawn.global_position, spawn, 60.0)
	for object in zone_root.find_children("*", "RoseZoneObject", true, false):
		if object.get("list") == "warp":
			add.call("Warp gates", "Warp gate %d  (%s)" % [object.get("warp_id"), _block_of(object)], object.global_position, object, 40.0)


func _block_of(node: Node) -> String:
	var at := node.get_parent()
	while at != null and at != zone_root:
		if block_xy(at).x >= 0:
			var xy := block_xy(at)
			return "block %d_%d" % [xy.x, xy.y]
		at = at.get_parent()
	return "zone"


func _fill_go_menu() -> void:
	_collect_entries()
	var popup := go_menu.get_popup()
	for child in popup.get_children(true):
		if child is PopupMenu:
			child.queue_free()
	popup.clear()
	if popup.id_pressed.is_connected(_on_go_menu):
		popup.id_pressed.disconnect(_on_go_menu)
	popup.id_pressed.connect(_on_go_menu)
	popup.add_item("Start spot", GO_START)
	popup.set_item_disabled(-1, _start_spot() == null)
	var groups := ["Revive spots", "Named spots", "Blocks", "NPCs", "Spawns", "Warp gates"]
	for group in groups:
		var sub := PopupMenu.new()
		sub.max_size = Vector2i(0, 600)
		var count := 0
		for i in entries.size():
			if entries[i].group == group:
				sub.add_item(entries[i].text, 100 + i)
				count += 1
		if count == 0:
			sub.free()
			continue
		sub.id_pressed.connect(_on_go_menu)
		popup.add_submenu_node_item("%s (%d)" % [group, count], sub)
	popup.add_separator()
	popup.add_item("Search...", GO_SEARCH)
	popup.add_item("Coordinates (ROSE x, y)...", GO_COORDINATES)


func _on_go_menu(id: int) -> void:
	match id:
		GO_START:
			var start := _start_spot()
			if start != null:
				_go(start.global_position, 80.0, start)
		GO_SEARCH:
			_collect_entries()
			search_field.text = ""
			_fill_search("")
			search_dialog.popup_centered(Vector2i(560, 480))
			search_field.grab_focus()
		GO_COORDINATES:
			coords_dialog.popup_centered(Vector2i(420, 0))
			coords_field.grab_focus()
			coords_field.select_all()
		_:
			if id >= 100 and id - 100 < entries.size():
				_go_to_entry(entries[id - 100])


func _make_dialogs() -> void:
	search_dialog = ConfirmationDialog.new()
	search_dialog.title = "Go to"
	search_dialog.ok_button_text = "Go"
	var box := VBoxContainer.new()
	search_field = LineEdit.new()
	search_field.placeholder_text = "NPC, monster, block, spot or node name"
	search_field.clear_button_enabled = true
	search_field.text_changed.connect(_fill_search)
	search_field.text_submitted.connect(func(_text): _search_go())
	box.add_child(search_field)
	search_list = ItemList.new()
	search_list.size_flags_vertical = Control.SIZE_EXPAND_FILL
	search_list.custom_minimum_size = Vector2(0, 360)
	search_list.item_activated.connect(func(_index): _search_go())
	box.add_child(search_list)
	search_dialog.add_child(box)
	search_dialog.confirmed.connect(_search_go)
	EditorInterface.get_base_control().add_child(search_dialog)

	coords_dialog = ConfirmationDialog.new()
	coords_dialog.title = "Go to ROSE coordinates"
	coords_dialog.ok_button_text = "Go"
	var coords_box := VBoxContainer.new()
	var hint := Label.new()
	hint.text = "x and y in metres, as /tp and the readout show them\n(\"5300 5300\", \"5300, 5300\" or \"/tp 20 5300 5300\")"
	coords_box.add_child(hint)
	coords_field = LineEdit.new()
	coords_field.text_submitted.connect(func(_text):
		coords_dialog.hide()
		_coords_go())
	coords_box.add_child(coords_field)
	coords_dialog.add_child(coords_box)
	coords_dialog.confirmed.connect(_coords_go)
	EditorInterface.get_base_control().add_child(coords_dialog)


func _fill_search(text: String) -> void:
	search_list.clear()
	var words := Array(text.to_lower().split(" ", false))
	for i in entries.size():
		var label: String = "%s: %s" % [entries[i].group, entries[i].text]
		var haystack := (label + " " + str(entries[i].node.name if entries[i].node else "")).to_lower()
		if words.all(func(w: String) -> bool: return haystack.contains(w)):
			search_list.add_item(label)
			search_list.set_item_metadata(-1, i)
	if search_list.item_count > 0:
		search_list.select(0)


func _search_go() -> void:
	var selected := search_list.get_selected_items()
	if selected.is_empty():
		return
	search_dialog.hide()
	_go_to_entry(entries[search_list.get_item_metadata(selected[0])])


func _coords_go() -> void:
	var numbers: Array[float] = []
	for word in coords_field.text.replace(",", " ").split(" ", false):
		if word.is_valid_float():
			numbers.append(float(word))
	if numbers.size() < 2:
		return
	# ROSE x east, y north; Godot's z points south.
	var x := numbers[-2]
	var z := -numbers[-1]
	_go(Vector3(x, ground_height(x, z), z), 80.0, null)


# ---------------------------------------------------------------------------------------
# Blocks: selecting one or F frames it

func _on_selection_changed() -> void:
	if _moving_camera or zone_root == null:
		return
	# Terrain3D makes its toolbar after this plugin starts.
	_show_terrain_tools(show_all_tools)
	var selected := EditorInterface.get_selection().get_top_selected_nodes()
	if selected.size() != 1 or block_xy(selected[0]).x < 0 or selected[0].get_parent() != zone_root:
		_last_selected_block = null
		return
	if selected[0] != _last_selected_block:
		_frame_block(selected[0])


## A block's warnings (things outside it) change when something in it moves.
func _refresh_block_warnings() -> void:
	if zone_root == null:
		return
	for node in EditorInterface.get_selection().get_selected_nodes():
		var at: Node = node
		while at != null and at != zone_root:
			if at.is_class("RoseBlock"):
				at.update_configuration_warnings()
				break
			at = at.get_parent()


func _frame_block(block: Node) -> void:
	var center := block_center(block_xy(block))
	center.y = ground_height(center.x, center.z)
	_go(center, 230.0, block)


func _forward_3d_gui_input(camera: Camera3D, event: InputEvent) -> int:
	if zone_root == null:
		return AFTER_GUI_INPUT_PASS
	if event is InputEventMouseMotion and show_readout:
		_update_readout(camera, event.position)
	elif event is InputEventKey and event.pressed and not event.echo and not (event.ctrl_pressed or event.alt_pressed or event.meta_pressed or event.shift_pressed):
		var selected := EditorInterface.get_selection().get_top_selected_nodes()
		if event.keycode == KEY_F and selected.size() == 1 and block_xy(selected[0]).x >= 0 and selected[0].get_parent() == zone_root:
			_frame_block(selected[0])
			return AFTER_GUI_INPUT_STOP
		if not show_all_tools and event.keycode in NO_EFFECT_KEYS and selected.size() == 1 and selected[0].is_class("Terrain3D"):
			return AFTER_GUI_INPUT_STOP
	return AFTER_GUI_INPUT_PASS


# ---------------------------------------------------------------------------------------
# ROSE coordinates under the mouse

func _update_readout(camera: Camera3D, mouse: Vector2) -> void:
	var container := camera.get_parent().get_parent() as SubViewportContainer
	if container != null and container.stretch_shrink == 2:
		mouse /= 2.0
	var from := camera.project_ray_origin(mouse)
	var direction := camera.project_ray_normal(mouse)
	var point := Vector3(INF, INF, INF)
	var ground := terrain()
	if ground != null:
		point = ground.get_intersection(from, direction, false)
	if not point.is_finite() or point.z > 3.0e38:
		# No ground under the mouse: where the ray meets height 0.
		if absf(direction.y) < 0.0001:
			mouse_text = ""
			update_overlays()
			return
		point = from + direction * (-from.y / direction.y)
	var x := point.x
	var y := -point.z
	var block := Vector2i(floori(point.x / 160.0), floori(point.z / 160.0) + 65)
	mouse_text = "x %.1f  y %.1f  height %.1f    block %d_%d    /tp %d %d %d" % [x, y, point.y, block.x, block.y, zone_id(), roundi(x), roundi(y)]
	update_overlays()


func _forward_3d_force_draw_over_viewport(overlay: Control) -> void:
	if zone_root == null or not show_readout or mouse_text == "":
		return
	var font := overlay.get_theme_default_font()
	var size := overlay.get_theme_default_font_size()
	var text_size := font.get_string_size(mouse_text, HORIZONTAL_ALIGNMENT_LEFT, -1, size)
	var at := Vector2(12, overlay.size.y - 14)
	overlay.draw_rect(Rect2(at + Vector2(-6, -text_size.y), text_size + Vector2(12, 8)), Color(0, 0, 0, 0.55))
	overlay.draw_string(font, at, mouse_text, HORIZONTAL_ALIGNMENT_LEFT, -1, size, Color(1, 0.9, 0.5))


# ---------------------------------------------------------------------------------------
# Block outlines, ground tiles

func _on_view_menu(id: int) -> void:
	var editor_settings := EditorInterface.get_editor_settings()
	match id:
		VIEW_OUTLINES:
			show_outlines = not show_outlines
			editor_settings.set_project_metadata(SETTINGS, "outlines", show_outlines)
			_apply_overlay()
		VIEW_TILES:
			show_tiles = not show_tiles
			editor_settings.set_project_metadata(SETTINGS, "tiles", show_tiles)
			_apply_overlay()
		VIEW_READOUT:
			show_readout = not show_readout
			editor_settings.set_project_metadata(SETTINGS, "readout", show_readout)
			mouse_text = ""
			update_overlays()
		VIEW_REDRAW:
			_apply_overlay(true)
		VIEW_ALL_TOOLS:
			_set_all_tools(not show_all_tools)
	_update_view_menu()


func _update_view_menu() -> void:
	var popup := view_menu.get_popup()
	popup.set_item_checked(popup.get_item_index(VIEW_OUTLINES), show_outlines)
	popup.set_item_checked(popup.get_item_index(VIEW_TILES), show_tiles)
	popup.set_item_checked(popup.get_item_index(VIEW_READOUT), show_readout)
	popup.set_item_checked(popup.get_item_index(VIEW_ALL_TOOLS), show_all_tools)


func _apply_overlay(redraw := false) -> void:
	if zone_root == null:
		return
	for block in zone_root.get_children():
		if block.is_class("RoseBlock"):
			block.set_overlay(show_outlines, show_tiles)
			if redraw:
				block.redraw_overlay()


# ---------------------------------------------------------------------------------------
# Drop to ground

func _drop_to_ground() -> void:
	if terrain() == null:
		return
	var undo := get_undo_redo()
	var moved := 0
	undo.create_action("Drop to ground")
	for node in EditorInterface.get_selection().get_top_selected_nodes():
		# Not the zone, its blocks, the groups in them, the terrain or water planes.
		if not node is Node3D or node == zone_root or node.get_parent() == zone_root or block_xy(node.get_parent()).x >= 0:
			continue
		if node.is_class("Terrain3D") or node.is_class("RoseWater"):
			continue
		var at: Vector3 = node.global_position
		var height := ground_height(at.x, at.z, NAN)
		if is_nan(height):
			continue
		undo.add_do_property(node, "global_position", Vector3(at.x, height, at.z))
		undo.add_undo_property(node, "global_position", at)
		moved += 1
	undo.commit_action()
	if moved == 0:
		push_warning("Drop to ground: select models or markers (NPCs, spawns, sounds, effects) that stand over the ground.")


# ---------------------------------------------------------------------------------------
# Terrain3D toolbar

func _set_all_tools(show: bool) -> void:
	show_all_tools = show
	EditorInterface.get_editor_settings().set_project_metadata(SETTINGS, "all_tools", show)
	if all_tools_button:
		all_tools_button.set_pressed_no_signal(show)
	_show_terrain_tools(show or zone_root == null)
	_update_view_menu()


func _find_terrain_toolbar() -> Control:
	if terrain_toolbar != null and is_instance_valid(terrain_toolbar):
		return terrain_toolbar
	for node in EditorInterface.get_base_control().find_children("*", "VFlowContainer", true, false):
		var script: Script = node.get_script()
		if script != null and script.resource_path.ends_with("terrain_3d/src/toolbar.gd"):
			terrain_toolbar = node
			all_tools_button = Button.new()
			all_tools_button.flat = true
			all_tools_button.toggle_mode = true
			all_tools_button.icon = EditorInterface.get_base_control().get_theme_icon("GuiVisibilityVisible", "EditorIcons")
			all_tools_button.tooltip_text = "Show all terrain tools\nPaint color, Wetness, Autoshader, Navigation and Instancer have no effect on ROSE zones, so they are hidden."
			all_tools_button.toggled.connect(_set_all_tools)
			all_tools_separator = HSeparator.new()
			terrain_toolbar.add_child(all_tools_separator)
			terrain_toolbar.add_child(all_tools_button)
			all_tools_button.set_pressed_no_signal(show_all_tools)
			return terrain_toolbar
	return null


func _show_terrain_tools(show: bool) -> void:
	var bar := _find_terrain_toolbar()
	if bar == null:
		return
	if show:
		hidden_tools.sort_custom(func(a, b): return a[1] < b[1])
		for item in hidden_tools:
			bar.add_child(item[0])
			bar.move_child(item[0], item[1])
		hidden_tools.clear()
		return
	if not hidden_tools.is_empty():
		return
	var children := bar.get_children()
	var take: Array[Node] = []
	for i in children.size():
		var child := children[i]
		if child.name in NO_EFFECT_TOOLS:
			take.append(child)
			# The separator before Paint Color closes a group that is all hidden.
			if child.name == "PaintColor" and i > 0 and children[i - 1] is HSeparator:
				take.append(children[i - 1])
			if child is BaseButton and child.button_pressed:
				var raise: BaseButton = bar.get_button("Raise")
				if raise:
					raise.set_pressed(true)
	for child in take:
		hidden_tools.append([child, child.get_index()])
	for item in hidden_tools:
		bar.remove_child(item[0])
