## The Graphs screen: an NPC dialog (RoseDialog) or the quest steps of one folder of
## res://data/quest_triggers/ (RoseQuestTrigger) as boxes and wires (see data/README.md).
## - Dialog: one box per menu, its lines as rows. Each line's port is wired to its next_menu;
##   an answer that ends the talk goes to "End". Amber rows and wires only show when their
##   show_if passes; the chips say what they check ("If …") and do ("Do …").
## - Quest steps: one box per step, its conditions and rewards as rows. "If it fails, try"
##   is wired to next_trigger, a "Run step" reward to the step it names; steps of other
##   folders are grey boxes (double-click one to open its folder).
## - Click a row or a chip to edit it in the Inspector. Drag a wire to change where a line
##   leads (drop it on empty space: nowhere). Right-click a row or the background for more.
## - A box you move keeps its place (graph_position in its resource); the rest are placed
##   by the graph. Every change can be undone (Ctrl+Z); Ctrl+S saves.
## Only the dialog or folder shown is loaded, a few dozen boxes a frame; nothing runs while
## the screen is idle.
@tool
extends VBoxContainer

const SETTINGS := "rose_data_tools"
const DIALOGS_DIR := "res://data/dialogs"
const STEPS_DIR := "res://data/quest_triggers"
## Where "Used by" looks for a step (text search, nothing is loaded).
const USED_BY_DIRS := ["res://data/dialogs", "res://data/quest_triggers", "res://data/ai", "res://data/monsters", "res://data/npcs", "res://zones"]
## Boxes made (or step files loaded) per frame while a graph opens.
const BOXES_PER_FRAME := 24
## Files read per frame by "Used by".
const FILES_PER_FRAME := 60
const BOX_WIDTH := 400.0
const COLUMN_GAP := 140.0
const BOX_GAP := 36.0
const ROW_HEIGHT := 31.0
const BOX_CHROME := 70.0
const TEXT_MAX := 64
const CHIP_MAX := 40
## Chips shown per list of a line; the rest are one "+N" chip.
const CHIPS_PER_LIST := 2
const COMPARE := ["==", "!=", "<", "<=", ">", ">=", "=", "+=", "-="]
const SKIP := ["resource_local_to_scene", "resource_name", "resource_path", "resource_scene_unique_id", "script"]

const COLOR_NPC := Color(0.55, 0.75, 1.0)
const COLOR_ANSWER := Color(0.92, 0.92, 0.92)
const COLOR_CONDITIONAL := Color(1.0, 0.72, 0.25)
const COLOR_STEP := Color(0.6, 0.9, 0.6)
const COLOR_FAIL := Color(1.0, 0.5, 0.45)
const COLOR_STUB := Color(0.5, 0.5, 0.5)

enum Mode { NONE, DIALOG, STEPS }
enum { MENU_EDIT_TEXT, MENU_TOGGLE_KIND, MENU_UP, MENU_DOWN, MENU_ADD_NPC, MENU_ADD_ANSWER, MENU_DELETE_LINE, MENU_INSPECT, MENU_ADD_BOX, MENU_DELETE_BOX, MENU_USED_BY, MENU_OPEN_FOLDER, MENU_LAYOUT }

var plugin: EditorPlugin
## Resources changed here and not saved yet, by path.
var dirty := {}

var mode := Mode.NONE
## The dialog shown, or the steps of the folder shown ({res, path}, in file order).
var dialog: Resource
var dialog_path := ""
var folder := ""
var steps: Array[Dictionary] = []

## Every step file by its file name (no folder), listed the first time steps are shown.
var _step_files := {}
## The graph's boxes by what they show: a menu (by number) or a step (by resource).
var _boxes: Array[GraphNode] = []
var _end_box: GraphNode
var _stubs := {}
## Resource -> box number, for the lines, cases and blocks inside each box: an Inspector
## change to one of them redraws that box.
var _box_of := {}
var _auto_position := {}
var _build_queue: Array = []
var _build_started := 0
var _after_build: Callable
var _find_hits: Array[GraphNode] = []
var _find_at := -1
var _context_box := -1
var _context_line := -1
var _context_at := Vector2.ZERO
var _used_by_files: Array[String] = []
var _used_by_needles: PackedStringArray = []
var _used_by_hits: Array[String] = []
var _used_by_at := 0
var _used_by_started := 0
var _refresh_pending := {}
var _chip_styles := {}
var _block_properties := {}
var _markup := RegEx.create_from_string("\\{/?[a-z]+(=\\d+)?\\}")
## Set while Find selects boxes, so selecting them doesn't fill the Inspector.
var _finding := false

var kind_button: OptionButton
var item_button: OptionButton
var find_field: LineEdit
var add_button: Button
var used_by_button: Button
var save_button: Button
var status_label: Label
var graph: GraphEdit
var context_menu: PopupMenu
var text_popup: PopupPanel
var text_edit: TextEdit
var name_dialog: ConfirmationDialog
var name_field: LineEdit
var used_by_dialog: AcceptDialog
var used_by_list: ItemList


func _init() -> void:
	name = "GraphsScreen"
	size_flags_vertical = SIZE_EXPAND_FILL
	var bar := HBoxContainer.new()
	add_child(bar)
	kind_button = OptionButton.new()
	kind_button.add_item("NPC dialogs")
	kind_button.add_item("Quest steps")
	kind_button.tooltip_text = "Show an NPC dialog (res://data/dialogs/) or the steps of a quest step folder (res://data/quest_triggers/)"
	kind_button.item_selected.connect(func(_index: int) -> void: _fill_items())
	bar.add_child(kind_button)
	item_button = OptionButton.new()
	item_button.fit_to_longest_item = false
	item_button.custom_minimum_size.x = 320
	item_button.tooltip_text = "The dialog or step folder to show"
	item_button.item_selected.connect(_on_item_selected)
	bar.add_child(item_button)
	find_field = LineEdit.new()
	find_field.placeholder_text = "Find in boxes (Enter: next)"
	find_field.tooltip_text = "Selects the boxes whose words, chips or names contain this; Enter goes to the next one"
	find_field.clear_button_enabled = true
	find_field.size_flags_horizontal = SIZE_EXPAND_FILL
	find_field.text_changed.connect(func(_text: String) -> void: _find(false))
	find_field.text_submitted.connect(func(_text: String) -> void: _find(true))
	bar.add_child(find_field)
	add_button = Button.new()
	add_button.pressed.connect(func() -> void: _add_box(_view_center()))
	bar.add_child(add_button)
	used_by_button = Button.new()
	used_by_button.text = "Used by"
	used_by_button.tooltip_text = "Which dialogs, steps, AIs, monsters and zones run the selected step (searches the files' text)"
	used_by_button.pressed.connect(_used_by_selected)
	bar.add_child(used_by_button)
	var layout := Button.new()
	layout.text = "Auto layout"
	layout.tooltip_text = "Puts the selected boxes (or every box) back where the graph places them, forgetting where they were moved"
	layout.pressed.connect(_reset_positions)
	bar.add_child(layout)
	save_button = Button.new()
	save_button.tooltip_text = "Save the resources changed here (Ctrl+S does too)"
	save_button.pressed.connect(save_changes)
	bar.add_child(save_button)
	status_label = Label.new()
	bar.add_child(status_label)

	graph = GraphEdit.new()
	graph.size_flags_vertical = SIZE_EXPAND_FILL
	graph.minimap_enabled = true
	graph.right_disconnects = true
	graph.show_arrange_button = false
	graph.connection_lines_thickness = 2.0
	graph.connection_request.connect(_on_connection_request)
	graph.disconnection_request.connect(_on_disconnection_request)
	graph.connection_to_empty.connect(func(from: StringName, port: int, _at: Vector2) -> void: _set_target(from, port, -1, null))
	graph.delete_nodes_request.connect(_on_delete_request)
	graph.end_node_move.connect(_on_end_node_move)
	graph.node_selected.connect(_on_node_selected)
	graph.popup_request.connect(_on_popup_request)
	add_child(graph)

	var help := Label.new()
	help.text = "Click a row or chip to edit it in the Inspector · double-click a row to change its words · drag a wire to change where it leads · right-click for more · Ctrl+Z undoes, Ctrl+S saves"
	help.add_theme_color_override("font_color", Color(1, 1, 1, 0.55))
	help.text_overrun_behavior = TextServer.OVERRUN_TRIM_ELLIPSIS
	add_child(help)

	context_menu = PopupMenu.new()
	context_menu.id_pressed.connect(_on_context_menu)
	add_child(context_menu)
	_make_text_popup()
	_make_name_dialog()
	_make_used_by_dialog()
	set_process(false)
	_update_bar()


func _ready() -> void:
	EditorInterface.get_inspector().property_edited.connect(_on_inspector_edited)


## Called when the screen is shown: lists the dialogs and folders the first time and opens
## the last graph shown.
func open() -> void:
	if item_button.item_count > 0 or mode != Mode.NONE:
		return
	var last: String = EditorInterface.get_editor_settings().get_project_metadata(SETTINGS, "graph", "")
	kind_button.select(1 if last.begins_with(STEPS_DIR) else 0)
	_fill_items()
	if not last.is_empty() and (FileAccess.file_exists(last) or DirAccess.dir_exists_absolute(last)):
		_open_path(last)


## Shows a dialog, or the folder of a step with that step selected.
func open_resource(res: Resource) -> void:
	if res == null or res.resource_path.is_empty():
		return
	if res.is_class("RoseDialog"):
		_open_path(res.resource_path)
	elif res.is_class("RoseQuestTrigger"):
		_open_path(res.resource_path.get_base_dir(), res)


func _open_path(path: String, select: Resource = null) -> void:
	var steps_folder := path.begins_with(STEPS_DIR)
	if kind_button.selected != (1 if steps_folder else 0) or item_button.item_count == 0:
		kind_button.select(1 if steps_folder else 0)
		_fill_items()
	for i in item_button.item_count:
		if item_button.get_item_metadata(i) == path:
			item_button.select(i)
	if steps_folder:
		_open_steps(path, select)
	else:
		_open_dialog(path)


func _fill_items() -> void:
	item_button.clear()
	if kind_button.selected == 0:
		for file in DirAccess.get_files_at(DIALOGS_DIR):
			if file.ends_with(".tres"):
				item_button.add_item(file.get_basename())
				item_button.set_item_metadata(item_button.item_count - 1, DIALOGS_DIR.path_join(file))
	else:
		for sub in DirAccess.get_directories_at(STEPS_DIR):
			item_button.add_item(sub)
			item_button.set_item_metadata(item_button.item_count - 1, STEPS_DIR.path_join(sub))
	item_button.select(-1)
	item_button.text = "Pick a dialog" if kind_button.selected == 0 else "Pick a step folder"


func _on_item_selected(index: int) -> void:
	_open_path(item_button.get_item_metadata(index))


# ---------------------------------------------------------------------------------------
# Opening and building

func _clear() -> void:
	_build_queue.clear()
	graph.clear_connections()
	for child in graph.get_children():
		if child is GraphNode:
			graph.remove_child(child)
			child.queue_free()
	_boxes.clear()
	_stubs.clear()
	_end_box = null
	_box_of.clear()
	_auto_position.clear()
	_find_hits.clear()
	_find_at = -1


func _open_dialog(path: String) -> void:
	# The same instance the Inspector edits, so a change in either shows in both.
	var res := load(path) as Resource
	if res == null or not res.is_class("RoseDialog"):
		_status("%s is not a RoseDialog" % path)
		return
	_clear()
	mode = Mode.DIALOG
	dialog = res
	dialog_path = path
	folder = ""
	steps.clear()
	EditorInterface.get_editor_settings().set_project_metadata(SETTINGS, "graph", path)
	_build_started = Time.get_ticks_msec()
	_layout_dialog()
	_end_box = _make_end_box()
	for m in _menus().size():
		_build_queue.append(m)
	_after_build = func() -> void:
		_connect_all()
		_show_start(_boxes[0] if not _boxes.is_empty() else null)
		_status("%d menus, %d lines · %d ms" % [_menus().size(), _line_count(), Time.get_ticks_msec() - _build_started])
	_update_bar()
	set_process(true)


func _open_steps(path: String, select: Resource = null) -> void:
	if _step_files.is_empty():
		for sub in DirAccess.get_directories_at(STEPS_DIR):
			for file in DirAccess.get_files_at(STEPS_DIR.path_join(sub)):
				if file.ends_with(".tres"):
					_step_files[file.get_basename()] = STEPS_DIR.path_join(sub).path_join(file)
	_clear()
	mode = Mode.STEPS
	dialog = null
	dialog_path = ""
	folder = path
	steps.clear()
	EditorInterface.get_editor_settings().set_project_metadata(SETTINGS, "graph", path)
	_build_started = Time.get_ticks_msec()
	var files := Array(DirAccess.get_files_at(path)).filter(func(f: String) -> bool: return f.ends_with(".tres"))
	# Loading is the slow part, so the files are loaded a few a frame and the boxes made
	# once all are in (the layout needs every wire).
	for file in files:
		_build_queue.append(path.path_join(file))
	_after_build = func() -> void:
		_layout_steps()
		for s in steps.size():
			_boxes.append(_make_step_box(s))
		_connect_all()
		var shown: GraphNode = null
		for s in steps.size():
			if steps[s].res == select:
				shown = _boxes[s]
		if shown != null:
			_show_box(shown)
		else:
			_show_start(_boxes[0] if not _boxes.is_empty() else null)
		_status("%d steps · %d ms" % [steps.size(), Time.get_ticks_msec() - _build_started])
	_update_bar()
	set_process(true)


## Makes a few boxes (or loads a few steps) a frame, so a big graph never stalls the editor.
func _process(_delta: float) -> void:
	if not _used_by_files.is_empty():
		_used_by_step()
		return
	var n := 0
	while not _build_queue.is_empty() and n < BOXES_PER_FRAME:
		var next: Variant = _build_queue.pop_front()
		if next is int:
			_boxes.append(_make_menu_box(next))
		else:
			var res := load(next) as Resource
			if res != null and res.is_class("RoseQuestTrigger"):
				steps.append({"res": res, "path": next})
		n += 1
	if not _build_queue.is_empty():
		_status("Opening… %d left" % _build_queue.size())
		return
	set_process(false)
	if _after_build.is_valid():
		var done := _after_build
		_after_build = Callable()
		done.call()


func _menus() -> Array:
	return dialog.get("menus") if dialog != null else []


func _lines(m: int) -> Array:
	var menu: Resource = _menus()[m]
	return menu.get("lines") if menu != null else []


func _line_count() -> int:
	var n := 0
	for m in _menus().size():
		n += _lines(m).size()
	return n


## Places the menus in columns by how many answers away from menu 0 they are (menus no
## line leads to start their own columns below).
func _layout_dialog() -> void:
	var menus := _menus()
	var depth := {}
	var order: Array[int] = []
	for root in menus.size():
		if depth.has(root):
			continue
		depth[root] = 0
		var queue: Array[int] = [root]
		while not queue.is_empty():
			var m: int = queue.pop_front()
			order.append(m)
			for line in _lines(m):
				var to: int = line.get("next_menu") if line != null else -1
				if to >= 0 and to < menus.size() and not depth.has(to):
					depth[to] = depth[m] + 1
					queue.append(to)
	var heights := {}
	for m in menus.size():
		var rows := 1
		for line in _lines(m):
			rows += 2 if line != null and not (line.get("show_if").is_empty() and line.get("on_choose").is_empty()) else 1
		heights[m] = BOX_CHROME + ROW_HEIGHT * rows
	_auto_position = _columns(order, depth, heights)
	var right := 0.0
	for at in _auto_position.values():
		right = maxf(right, at.x)
	_auto_position["end"] = Vector2(right + BOX_WIDTH + COLUMN_GAP, 0)


## Positions for boxes in `order`, each in the column of its depth, stacked from the top.
func _columns(order: Array, depth: Dictionary, heights: Dictionary) -> Dictionary:
	var y_of := {}
	var out := {}
	for key in order:
		var column: int = depth[key]
		var y: float = y_of.get(column, 0.0)
		out[key] = Vector2(column * (BOX_WIDTH + COLUMN_GAP), y)
		y_of[column] = y + heights[key] + BOX_GAP
	return out


func _layout_steps() -> void:
	var index := {}
	for s in steps.size():
		index[steps[s].res] = s
	var targets := {}
	var has_parent := {}
	for s in steps.size():
		targets[s] = []
		for target in _step_targets(steps[s].res):
			var t := _step_in_folder(target)
			if t >= 0 and t != s:
				targets[s].append(t)
				has_parent[t] = true
	var depth := {}
	var order: Array = []
	var roots: Array = range(steps.size()).filter(func(s: int) -> bool: return not has_parent.has(s))
	roots.append_array(range(steps.size()))
	for root in roots:
		if depth.has(root):
			continue
		depth[root] = 0
		var queue: Array = [root]
		while not queue.is_empty():
			var s: int = queue.pop_front()
			order.append(s)
			for t in targets[s]:
				if not depth.has(t):
					depth[t] = depth[s] + 1
					queue.append(t)
	var heights := {}
	for s in steps.size():
		var res: Resource = steps[s].res
		heights[s] = BOX_CHROME + ROW_HEIGHT * (res.get("conditions").size() + res.get("rewards").size() + 2)
	_auto_position = _columns(order, depth, heights)


## What a step leads to: its next_trigger (a resource) and the steps its rewards run (names).
func _step_targets(res: Resource) -> Array:
	var out := []
	var next: Resource = res.get("next_trigger")
	if next != null:
		out.append(next)
	for block in res.get("rewards"):
		if block != null and ("trigger" in block) and not str(block.get("trigger")).is_empty():
			out.append(str(block.get("trigger")))
	return out


## A step of the folder shown by resource or name, or -1.
func _step_in_folder(target: Variant) -> int:
	for s in steps.size():
		var res: Resource = steps[s].res
		if (target is Resource and res == target) or (target is String and str(res.get("name")) == target):
			return s
	return -1


# ---------------------------------------------------------------------------------------
# Boxes

func _new_box(node_name: String, title: String) -> GraphNode:
	var box := GraphNode.new()
	box.name = node_name
	box.title = title
	box.custom_minimum_size.x = BOX_WIDTH
	box.resizable = false
	graph.add_child(box)
	return box


## The position a box shows at: where it was moved to, or where the graph places it.
func _box_position(res: Resource, key: Variant) -> Vector2:
	var saved: Vector2 = res.get("graph_position") if res != null else Vector2.ZERO
	if saved != Vector2.ZERO:
		return saved
	return _auto_position.get(key, Vector2.ZERO)


func _make_menu_box(m: int) -> GraphNode:
	var menu: Resource = _menus()[m]
	var box := _new_box("m%d" % m, "Menu %d%s" % [m, "  · where the talk starts" if m == 0 else ""])
	box.position_offset = _box_position(menu, m)
	box.set_meta("res", menu)
	_fill_menu_box(box, m)
	return box


func _fill_menu_box(box: GraphNode, m: int) -> void:
	_empty_box(box)
	var menu: Resource = _menus()[m]
	_box_of[menu] = m
	var find := PackedStringArray(["menu %d" % m])
	var header := Label.new()
	var lines := _lines(m)
	header.text = "1 line" if lines.size() == 1 else "%d lines" % lines.size()
	header.add_theme_color_override("font_color", Color(1, 1, 1, 0.5))
	box.add_child(header)
	box.set_slot(0, true, 0, COLOR_ANSWER, false, 0, Color.WHITE)
	for l in lines.size():
		var line: Resource = lines[l]
		var row := VBoxContainer.new()
		row.add_theme_constant_override("separation", 0)
		box.add_child(row)
		if line == null:
			var empty := Label.new()
			empty.text = "(empty place)"
			row.add_child(empty)
			box.set_slot(l + 1, false, 0, Color.WHITE, true, 0, Color(1, 1, 1, 0.3))
			continue
		_box_of[line] = m
		var answer: bool = str(line.get("kind")) == "Answer"
		var conditional: bool = not line.get("show_if").is_empty()
		var text := Button.new()
		text.flat = true
		text.alignment = HORIZONTAL_ALIGNMENT_LEFT
		text.text_overrun_behavior = TextServer.OVERRUN_TRIM_ELLIPSIS
		text.size_flags_horizontal = SIZE_EXPAND_FILL
		text.custom_minimum_size.x = 120
		var words := _plain(str(line.get("text")))
		text.text = ("You: " if answer else "NPC: ") + _short(words, TEXT_MAX)
		text.tooltip_text = ("Your answer" if answer else "The NPC says") + ":\n" + str(line.get("text")) + "\n\nClick: edit in the Inspector · double-click: change the words · right-click: more"
		text.add_theme_color_override("font_color", COLOR_ANSWER if answer else COLOR_NPC)
		text.pressed.connect(_inspect.bind(line))
		text.gui_input.connect(_on_line_input.bind(m, l))
		row.add_child(text)
		find.append(words)
		# The chips go on their own row under the words, so neither is cut short.
		var chips := HFlowContainer.new()
		_add_case_chips(chips, line, "show_if", "If ", find, m)
		_add_case_chips(chips, line, "on_choose", "Do ", find, m)
		if chips.get_child_count() > 0:
			var indent := MarginContainer.new()
			indent.add_theme_constant_override("margin_left", 18)
			indent.add_child(chips)
			row.add_child(indent)
		else:
			chips.free()
		var color := COLOR_CONDITIONAL if conditional else (COLOR_ANSWER if answer else COLOR_NPC)
		box.set_slot(l + 1, false, 0, Color.WHITE, true, 0, color)
	var footer := HBoxContainer.new()
	for answer in [false, true]:
		var add := Button.new()
		add.text = "+ Answer" if answer else "+ NPC line"
		add.flat = true
		add.tooltip_text = "Add a line at the bottom of this menu"
		add.add_theme_color_override("font_color", Color(1, 1, 1, 0.55))
		add.pressed.connect(_add_line.bind(m, lines.size(), answer))
		footer.add_child(add)
	box.add_child(footer)
	box.set_meta("find", " ".join(find).to_lower())


## One chip per case of a line's show_if or on_choose (the first few), and "+N" for more.
func _add_case_chips(row: Container, line: Resource, property: String, prefix: String, find: PackedStringArray, m: int) -> void:
	var cases: Array = line.get(property)
	for c in cases.size():
		var case: Resource = cases[c]
		if case == null:
			continue
		_box_of[case] = m
		for block in case.get("conditions") + case.get("actions"):
			if block != null:
				_box_of[block] = m
		var summary := _case_text(case, property == "on_choose")
		find.append(summary)
		if c >= CHIPS_PER_LIST:
			continue
		if c == CHIPS_PER_LIST - 1 and cases.size() > CHIPS_PER_LIST:
			row.add_child(_chip("+%d" % (cases.size() - c), "%d more cases in %s: edit the line in the Inspector" % [cases.size() - c, property], property, line))
			continue
		row.add_child(_chip(prefix + _short(summary, CHIP_MAX), "%s case %d:\n%s\n\nClick: edit it in the Inspector" % [property, c, summary], property, case))


func _chip(text: String, tooltip: String, property: String, res: Resource) -> Button:
	var chip := Button.new()
	chip.text = text
	chip.tooltip_text = tooltip
	chip.add_theme_font_size_override("font_size", 12)
	var style := _chip_style(property)
	for state in ["normal", "hover", "pressed", "focus"]:
		chip.add_theme_stylebox_override(state, style)
	chip.pressed.connect(_inspect.bind(res))
	return chip


func _chip_style(property: String) -> StyleBoxFlat:
	if not _chip_styles.has(property):
		var style := StyleBoxFlat.new()
		style.bg_color = Color(0.45, 0.32, 0.1, 0.9) if property in ["show_if", "conditions"] else Color(0.15, 0.38, 0.22, 0.9)
		style.set_corner_radius_all(8)
		style.content_margin_left = 6
		style.content_margin_right = 6
		style.content_margin_top = 1
		style.content_margin_bottom = 1
		_chip_styles[property] = style
	return _chip_styles[property]


func _make_end_box() -> GraphNode:
	var box := _new_box("end", "End")
	box.custom_minimum_size.x = 160
	box.position_offset = _auto_position.get("end", Vector2.ZERO)
	var label := Label.new()
	label.text = "The talk ends"
	box.add_child(label)
	box.set_slot(0, true, 0, COLOR_ANSWER, false, 0, Color.WHITE)
	return box


func _make_step_box(s: int) -> GraphNode:
	var res: Resource = steps[s].res
	var box := _new_box("s%d" % s, str(res.get("name")))
	box.position_offset = _box_position(res, s)
	box.set_meta("res", res)
	_fill_step_box(box, s)
	return box


func _fill_step_box(box: GraphNode, s: int) -> void:
	_empty_box(box)
	var res: Resource = steps[s].res
	_box_of[res] = s
	box.title = str(res.get("name"))
	var find := PackedStringArray([str(res.get("name"))])
	var ports: Array = []
	var header := Label.new()
	header.text = steps[s].path.get_file() + ("" if FileAccess.file_exists(steps[s].path) else "  (new, saved with Ctrl+S)")
	header.add_theme_color_override("font_color", Color(1, 1, 1, 0.5))
	box.add_child(header)
	box.set_slot(0, true, 0, COLOR_STEP, false, 0, Color.WHITE)
	var slot := 1
	for property in ["conditions", "rewards"]:
		var blocks: Array = res.get(property)
		for block in blocks:
			if block == null:
				continue
			_box_of[block] = s
			var summary := _block_text(block)
			find.append(summary)
			var row := HBoxContainer.new()
			row.add_child(_chip(("If " if property == "conditions" else "Then ") + _short(summary, TEXT_MAX), "%s:\n%s\n\nClick: edit it in the Inspector" % [property, summary], property, block))
			box.add_child(row)
			var runs: bool = property == "rewards" and "trigger" in block
			box.set_slot(slot, false, 0, Color.WHITE, runs, 0, COLOR_STEP)
			if runs:
				ports.append(block)
			slot += 1
	var fail := Label.new()
	var next: Resource = res.get("next_trigger")
	fail.text = "If it fails, try →" if next != null else "If it fails, try (none)"
	fail.horizontal_alignment = HORIZONTAL_ALIGNMENT_RIGHT
	fail.tooltip_text = "next_trigger: the step tried when a condition fails. Drag the wire to another step to change it"
	fail.add_theme_color_override("font_color", COLOR_FAIL)
	box.add_child(fail)
	box.set_slot(slot, false, 0, Color.WHITE, true, 0, COLOR_FAIL)
	ports.append(null)
	box.set_meta("ports", ports)
	box.set_meta("find", " ".join(find).to_lower())


## A grey box for a step of another folder (or one that isn't there).
func _stub(target: Variant) -> GraphNode:
	var key: String = target.resource_path if target is Resource else str(target)
	if _stubs.has(key):
		return _stubs[key]
	var step_name: String = str(target.get("name")) if target is Resource else str(target)
	var path: String = target.resource_path if target is Resource else _step_files.get(_stem(step_name), "")
	var box := _new_box("x%d" % _stubs.size(), step_name)
	box.self_modulate = COLOR_STUB
	var label := Label.new()
	label.text = "in %s · double-click to open" % path.get_base_dir().get_file() if not path.is_empty() else "no such step"
	label.add_theme_color_override("font_color", Color(1, 1, 1, 0.5))
	box.add_child(label)
	box.set_slot(0, true, 0, COLOR_STUB, false, 0, Color.WHITE)
	box.set_meta("path", path)
	box.set_meta("find", step_name.to_lower())
	box.gui_input.connect(func(event: InputEvent) -> void:
		if event is InputEventMouseButton and event.double_click and not path.is_empty():
			_open_path.call_deferred(path.get_base_dir(), load(path)))
	var right := 0.0
	for at in _auto_position.values():
		right = maxf(right, at.x)
	box.position_offset = Vector2(right + BOX_WIDTH + COLUMN_GAP, _stubs.size() * (BOX_CHROME + BOX_GAP))
	_stubs[key] = box
	return box


func _empty_box(box: GraphNode) -> void:
	box.clear_all_slots()
	for child in box.get_children():
		box.remove_child(child)
		child.queue_free()
	box.size = Vector2.ZERO


## Draws every wire again (cheap: a few hundred connections).
func _connect_all() -> void:
	graph.clear_connections()
	if mode == Mode.DIALOG:
		var count := _menus().size()
		for m in mini(count, _boxes.size()):
			var lines := _lines(m)
			for l in lines.size():
				var line: Resource = lines[l]
				if line == null:
					continue
				var to: int = line.get("next_menu")
				if to >= 0 and to < _boxes.size():
					graph.connect_node(_boxes[m].name, l, _boxes[to].name, 0)
				elif to < 0 and str(line.get("kind")) == "Answer" and _end_box != null:
					graph.connect_node(_boxes[m].name, l, _end_box.name, 0)
	elif mode == Mode.STEPS:
		for s in _boxes.size():
			var res: Resource = steps[s].res
			var ports: Array = _boxes[s].get_meta("ports", [])
			for p in ports.size():
				var target: Variant = ports[p].get("trigger") if ports[p] != null else res.get("next_trigger")
				if target == null or (target is String and target.is_empty()):
					continue
				var t := _step_in_folder(target)
				var to: GraphNode = _boxes[t] if t >= 0 else _stub(target)
				graph.connect_node(_boxes[s].name, p, to.name, 0)


## Redraws the boxes changed (by number), or everything when the menus or steps changed.
func _refresh(boxes: Array) -> void:
	if mode == Mode.DIALOG:
		if _menus().size() != _boxes.size():
			_open_dialog(dialog_path)
			return
		for m in boxes:
			if m >= 0 and m < _boxes.size():
				_fill_menu_box(_boxes[m], m)
				_boxes[m].position_offset = _box_position(_menus()[m], m)
	elif mode == Mode.STEPS:
		if steps.size() != _boxes.size():
			_rebuild_steps()
			return
		for s in boxes:
			if s >= 0 and s < _boxes.size():
				_fill_step_box(_boxes[s], s)
				_boxes[s].position_offset = _box_position(steps[s].res, s)
	_connect_all()
	_find(false)
	_update_bar()


func _rebuild_steps() -> void:
	var offset := graph.scroll_offset
	_clear()
	_layout_steps()
	for s in steps.size():
		_boxes.append(_make_step_box(s))
	_connect_all()
	graph.scroll_offset = offset
	_update_bar()


# ---------------------------------------------------------------------------------------
# Text for rows and chips

## A dialog text without its markup ({br}, {b}, {fc=N}) on one line.
func _plain(text: String) -> String:
	var out := text.replace("{br}", " ").replace("\n", " ")
	out = _markup.sub(out, "", true)
	return " ".join(out.split(" ", false))


static func _short(text: String, length: int) -> String:
	return text if text.length() <= length else text.substr(0, length - 1) + "…"


## A case as a line of text: its conditions, then what it does or shows.
func _case_text(case: Resource, choose: bool) -> String:
	var parts := PackedStringArray()
	for block in case.get("conditions"):
		if block != null:
			parts.append(_block_text(block))
	var actions := PackedStringArray()
	for block in case.get("actions"):
		if block != null:
			actions.append(_block_text(block))
	var text := " and ".join(parts)
	if not choose and not case.get("show"):
		text = ("%s: hide" % text) if not text.is_empty() else "hide"
	if not actions.is_empty():
		text = ("%s: %s" % [text, ", ".join(actions)]) if not text.is_empty() else ", ".join(actions)
	return text if not text.is_empty() else "always"


## A condition or action block as words: what it is ("Quest step", from its class) and its
## fields, with special words for the common ones.
func _block_text(block: Resource) -> String:
	var cls := block.get_class()
	var kind := cls.trim_prefix("RoseDialogIf").trim_prefix("RoseDialogDo").trim_prefix("RoseCondition").trim_prefix("RoseReward")
	match cls:
		"RoseDialogIfQuestStep":
			return "step %s %s" % [_link_text(block.get("step")), "passes" if block.get("passes") else "fails"]
		"RoseDialogIfQuest":
			return "quest %s %s" % [_link_text(block.get("quest")), "active" if block.get("active") else "not active"]
		"RoseDialogDoQuestStep":
			return "step %s" % _link_text(block.get("step"))
		"RoseDialogDoOpen":
			return "open %s" % block.get("window")
		"RoseRewardTrigger":
			return "run step %s" % block.get("trigger")
	if not _block_properties.has(cls):
		_block_properties[cls] = ClassDB.class_get_property_list(cls, true).filter(func(property: Dictionary) -> bool:
			return property.usage & PROPERTY_USAGE_STORAGE and not property.name in SKIP)
	# Fields as words: a choice (ability, window ...) or link by itself, a number with its
	# name, "operator value" as a comparison (Job >= 111); the kind first unless it starts
	# with a choice that already says what it is.
	var parts := PackedStringArray()
	var named := false
	for property in _block_properties[cls]:
		var value: Variant = block.get(property.name)
		if value is bool:
			parts.append(property.name.replace("_", " ") if value else "not " + property.name.replace("_", " "))
		elif value is Resource or (value == null and property.type == TYPE_OBJECT):
			parts.append(_link_text(value))
		elif property.name in ["operator", "value"]:
			parts.append(str(value))
		elif value is String or value is StringName:
			parts.append(str(value))
			named = named or parts.size() == 1
		elif kind.to_lower().ends_with(property.name.replace("_", "")):
			parts.append(str(value))
		else:
			parts.append("%s %s" % [property.name.replace("_", " "), value])
	# A comparison saved after its number reads "count == 1", not "count 1 ==".
	if parts.size() >= 2 and parts[-1] in COMPARE and parts[-2].contains(" "):
		var number := parts[-2].rsplit(" ", true, 1)
		parts[-2] = "%s %s %s" % [number[0], parts[-1], number[1]]
		parts.remove_at(parts.size() - 1)
	var words := kind.capitalize().to_lower()
	if parts.is_empty():
		return words
	return " ".join(parts) if named else "%s %s" % [words, " ".join(parts)]


func _link_text(res: Variant) -> String:
	if not res is Resource:
		return "(none)"
	var key: Variant = res.get("id") if "id" in res else null
	var title: Variant = res.get("name")
	if key != null and title != null:
		return "%s %s" % [key, title]
	if title != null and not str(title).is_empty():
		return str(title)
	return res.resource_path.get_file().get_basename()


## A step's file name from its name, as the conversion makes it.
static func _stem(step_name: String) -> String:
	var out := ""
	for c in step_name:
		out += c if (c >= "a" and c <= "z") or (c >= "A" and c <= "Z") or (c >= "0" and c <= "9") or c == "-" or c == "_" else "_"
	return out


# ---------------------------------------------------------------------------------------
# Inspector

func _inspect(res: Object) -> void:
	# Deferred: never in the same frame as something else the editor opens.
	EditorInterface.inspect_object.call_deferred(res, "", true)


func _on_node_selected(node: Node) -> void:
	if _finding:
		return
	if node.has_meta("res"):
		_inspect(node.get_meta("res"))
	elif node.has_meta("path") and not str(node.get_meta("path")).is_empty():
		_inspect(load(node.get_meta("path")))
	_update_bar()


## A change in the Inspector to the dialog or a step, or to anything inside a box, redraws
## that box (once, after the frame).
func _on_inspector_edited(_property: String) -> void:
	var res := EditorInterface.get_inspector().get_edited_object() as Resource
	if res == null or mode == Mode.NONE:
		return
	if res == dialog:
		_mark(dialog)
		_refresh_pending[-1] = true
	elif _box_of.has(res):
		_mark(dialog if mode == Mode.DIALOG else steps[_box_of[res]].res)
		_refresh_pending[_box_of[res]] = true
	else:
		return
	_flush_refresh.call_deferred()


func _flush_refresh() -> void:
	if _refresh_pending.is_empty():
		return
	var boxes: Array = _refresh_pending.keys()
	_refresh_pending.clear()
	if -1 in boxes:
		boxes = range(_boxes.size())
	_refresh(boxes)


# ---------------------------------------------------------------------------------------
# Changing things (each change is one undoable action)

func _undo() -> EditorUndoRedoManager:
	return plugin.get_undo_redo()


func _top() -> Resource:
	return dialog if mode == Mode.DIALOG else (steps[0].res if not steps.is_empty() else null)


## Marks a resource changed, for Save.
func _mark(res: Resource) -> void:
	if res == null:
		return
	var path := res.resource_path
	for entry in steps:
		if entry.res == res:
			path = entry.path
	if not path.is_empty():
		dirty[path] = res
	_update_bar()


## The do and undo of a change ends by marking `res` changed and redrawing `boxes`.
func _commit(res: Resource, boxes: Array) -> void:
	_undo().add_do_method(self, "_changed", res, boxes)
	_undo().add_undo_method(self, "_changed", res, boxes)
	_undo().commit_action()


func _changed(res: Resource, boxes: Array) -> void:
	_mark(res)
	if res != null:
		res.notify_property_list_changed()
	_refresh(boxes)


func _set_line(m: int, l: int, property: String, value: Variant, action: String) -> void:
	var line: Resource = _lines(m)[l]
	if line == null or line.get(property) == value:
		return
	_undo().create_action(action, UndoRedo.MERGE_DISABLE, dialog)
	_undo().add_do_property(line, property, value)
	_undo().add_undo_property(line, property, line.get(property))
	_commit(dialog, [m])


## Replaces a menu's lines with `lines` (a changed copy).
func _set_lines(m: int, lines: Array, action: String) -> void:
	var menu: Resource = _menus()[m]
	_undo().create_action(action, UndoRedo.MERGE_DISABLE, dialog)
	_undo().add_do_property(menu, "lines", lines)
	_undo().add_undo_property(menu, "lines", menu.get("lines").duplicate())
	_commit(dialog, [m])


func _add_line(m: int, at: int, answer: bool) -> void:
	var line: Resource = ClassDB.instantiate("RoseDialogLine")
	line.set("kind", "Answer" if answer else "Npc")
	line.set("text", "New answer" if answer else "New line")
	var lines: Array = _lines(m).duplicate()
	lines.insert(clampi(at, 0, lines.size()), line)
	_set_lines(m, lines, "Add a line to menu %d" % m)


func _move_line(m: int, l: int, by: int) -> void:
	var lines: Array = _lines(m).duplicate()
	var to := l + by
	if to < 0 or to >= lines.size():
		return
	var line: Variant = lines[l]
	lines[l] = lines[to]
	lines[to] = line
	_set_lines(m, lines, "Move a line in menu %d" % m)


func _delete_line(m: int, l: int) -> void:
	var lines: Array = _lines(m).duplicate()
	lines.remove_at(l)
	_set_lines(m, lines, "Delete a line of menu %d" % m)


## Adds a menu (dialog) or a step (folder) where the view is or where you right-clicked.
func _add_box(at: Vector2) -> void:
	if mode == Mode.DIALOG:
		var menu: Resource = ClassDB.instantiate("RoseDialogMenu")
		var line: Resource = ClassDB.instantiate("RoseDialogLine")
		line.set("text", "New line")
		var lines: Array = menu.get("lines")
		lines.append(line)
		menu.set("lines", lines)
		menu.set("graph_position", _not_zero(at))
		menu.resource_name = "Menu %d" % _menus().size()
		var menus: Array = _menus().duplicate()
		menus.append(menu)
		_undo().create_action("Add menu %d" % (menus.size() - 1), UndoRedo.MERGE_DISABLE, dialog)
		_undo().add_do_property(dialog, "menus", menus)
		_undo().add_undo_property(dialog, "menus", _menus().duplicate())
		_commit(dialog, [])
	elif mode == Mode.STEPS:
		_context_at = at
		name_field.text = ""
		name_dialog.popup_centered(Vector2i(420, 0))
		name_field.grab_focus()


func _on_name_confirmed() -> void:
	var step_name := name_field.text.strip_edges()
	var path := folder.path_join(_stem(step_name) + ".tres")
	if step_name.is_empty():
		return
	if _step_files.has(_stem(step_name)) or FileAccess.file_exists(path):
		_toast("There is already a step %s" % step_name)
		return
	var res: Resource = ClassDB.instantiate("RoseQuestTrigger")
	res.set("name", step_name)
	res.set("graph_position", _not_zero(_context_at))
	var entry := {"res": res, "path": path}
	_undo().create_action("Add step %s" % step_name, UndoRedo.MERGE_DISABLE, res)
	_undo().add_do_method(self, "_insert_step", entry)
	_undo().add_undo_method(self, "_remove_step", entry)
	_undo().commit_action()


func _insert_step(entry: Dictionary) -> void:
	steps.append(entry)
	_step_files[entry.path.get_file().get_basename()] = entry.path
	dirty[entry.path] = entry.res
	_rebuild_steps()


func _remove_step(entry: Dictionary) -> void:
	steps.erase(entry)
	_step_files.erase(entry.path.get_file().get_basename())
	dirty.erase(entry.path)
	_rebuild_steps()


func _on_delete_request(nodes: Array[StringName]) -> void:
	if mode != Mode.DIALOG:
		_toast("Delete a step's file in the FileSystem dock (it warns about what still links it)")
		return
	var doomed: Array[int] = []
	for node_name in nodes:
		if str(node_name).begins_with("m"):
			var m := int(str(node_name).substr(1))
			if m == 0:
				_toast("Menu 0 is where the talk starts; it stays")
			else:
				doomed.append(m)
	if doomed.is_empty():
		return
	doomed.sort()
	# Menus after a deleted one move up a number, so every line is pointed again; lines that
	# led to a deleted menu end the talk.
	var renumber := {}
	var shift := 0
	for m in _menus().size():
		if m in doomed:
			shift += 1
			renumber[m] = -1
		else:
			renumber[m] = m - shift
	var menus: Array = _menus().duplicate()
	for i in range(doomed.size() - 1, -1, -1):
		menus.remove_at(doomed[i])
	_undo().create_action("Delete %d menus" % doomed.size() if doomed.size() > 1 else "Delete menu %d" % doomed[0], UndoRedo.MERGE_DISABLE, dialog)
	for m in _menus().size():
		if m in doomed:
			continue
		# The Inspector lists menus by these names.
		var menu: Resource = _menus()[m]
		if renumber[m] != m and menu != null and menu.resource_name == "Menu %d" % m:
			_undo().add_do_property(menu, "resource_name", "Menu %d" % renumber[m])
			_undo().add_undo_property(menu, "resource_name", menu.resource_name)
		for line in _lines(m):
			if line == null:
				continue
			var to: int = line.get("next_menu")
			if renumber.has(to) and renumber[to] != to:
				_undo().add_do_property(line, "next_menu", renumber[to])
				_undo().add_undo_property(line, "next_menu", to)
	_undo().add_do_property(dialog, "menus", menus)
	_undo().add_undo_property(dialog, "menus", _menus().duplicate())
	_commit(dialog, [])


func _on_connection_request(from: StringName, port: int, to: StringName, _to_port: int) -> void:
	var to_box := graph.get_node_or_null(NodePath(to)) as GraphNode
	if to_box == null:
		return
	if mode == Mode.DIALOG:
		_set_target(from, port, -1 if to_box == _end_box else _boxes.find(to_box), null)
	else:
		var target: Resource = to_box.get_meta("res", null)
		if target == null and to_box.has_meta("path") and not str(to_box.get_meta("path")).is_empty():
			target = load(to_box.get_meta("path"))
		if target != null:
			_set_target(from, port, -1, target)


func _on_disconnection_request(from: StringName, port: int, _to: StringName, _to_port: int) -> void:
	_set_target(from, port, -1, null)


## Points a line (dialog: menu `to`, -1 = nowhere) or a step's port (`step`, null = none).
func _set_target(from: StringName, port: int, to: int, step: Resource) -> void:
	var box := graph.get_node_or_null(NodePath(from)) as GraphNode
	var b := _boxes.find(box)
	if b < 0:
		return
	if mode == Mode.DIALOG:
		_set_line(b, port, "next_menu", to, "Point a line of menu %d at %s" % [b, "menu %d" % to if to >= 0 else "nowhere"])
		return
	var res: Resource = steps[b].res
	var ports: Array = box.get_meta("ports", [])
	if port >= ports.size():
		return
	var block: Resource = ports[port]
	if block == null:
		if res.get("next_trigger") == step:
			return
		_undo().create_action("Set what %s tries when it fails" % res.get("name"), UndoRedo.MERGE_DISABLE, res)
		_undo().add_do_property(res, "next_trigger", step)
		_undo().add_undo_property(res, "next_trigger", res.get("next_trigger"))
	else:
		var step_name := str(step.get("name")) if step != null else ""
		if str(block.get("trigger")) == step_name:
			return
		_undo().create_action("Set the step %s runs" % res.get("name"), UndoRedo.MERGE_DISABLE, res)
		_undo().add_do_property(block, "trigger", step_name)
		_undo().add_undo_property(block, "trigger", block.get("trigger"))
	_commit(res, [b])


## Moving boxes saves where they are, in their menu or step.
func _on_end_node_move() -> void:
	var moved := []
	for b in _boxes.size():
		var box := _boxes[b]
		var res: Resource = box.get_meta("res", null)
		if box.selected and res != null and res.get("graph_position") != box.position_offset:
			moved.append(b)
	if moved.is_empty():
		return
	var top := _top()
	_undo().create_action("Move %d boxes" % moved.size(), UndoRedo.MERGE_DISABLE, top)
	for b in moved:
		var res: Resource = _boxes[b].get_meta("res")
		_undo().add_do_property(res, "graph_position", _not_zero(_boxes[b].position_offset))
		_undo().add_undo_property(res, "graph_position", res.get("graph_position"))
		if mode == Mode.STEPS:
			_undo().add_do_method(self, "_mark", res)
	_undo().add_do_method(self, "_place_boxes")
	_undo().add_undo_method(self, "_place_boxes")
	if mode == Mode.DIALOG:
		_undo().add_do_method(self, "_mark", dialog)
	_undo().commit_action()


func _place_boxes() -> void:
	for b in _boxes.size():
		_boxes[b].position_offset = _box_position(_boxes[b].get_meta("res"), b)


## (0, 0) means "not moved", so a box put exactly there is kept a pixel away.
static func _not_zero(at: Vector2) -> Vector2:
	return at if at != Vector2.ZERO else Vector2(0, 1)


func _reset_positions() -> void:
	var picked := _boxes.filter(func(box: GraphNode) -> bool: return box.selected)
	if picked.is_empty():
		picked = _boxes
	picked = picked.filter(func(box: GraphNode) -> bool: return box.get_meta("res").get("graph_position") != Vector2.ZERO)
	if picked.is_empty():
		return
	_undo().create_action("Auto layout %d boxes" % picked.size(), UndoRedo.MERGE_DISABLE, _top())
	for box in picked:
		var res: Resource = box.get_meta("res")
		_undo().add_do_property(res, "graph_position", Vector2.ZERO)
		_undo().add_undo_property(res, "graph_position", res.get("graph_position"))
		_undo().add_do_method(self, "_mark", res if mode == Mode.STEPS else dialog)
		_undo().add_undo_method(self, "_mark", res if mode == Mode.STEPS else dialog)
	_undo().add_do_method(self, "_place_boxes")
	_undo().add_undo_method(self, "_place_boxes")
	_undo().commit_action()


# ---------------------------------------------------------------------------------------
# Rows: double-click to change the words, right-click for more

func _on_line_input(event: InputEvent, m: int, l: int) -> void:
	if not event is InputEventMouseButton or not event.pressed:
		return
	if event.button_index == MOUSE_BUTTON_LEFT and event.double_click:
		_edit_text(m, l)
	elif event.button_index == MOUSE_BUTTON_RIGHT:
		get_viewport().set_input_as_handled()
		_context_box = m
		_context_line = l
		var line: Resource = _lines(m)[l]
		context_menu.clear()
		context_menu.add_item("Change the words…", MENU_EDIT_TEXT)
		context_menu.add_item("Make it an NPC line" if str(line.get("kind")) == "Answer" else "Make it an answer", MENU_TOGGLE_KIND)
		context_menu.add_item("Edit in the Inspector", MENU_INSPECT)
		context_menu.add_separator()
		context_menu.add_item("Move up", MENU_UP)
		context_menu.add_item("Move down", MENU_DOWN)
		context_menu.add_item("Add an NPC line below", MENU_ADD_NPC)
		context_menu.add_item("Add an answer below", MENU_ADD_ANSWER)
		context_menu.add_separator()
		context_menu.add_item("Delete the line", MENU_DELETE_LINE)
		context_menu.set_item_disabled(context_menu.get_item_index(MENU_UP), l == 0)
		context_menu.set_item_disabled(context_menu.get_item_index(MENU_DOWN), l == _lines(m).size() - 1)
		_popup_context()


func _on_popup_request(at: Vector2) -> void:
	_context_at = (at + graph.scroll_offset) / graph.zoom
	context_menu.clear()
	context_menu.add_item("Add a menu here" if mode == Mode.DIALOG else "Add a step here…", MENU_ADD_BOX)
	var selected := _selected_box()
	if mode == Mode.DIALOG:
		context_menu.add_item("Delete the selected menus", MENU_DELETE_BOX)
	if mode == Mode.STEPS:
		context_menu.add_item("Used by… (the selected step)", MENU_USED_BY)
		context_menu.set_item_disabled(context_menu.get_item_index(MENU_USED_BY), selected == null or not selected.has_meta("res"))
		var stub := selected != null and selected.has_meta("path") and not str(selected.get_meta("path")).is_empty()
		context_menu.add_item("Open the folder of the selected grey step", MENU_OPEN_FOLDER)
		context_menu.set_item_disabled(context_menu.get_item_index(MENU_OPEN_FOLDER), not stub)
	context_menu.add_item("Auto layout (the selected boxes, or all)", MENU_LAYOUT)
	if mode == Mode.NONE:
		return
	_popup_context()


func _popup_context() -> void:
	context_menu.reset_size()
	context_menu.position = Vector2i(DisplayServer.mouse_get_position())
	context_menu.popup()


func _on_context_menu(id: int) -> void:
	var m := _context_box
	var l := _context_line
	match id:
		MENU_EDIT_TEXT:
			_edit_text(m, l)
		MENU_TOGGLE_KIND:
			var answer := str(_lines(m)[l].get("kind")) == "Answer"
			_set_line(m, l, "kind", "Npc" if answer else "Answer", "Make a line of menu %d %s" % [m, "an NPC line" if answer else "an answer"])
		MENU_INSPECT:
			_inspect(_lines(m)[l])
		MENU_UP:
			_move_line(m, l, -1)
		MENU_DOWN:
			_move_line(m, l, 1)
		MENU_ADD_NPC, MENU_ADD_ANSWER:
			_add_line(m, l + 1, id == MENU_ADD_ANSWER)
		MENU_DELETE_LINE:
			_delete_line(m, l)
		MENU_ADD_BOX:
			_add_box(_context_at)
		MENU_DELETE_BOX:
			var names: Array[StringName] = []
			for box in _boxes:
				if box.selected:
					names.append(box.name)
			_on_delete_request(names)
		MENU_USED_BY:
			_used_by_selected()
		MENU_OPEN_FOLDER:
			var path: String = _selected_box().get_meta("path")
			_open_path(path.get_base_dir(), load(path))
		MENU_LAYOUT:
			_reset_positions()


func _edit_text(m: int, l: int) -> void:
	var line: Resource = _lines(m)[l]
	if line == null:
		return
	_context_box = m
	_context_line = l
	text_edit.text = str(line.get("text"))
	text_popup.popup(Rect2i(Vector2i(DisplayServer.mouse_get_position()) - Vector2i(40, 20), Vector2i(480, 180)))
	text_edit.grab_focus()
	text_edit.select_all()


func _make_text_popup() -> void:
	text_popup = PopupPanel.new()
	var column := VBoxContainer.new()
	text_popup.add_child(column)
	var hint := Label.new()
	hint.text = "{br} breaks the line, {b}…{/b} is bold, <NAME> is the player's name"
	hint.add_theme_color_override("font_color", Color(1, 1, 1, 0.55))
	column.add_child(hint)
	text_edit = TextEdit.new()
	text_edit.wrap_mode = TextEdit.LINE_WRAPPING_BOUNDARY
	text_edit.custom_minimum_size = Vector2(460, 110)
	text_edit.size_flags_vertical = SIZE_EXPAND_FILL
	text_edit.gui_input.connect(func(event: InputEvent) -> void:
		if event is InputEventKey and event.pressed and event.keycode == KEY_ENTER and event.ctrl_pressed:
			_commit_text()
			text_edit.accept_event())
	column.add_child(text_edit)
	var buttons := HBoxContainer.new()
	buttons.alignment = BoxContainer.ALIGNMENT_END
	column.add_child(buttons)
	var cancel := Button.new()
	cancel.text = "Cancel"
	cancel.pressed.connect(text_popup.hide)
	buttons.add_child(cancel)
	var ok := Button.new()
	ok.text = "OK (Ctrl+Enter)"
	ok.pressed.connect(_commit_text)
	buttons.add_child(ok)
	add_child(text_popup)


func _commit_text() -> void:
	text_popup.hide()
	if _context_box >= 0 and _context_box < _menus().size() and _context_line < _lines(_context_box).size():
		_set_line(_context_box, _context_line, "text", text_edit.text, "Change a line of menu %d" % _context_box)


func _make_name_dialog() -> void:
	name_dialog = ConfirmationDialog.new()
	name_dialog.title = "Add a quest step"
	var column := VBoxContainer.new()
	var label := Label.new()
	label.text = "Its name (what dialogs and other steps call it, like 124-05). The file is saved with Ctrl+S."
	label.autowrap_mode = TextServer.AUTOWRAP_WORD
	label.custom_minimum_size.x = 380
	column.add_child(label)
	name_field = LineEdit.new()
	name_field.text_submitted.connect(func(_text: String) -> void:
		name_dialog.hide()
		_on_name_confirmed())
	column.add_child(name_field)
	name_dialog.add_child(column)
	name_dialog.confirmed.connect(_on_name_confirmed)
	add_child(name_dialog)


# ---------------------------------------------------------------------------------------
# Find

func _find(next: bool) -> void:
	var query := find_field.text.strip_edges().to_lower()
	_finding = true
	for box in _find_hits:
		if is_instance_valid(box):
			box.selected = false
	var hits: Array[GraphNode] = []
	if not query.is_empty():
		for child in graph.get_children():
			if child is GraphNode and str(child.get_meta("find", "")).contains(query):
				child.selected = true
				hits.append(child)
	_finding = false
	if hits != _find_hits:
		_find_hits = hits
		_find_at = -1
	if query.is_empty():
		return
	if hits.is_empty():
		_status("Nothing found")
		return
	if next or _find_at < 0:
		_find_at = (_find_at + 1) % hits.size()
		_show_box(hits[_find_at])
	_status("%d of %d boxes" % [_find_at + 1, hits.size()])


func _show_box(box: GraphNode) -> void:
	if box == null:
		return
	var center := box.position_offset + box.size * 0.5
	graph.scroll_offset = center * graph.zoom - graph.size * 0.5


## Scrolls so the box is at the top left (where a graph starts).
func _show_start(box: GraphNode) -> void:
	if box != null:
		graph.scroll_offset = box.position_offset * graph.zoom - Vector2(80, 60)


func _view_center() -> Vector2:
	return (graph.scroll_offset + graph.size * 0.5) / graph.zoom


func _selected_box() -> GraphNode:
	for child in graph.get_children():
		if child is GraphNode and child.selected:
			return child
	return null


# ---------------------------------------------------------------------------------------
# Used by: a text search of the files, nothing loaded

func _make_used_by_dialog() -> void:
	used_by_dialog = AcceptDialog.new()
	used_by_dialog.title = "Used by"
	used_by_dialog.ok_button_text = "Close"
	used_by_list = ItemList.new()
	used_by_list.custom_minimum_size = Vector2(520, 300)
	used_by_list.item_activated.connect(_on_used_by_activated)
	used_by_dialog.add_child(used_by_list)
	add_child(used_by_dialog)


func _used_by_selected() -> void:
	var box := _selected_box()
	if mode != Mode.STEPS or box == null:
		_toast("Select a step first")
		return
	var res: Resource = box.get_meta("res", null)
	var path: String = box.get_meta("path", "")
	if res != null:
		path = steps[_boxes.find(box)].path
	var step_name: String = str(res.get("name")) if res != null else box.title
	_used_by_needles = PackedStringArray(["path=\"%s\"" % path, "trigger = \"%s\"" % step_name, "trigger_name = \"%s\"" % step_name])
	_used_by_files.clear()
	for dir in USED_BY_DIRS:
		_list_files(dir, _used_by_files)
	_used_by_files.erase(path)
	_used_by_hits.clear()
	_used_by_at = 0
	_used_by_started = Time.get_ticks_msec()
	used_by_dialog.title = "Used by %s" % step_name
	set_process(true)


func _list_files(dir: String, out: Array[String]) -> void:
	for file in DirAccess.get_files_at(dir):
		if file.ends_with(".tres") or file.ends_with(".tscn"):
			out.append(dir.path_join(file))
	for sub in DirAccess.get_directories_at(dir):
		if not sub.begins_with("."):
			_list_files(dir.path_join(sub), out)


func _used_by_step() -> void:
	var end := mini(_used_by_at + FILES_PER_FRAME, _used_by_files.size())
	for i in range(_used_by_at, end):
		var text := FileAccess.get_file_as_string(_used_by_files[i])
		for needle in _used_by_needles:
			if text.contains(needle):
				_used_by_hits.append(_used_by_files[i])
				break
	_used_by_at = end
	_status("Used by… %d of %d files" % [_used_by_at, _used_by_files.size()])
	if _used_by_at < _used_by_files.size():
		return
	var searched := _used_by_files.size()
	_used_by_files.clear()
	set_process(not _build_queue.is_empty())
	used_by_list.clear()
	for hit in _used_by_hits:
		used_by_list.add_item(hit.trim_prefix("res://"))
		used_by_list.set_item_metadata(used_by_list.item_count - 1, hit)
	if _used_by_hits.is_empty():
		used_by_list.add_item("Nothing runs it")
	_status("Used by: %d of %d files · %d ms" % [_used_by_hits.size(), searched, Time.get_ticks_msec() - _used_by_started])
	used_by_dialog.popup_centered()


## Opens what uses the step: a dialog or step folder in the graph (the step's name in Find),
## a zone scene, or anything else in the Inspector.
func _on_used_by_activated(index: int) -> void:
	var path: Variant = used_by_list.get_item_metadata(index)
	if path == null:
		return
	used_by_dialog.hide()
	var step_name := used_by_dialog.title.trim_prefix("Used by ")
	if path.begins_with(DIALOGS_DIR) or path.begins_with(STEPS_DIR):
		_open_path(path if path.begins_with(DIALOGS_DIR) else path.get_base_dir(), load(path) if path.begins_with(STEPS_DIR) else null)
		find_field.text = step_name
		_after_find(step_name)
	elif path.ends_with(".tscn"):
		EditorInterface.open_scene_from_path.call_deferred(path)
	else:
		_inspect(load(path))


## Finds the step in the graph once it is built.
func _after_find(_step_name: String) -> void:
	if _after_build.is_valid():
		var then := _after_build
		_after_build = func() -> void:
			then.call()
			_find(true)
	else:
		_find(true)


# ---------------------------------------------------------------------------------------
# Saving

func save_changes() -> void:
	var saved := 0
	for path in dirty.keys():
		var res: Resource = dirty[path]
		var new_file := not FileAccess.file_exists(path)
		var error := ResourceSaver.save(res, path)
		if error != OK:
			push_error("rose: can't save %s (%s)" % [path, error_string(error)])
			continue
		if new_file:
			res.take_over_path(path)
			EditorInterface.get_resource_filesystem().update_file(path)
		saved += 1
	dirty.clear()
	_update_bar()
	if mode == Mode.STEPS:
		for b in _boxes.size():
			_fill_step_box(_boxes[b], b)
		_connect_all()
	if saved > 0:
		_toast("Saved %d resources. Run --export-data so the game gets them." % saved)


func _update_bar() -> void:
	save_button.text = "Save (%d)" % dirty.size() if not dirty.is_empty() else "Saved"
	save_button.disabled = dirty.is_empty()
	add_button.text = "Add step" if mode == Mode.STEPS else "Add menu"
	add_button.tooltip_text = "Add a quest step file to this folder" if mode == Mode.STEPS else "Add a menu with one NPC line"
	add_button.disabled = mode == Mode.NONE
	used_by_button.visible = mode == Mode.STEPS


func _status(text: String) -> void:
	status_label.text = text


func _toast(text: String) -> void:
	EditorInterface.get_editor_toaster().push_toast(text)
