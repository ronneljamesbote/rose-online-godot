## The game data table: every resource of one kind in res://data/ (a class: weapons,
## monsters, skill levels ...) as a row, its Inspector fields as columns.
## - Pick the kind, then a group tab (the Inspector's groups) or the columns to show.
## - Click a column name to sort, type in the filter ("woopie", "level>=30", "name~jam").
## - Selecting a row shows the resource in the Inspector; changes there show here.
## - Double-click a cell (or press Enter) to change it. With more rows selected (Shift or
##   Ctrl click), the new value goes into all of them; "Change..." adds to or multiplies a
##   number in many rows. Drop a resource from the FileSystem dock on a link cell to set it.
## - Ctrl+C copies the selected cells, Ctrl+V pastes cells from a spreadsheet. A pasted
##   table that starts with the column names (as "Copy table" writes it) finds its rows by id.
## - Every change can be undone (Ctrl+Z); Ctrl+S saves the changed resources.
@tool
extends VBoxContainer

const SETTINGS := "rose_data_tools"
const DATA_DIR := "res://data"
## Resource fields that aren't game data (graph_position: where the Graphs screen shows a box).
const SKIP := ["resource_local_to_scene", "resource_name", "resource_path", "resource_scene_unique_id", "script", "graph_position"]
## Kinds that number their resources together: a monster and an NPC never have the same id.
const SHARED_IDS := {"RoseMonster": ["RoseNpc"], "RoseNpc": ["RoseMonster"]}
const ALL_GROUPS := "All fields"
const LIST_TEXT_MAX := 80
const FILTER_HELP := "Filter: words to find, or field and value: level>=30 run_speed<400 name~jam drop_table= (no link)"

enum { MENU_INSPECT, MENU_SHOW_FILE, MENU_INSPECT_LINK, MENU_CLEAR_LINK, MENU_COPY, MENU_COPY_TABLE, MENU_PASTE, MENU_CHANGE, MENU_DUPLICATE }
enum { HOW_SET, HOW_ADD, HOW_MULTIPLY }

var plugin: EditorPlugin
## Resources changed in the table and not saved yet, by path.
var dirty := {}

## The kinds found in res://data/: {class, label, paths}.
var kinds: Array[Dictionary] = []
var kind := {}
## The fields of the kind: {name, type, kind, group, read_only, options, hint_string, multiline}.
var columns: Array[Dictionary] = []
## The columns shown, the key (id) first.
var shown: Array[Dictionary] = []
## The resources of the kind: {res, path}.
var rows: Array[Dictionary] = []
## The rows that pass the filter, sorted.
var shown_rows: Array[Dictionary] = []
var key_field := ""
var sort_field := ""
var sort_up := true
var group := ALL_GROUPS
var hidden_fields := {}
var _item_of := {}
var _context_item: TreeItem
var _context_column := -1
var _inspect_pending: Resource

var kind_button: OptionButton
var filter_field: LineEdit
var filter_timer: Timer
var columns_menu: MenuButton
var change_button: Button
var duplicate_button: Button
var save_button: Button
var count_label: Label
var group_tabs: TabBar
var tree: Tree
var context_menu: PopupMenu
var change_dialog: ConfirmationDialog
var change_field: OptionButton
var change_how: OptionButton
var change_value: LineEdit
var change_rows: OptionButton
var change_fields: Array[Dictionary] = []


func _init() -> void:
	name = "GameDataTable"
	size_flags_vertical = SIZE_EXPAND_FILL
	var bar := HBoxContainer.new()
	add_child(bar)
	kind_button = OptionButton.new()
	kind_button.tooltip_text = "The kind of game data to show: one row per resource of this class in res://data/"
	kind_button.item_selected.connect(func(index: int) -> void: _load_kind(kinds[index]))
	bar.add_child(kind_button)
	filter_field = LineEdit.new()
	filter_field.placeholder_text = FILTER_HELP
	filter_field.tooltip_text = FILTER_HELP + "\nAll words must match. Comparisons: = != < <= > >= and ~ (contains)."
	filter_field.clear_button_enabled = true
	filter_field.size_flags_horizontal = SIZE_EXPAND_FILL
	bar.add_child(filter_field)
	filter_timer = Timer.new()
	filter_timer.one_shot = true
	filter_timer.wait_time = 0.25
	filter_timer.timeout.connect(_rebuild)
	add_child(filter_timer)
	filter_field.text_changed.connect(func(_text: String) -> void: filter_timer.start())
	columns_menu = MenuButton.new()
	columns_menu.text = "Columns"
	columns_menu.flat = false
	columns_menu.tooltip_text = "Show or hide columns (the id and name always show)"
	columns_menu.about_to_popup.connect(_fill_columns_menu)
	columns_menu.get_popup().hide_on_checkable_item_selection = false
	columns_menu.get_popup().id_pressed.connect(_on_columns_menu)
	bar.add_child(columns_menu)
	change_button = Button.new()
	change_button.text = "Change..."
	change_button.tooltip_text = "Set, add to or multiply a field in the selected rows or in every shown row"
	change_button.pressed.connect(_open_change_dialog)
	bar.add_child(change_button)
	duplicate_button = Button.new()
	duplicate_button.text = "Duplicate"
	duplicate_button.tooltip_text = "Copy the selected rows to new resources with the next free ids (a new item, monster ...)"
	duplicate_button.pressed.connect(_duplicate_selected)
	bar.add_child(duplicate_button)
	var reload := Button.new()
	reload.text = "Reload"
	reload.tooltip_text = "Look for new or removed resources in res://data/"
	reload.pressed.connect(_reload)
	bar.add_child(reload)
	save_button = Button.new()
	save_button.tooltip_text = "Save the resources changed in the table (Ctrl+S does too)"
	save_button.pressed.connect(save_changes)
	bar.add_child(save_button)
	count_label = Label.new()
	bar.add_child(count_label)

	group_tabs = TabBar.new()
	group_tabs.clip_tabs = false
	group_tabs.tab_changed.connect(_on_group_changed)
	add_child(group_tabs)

	tree = Tree.new()
	tree.size_flags_vertical = SIZE_EXPAND_FILL
	tree.hide_root = true
	tree.column_titles_visible = true
	tree.select_mode = Tree.SELECT_MULTI
	tree.allow_rmb_select = true
	tree.item_edited.connect(_on_item_edited)
	tree.item_activated.connect(_on_item_activated)
	tree.item_mouse_selected.connect(_on_item_mouse_selected)
	tree.multi_selected.connect(_on_multi_selected)
	tree.column_title_clicked.connect(_on_column_title_clicked)
	tree.gui_input.connect(_on_tree_input)
	tree.set_drag_forwarding(Callable(), _can_drop, _drop)
	add_child(tree)

	var help := Label.new()
	help.text = "Double-click a cell or press Enter to change it (in every selected row) · Ctrl+C / Ctrl+V copy and paste with a spreadsheet · drop a resource on a link · right-click for more · Ctrl+Z undoes, Ctrl+S saves"
	help.add_theme_color_override("font_color", Color(1, 1, 1, 0.55))
	help.text_overrun_behavior = TextServer.OVERRUN_TRIM_ELLIPSIS
	add_child(help)

	context_menu = PopupMenu.new()
	context_menu.id_pressed.connect(_on_context_menu)
	add_child(context_menu)
	_make_change_dialog()
	_update_save_button()


func _ready() -> void:
	EditorInterface.get_inspector().property_edited.connect(_on_inspector_edited)


## Called when the screen is shown: finds the kinds the first time.
func open() -> void:
	if kinds.is_empty():
		_reload()


func _settings() -> EditorSettings:
	return EditorInterface.get_editor_settings()


# ---------------------------------------------------------------------------------------
# Kinds and rows

## Finds every kind in res://data/ (by the class written at the top of each .tres, so a
## resource can sit in any folder) and loads the last one shown.
func _reload() -> void:
	var by_class := {}
	var files: Array[String] = []
	_list_tres(DATA_DIR, files)
	for path in files:
		var cls := _file_class(path)
		if not cls.is_empty():
			if not by_class.has(cls):
				by_class[cls] = []
			by_class[cls].append(path)
	kinds.clear()
	for cls in by_class:
		var paths: Array = by_class[cls]
		kinds.append({"class": cls, "label": "%s  (%d)" % [_common_folder(paths), paths.size()], "paths": paths})
	kinds.sort_custom(func(a: Dictionary, b: Dictionary) -> bool: return a.label < b.label)
	kind_button.clear()
	for k in kinds:
		kind_button.add_item(k.label)
		kind_button.set_item_tooltip(kind_button.item_count - 1, k["class"])
	if kinds.is_empty():
		count_label.text = "No resources in res://data/"
		return
	var last: String = kind.get("class", _settings().get_project_metadata(SETTINGS, "kind", ""))
	var index := 0
	for i in kinds.size():
		if kinds[i]["class"] == last:
			index = i
	kind_button.select(index)
	_load_kind(kinds[index])


func _list_tres(dir: String, out: Array[String]) -> void:
	for file in DirAccess.get_files_at(dir):
		if file.ends_with(".tres"):
			out.append(dir.path_join(file))
	for sub in DirAccess.get_directories_at(dir):
		if not sub.begins_with("."):
			_list_tres(dir.path_join(sub), out)


## The class of a .tres from its first line (`[gd_resource type="RoseMonster" ...]`).
static func _file_class(path: String) -> String:
	var file := FileAccess.open(path, FileAccess.READ)
	if file == null:
		return ""
	var line := file.get_line()
	var start := line.find("type=\"")
	if not line.begins_with("[gd_resource") or start < 0:
		return ""
	start += 6
	return line.substr(start, line.find("\"", start) - start)


## The folder all the paths share, below res://data/ ("items/weapon", "skills").
static func _common_folder(paths: Array) -> String:
	var common: PackedStringArray = paths[0].get_base_dir().trim_prefix(DATA_DIR + "/").split("/")
	for path in paths:
		var parts: PackedStringArray = path.get_base_dir().trim_prefix(DATA_DIR + "/").split("/")
		var n := 0
		while n < common.size() and n < parts.size() and common[n] == parts[n]:
			n += 1
		common = common.slice(0, n)
	return "/".join(common) if not common.is_empty() else "data"


func _load_kind(k: Dictionary) -> void:
	var started := Time.get_ticks_msec()
	kind = k
	_settings().set_project_metadata(SETTINGS, "kind", k["class"])
	rows.clear()
	for path in k.paths:
		# The same instance the Inspector edits, so a change in either shows in both.
		var res := load(path) as Resource
		if res != null:
			rows.append({"res": res, "path": path})
	_read_columns()
	var cls: String = k["class"]
	hidden_fields = {}
	for field in _settings().get_project_metadata(SETTINGS, "hidden_" + cls, []):
		hidden_fields[field] = true
	group = _settings().get_project_metadata(SETTINGS, "group_" + cls, ALL_GROUPS)
	sort_field = _settings().get_project_metadata(SETTINGS, "sort_" + cls, key_field)
	sort_up = _settings().get_project_metadata(SETTINGS, "sort_up_" + cls, true)
	if not _has_column(sort_field):
		sort_field = key_field
	_fill_group_tabs()
	_rebuild()
	count_label.tooltip_text = "%s: %d resources, loaded and shown in %d ms" % [k["class"], rows.size(), Time.get_ticks_msec() - started]


## The columns: the Inspector fields of the first resource of the kind.
func _read_columns() -> void:
	columns.clear()
	key_field = ""
	if rows.is_empty():
		return
	var current_group := ""
	for property in rows[0].res.get_property_list():
		var usage: int = property.usage
		if usage & PROPERTY_USAGE_GROUP:
			current_group = property.name
			continue
		if usage & PROPERTY_USAGE_CATEGORY:
			current_group = ""
			continue
		if usage & PROPERTY_USAGE_SUBGROUP or not usage & PROPERTY_USAGE_EDITOR or property.name in SKIP:
			continue
		var column := {
			"name": property.name,
			"type": property.type,
			"group": current_group if not current_group.is_empty() else "Other",
			"read_only": usage & PROPERTY_USAGE_READ_ONLY != 0,
			"options": [],
			"hint_string": property.hint_string,
			"multiline": property.hint == PROPERTY_HINT_MULTILINE_TEXT,
		}
		if property.hint == PROPERTY_HINT_ENUM and property.type in [TYPE_INT, TYPE_STRING]:
			for entry in property.hint_string.split(","):
				var parts: PackedStringArray = entry.split(":")
				var value: Variant = parts[0]
				if property.type == TYPE_INT:
					value = int(parts[1]) if parts.size() > 1 else column.options.size()
				column.options.append({"name": parts[0], "value": value})
		match property.type:
			TYPE_BOOL:
				column.kind = "bool"
			TYPE_INT, TYPE_FLOAT:
				column.kind = "number"
			TYPE_STRING, TYPE_STRING_NAME:
				column.kind = "text"
			TYPE_OBJECT:
				column.kind = "link"
			TYPE_VECTOR2, TYPE_VECTOR2I, TYPE_VECTOR3, TYPE_VECTOR3I, TYPE_COLOR:
				column.kind = "value"
			_:
				column.kind = "list"
				column.read_only = true
		if not column.options.is_empty():
			column.kind = "enum"
		columns.append(column)
	for field in ["id", "grade"]:
		if _has_column(field):
			key_field = field
			break
	if key_field.is_empty():
		key_field = columns[0].name


func _has_column(field: String) -> bool:
	return columns.any(func(c: Dictionary) -> bool: return c.name == field)


func _column(field: String) -> Dictionary:
	for c in columns:
		if c.name == field:
			return c
	return {}


func _fill_group_tabs() -> void:
	var groups: Array[String] = [ALL_GROUPS]
	for c in columns:
		if not c.group in groups:
			groups.append(c.group)
	group_tabs.set_block_signals(true)
	group_tabs.clear_tabs()
	for g in groups:
		group_tabs.add_tab(g)
	if not group in groups:
		group = ALL_GROUPS
	group_tabs.current_tab = groups.find(group)
	group_tabs.set_block_signals(false)


func _on_group_changed(tab: int) -> void:
	group = group_tabs.get_tab_title(tab)
	_settings().set_project_metadata(SETTINGS, "group_" + kind["class"], group)
	_rebuild()


func _is_pinned(field: String) -> bool:
	return field == key_field or field == "name"


# ---------------------------------------------------------------------------------------
# The table

func _rebuild() -> void:
	if kind.is_empty():
		return
	var selected := _selected_resources()
	shown.clear()
	for c in columns:
		if c.name == key_field:
			shown.push_front(c)
		elif _is_pinned(c.name) or (not hidden_fields.has(c.name) and (group == ALL_GROUPS or c.group == group)):
			shown.append(c)
	var tests := _parse_filter(filter_field.text)
	shown_rows = rows.filter(func(row: Dictionary) -> bool: return _matches(row, tests))
	_sort_rows()

	tree.clear()
	_item_of.clear()
	tree.columns = maxi(1, shown.size())
	var root := tree.create_item()
	for row in shown_rows:
		var item := tree.create_item(root)
		item.set_metadata(0, row)
		_item_of[row.res] = item
		_fill_row(item, row)
		if row.res in selected:
			item.select(0)
	var font := tree.get_theme_font("font")
	var font_size := tree.get_theme_font_size("font_size")
	var scale := EditorInterface.get_editor_scale()
	for i in shown.size():
		var c := shown[i]
		var title: String = c.name
		if c.name == sort_field:
			title += "  ▲" if sort_up else "  ▼"
		tree.set_column_title(i, title)
		tree.set_column_title_tooltip_text(i, "%s: %s%s\nClick to sort" % [c.group, c.name, "  (read only)" if c.read_only else ""])
		tree.set_column_expand(i, false)
		tree.set_column_clip_content(i, true)
		var width := font.get_string_size(title, HORIZONTAL_ALIGNMENT_LEFT, -1, font_size).x + 24 * scale
		for r in mini(shown_rows.size(), 200):
			var text := _text(shown_rows[r].res.get(c.name), c)
			width = maxf(width, font.get_string_size(text, HORIZONTAL_ALIGNMENT_LEFT, -1, font_size).x + 16 * scale)
		if c.kind == "enum":
			width += 24 * scale  # the dropdown arrow
		tree.set_column_custom_minimum_width(i, int(clampf(width, 48 * scale, 320 * scale)))
	count_label.text = "%d of %d" % [shown_rows.size(), rows.size()] if shown_rows.size() != rows.size() else "%d" % rows.size()


func _fill_row(item: TreeItem, row: Dictionary) -> void:
	item.set_tooltip_text(0, row.path)
	for i in shown.size():
		var c := shown[i]
		var value: Variant = row.res.get(c.name)
		var editable: bool = not c.read_only
		item.clear_custom_bg_color(i)
		match c.kind:
			"bool":
				item.set_cell_mode(i, TreeItem.CELL_MODE_CHECK)
				item.set_checked(i, value)
				item.set_editable(i, editable)
			"enum":
				var names := []
				var index := -1
				for o in c.options:
					names.append(o.name)
					if o.value == value:
						index = names.size() - 1
				if index < 0:
					names.append(str(value))
					index = names.size() - 1
				item.set_cell_mode(i, TreeItem.CELL_MODE_RANGE)
				item.set_text(i, ",".join(names))
				item.set_range_config(i, 0, names.size() - 1, 1)
				item.set_range(i, index)
				item.set_editable(i, editable)
			"link", "list":
				item.set_cell_mode(i, TreeItem.CELL_MODE_STRING)
				var text := _text(value, c)
				item.set_text(i, text.left(LIST_TEXT_MAX) + ("..." if text.length() > LIST_TEXT_MAX else ""))
				item.set_tooltip_text(i, value.resource_path if value is Resource else text)
				item.set_editable(i, false)
				item.set_custom_color(i, Color(0.65, 0.8, 1.0) if c.kind == "link" else Color(1, 1, 1, 0.6))
				# An icon shows itself (it is loaded with the resource anyway).
				if value is Texture2D:
					item.set_icon(i, value)
					item.set_icon_max_width(i, 20)
			_:
				item.set_cell_mode(i, TreeItem.CELL_MODE_STRING)
				item.set_text(i, _text(value, c))
				item.set_editable(i, editable)
				item.set_edit_multiline(i, c.multiline)
				if c.kind == "number":
					item.set_text_alignment(i, HORIZONTAL_ALIGNMENT_RIGHT)
				if value is Color:
					item.set_custom_bg_color(i, value)
					item.set_custom_color(i, Color.BLACK if value.get_luminance() > 0.5 else Color.WHITE)
	# Changed and not saved yet: the id cell shows it.
	if dirty.has(row.path):
		item.set_custom_bg_color(0, Color(1.0, 0.65, 0.1, 0.35))


## A value as the table and the clipboard show it.
func _text(value: Variant, column: Dictionary) -> String:
	match typeof(value):
		TYPE_NIL:
			return ""
		TYPE_OBJECT:
			return _link_text(value)
		TYPE_BOOL:
			return "true" if value else "false"
		TYPE_INT:
			for o in column.get("options", []):
				if o.value == value:
					return o.name
			return str(value)
		TYPE_STRING, TYPE_STRING_NAME, TYPE_FLOAT:
			return str(value)
		TYPE_COLOR:
			return "#" + value.to_html(value.a < 1.0)
		TYPE_ARRAY:
			return ", ".join(value.map(func(v: Variant) -> String: return _text(v, {})))
		TYPE_PACKED_INT32_ARRAY, TYPE_PACKED_INT64_ARRAY, TYPE_PACKED_FLOAT32_ARRAY, TYPE_PACKED_FLOAT64_ARRAY, TYPE_PACKED_STRING_ARRAY:
			return ", ".join(Array(value).map(func(v: Variant) -> String: return str(v)))
	return var_to_str(value)


## A linked resource: its id and name ("147 woopie"), its label (an AI event) or its file name.
func _link_text(res: Object) -> String:
	if not res is Resource:
		return str(res)
	var key: Variant = res.get("id") if "id" in res else res.get("grade")
	var title: Variant = res.get("name")
	if key != null and title != null:
		return "%s %s" % [key, title]
	# A block inside a resource (an AI event) is named after what it does.
	if key == null and not res.resource_name.is_empty():
		return res.resource_name
	if title != null:
		return str(title)
	return res.resource_path.get_file().get_basename()


func _sort_rows() -> void:
	var c := _column(sort_field)
	if c.is_empty():
		return
	var keyed := shown_rows.map(func(row: Dictionary) -> Array:
		var value: Variant = row.res.get(c.name)
		var sort_key: Variant = value if typeof(value) in [TYPE_INT, TYPE_FLOAT, TYPE_BOOL] else _text(value, c).to_lower()
		return [sort_key, row.res.get(key_field), row])
	keyed.sort_custom(func(a: Array, b: Array) -> bool:
		if a[0] == b[0]:
			return a[1] < b[1]
		return (a[0] < b[0]) == sort_up)
	shown_rows.assign(keyed.map(func(k: Array) -> Dictionary: return k[2]))


func _on_column_title_clicked(column: int, mouse_button: int) -> void:
	if mouse_button != MOUSE_BUTTON_LEFT or column >= shown.size():
		return
	var field: String = shown[column].name
	sort_up = not sort_up if field == sort_field else true
	sort_field = field
	_settings().set_project_metadata(SETTINGS, "sort_" + kind["class"], sort_field)
	_settings().set_project_metadata(SETTINGS, "sort_up_" + kind["class"], sort_up)
	_rebuild()


# ---------------------------------------------------------------------------------------
# Filter

## The filter's words: {field, op, value} for "level>=30", {text} for anything else.
func _parse_filter(text: String) -> Array[Dictionary]:
	var tests: Array[Dictionary] = []
	var pattern := RegEx.create_from_string("^([A-Za-z_][A-Za-z0-9_]*)(>=|<=|!=|=|>|<|~)(.*)$")
	for word in text.strip_edges().to_lower().split(" ", false):
		var found := pattern.search(word)
		if found != null and _has_column(found.get_string(1)):
			tests.append({"field": found.get_string(1), "op": found.get_string(2), "value": found.get_string(3)})
		else:
			tests.append({"text": word})
	return tests


func _matches(row: Dictionary, tests: Array[Dictionary]) -> bool:
	for test in tests:
		if test.has("text"):
			var found: bool = row.path.get_file().to_lower().contains(test.text)
			for c in shown:
				if found:
					break
				found = _text(row.res.get(c.name), c).to_lower().contains(test.text)
			if not found:
				return false
			continue
		var c := _column(test.field)
		var value: Variant = row.res.get(test.field)
		var text := _text(value, c).to_lower()
		var want: String = test.value
		var numeric: bool = typeof(value) in [TYPE_INT, TYPE_FLOAT] and c.options.is_empty() and want.is_valid_float()
		var a: Variant = float(value) if numeric else text
		var b: Variant = want.to_float() if numeric else want
		var ok := true
		match test.op:
			"~":
				ok = text.contains(want)
			"=":
				ok = a == b
			"!=":
				ok = a != b
			">":
				ok = a > b
			"<":
				ok = a < b
			">=":
				ok = a >= b
			"<=":
				ok = a <= b
		if not ok:
			return false
	return true


# ---------------------------------------------------------------------------------------
# Columns menu

func _fill_columns_menu() -> void:
	var popup := columns_menu.get_popup()
	popup.clear()
	popup.add_item("Show all columns", 100000)
	var last_group := ""
	for i in columns.size():
		var c := columns[i]
		if _is_pinned(c.name):
			continue
		if c.group != last_group:
			popup.add_separator(c.group)
			last_group = c.group
		popup.add_check_item(c.name, i)
		popup.set_item_checked(popup.item_count - 1, not hidden_fields.has(c.name))


func _on_columns_menu(id: int) -> void:
	var popup := columns_menu.get_popup()
	if id == 100000:
		hidden_fields.clear()
		columns_menu.get_popup().hide()
	else:
		var field: String = columns[id].name
		if hidden_fields.has(field):
			hidden_fields.erase(field)
		else:
			hidden_fields[field] = true
		popup.set_item_checked(popup.get_item_index(id), not hidden_fields.has(field))
	_settings().set_project_metadata(SETTINGS, "hidden_" + kind["class"], hidden_fields.keys())
	_rebuild()


# ---------------------------------------------------------------------------------------
# Selection, the Inspector

## Items with at least one selected cell, top to bottom.
func _selected_items() -> Array[TreeItem]:
	var items: Array[TreeItem] = []
	var item := tree.get_next_selected(null)
	while item != null:
		items.append(item)
		item = tree.get_next_selected(item)
	return items


func _selected_resources() -> Array:
	return _selected_items().map(func(item: TreeItem) -> Resource: return item.get_metadata(0).res)


func _on_multi_selected(item: TreeItem, _column: int, selected: bool) -> void:
	if selected:
		_inspect_pending = item.get_metadata(0).res
		_inspect.call_deferred()


func _inspect() -> void:
	if _inspect_pending != null:
		EditorInterface.inspect_object(_inspect_pending, "", true)
		_inspect_pending = null


func _on_inspector_edited(_property: String) -> void:
	var res := EditorInterface.get_inspector().get_edited_object() as Resource
	if res != null and _item_of.has(res):
		var item: TreeItem = _item_of[res]
		dirty[res.resource_path] = res
		_fill_row(item, item.get_metadata(0))
		_update_save_button()


func _on_item_activated() -> void:
	var item := tree.get_selected()
	var i := tree.get_selected_column()
	if item == null or i < 0 or i >= shown.size():
		return
	var value: Variant = item.get_metadata(0).res.get(shown[i].name)
	if shown[i].kind == "link" and value is Resource:
		EditorInterface.inspect_object(value, "", true)
	elif item.is_editable(i):
		tree.edit_selected(true)


# ---------------------------------------------------------------------------------------
# Changing values

## Applies {res, field, value} changes as one undoable action; returns how many changed.
func _apply(changes: Array[Dictionary], action: String) -> int:
	changes = changes.filter(func(change: Dictionary) -> bool: return change.res.get(change.field) != change.value)
	if changes.is_empty():
		_refresh_rows([])
		return 0
	var undo_redo := plugin.get_undo_redo()
	undo_redo.create_action(action, UndoRedo.MERGE_DISABLE, changes[0].res)
	var touched := []
	for change in changes:
		undo_redo.add_do_property(change.res, change.field, change.value)
		undo_redo.add_undo_property(change.res, change.field, change.res.get(change.field))
		if not change.res in touched:
			touched.append(change.res)
	undo_redo.add_do_method(self, "_refresh_rows", touched)
	undo_redo.add_undo_method(self, "_refresh_rows", touched)
	undo_redo.commit_action()
	return changes.size()


## Marks resources changed and shows their new values here and in the Inspector.
func _refresh_rows(resources: Array) -> void:
	for res in resources:
		dirty[res.resource_path] = res
		res.notify_property_list_changed()
		if _item_of.has(res):
			var item: TreeItem = _item_of[res]
			_fill_row(item, item.get_metadata(0))
	_update_save_button()


func _on_item_edited() -> void:
	var item := tree.get_edited()
	var i := tree.get_edited_column()
	if item == null or i < 0 or i >= shown.size():
		return
	var c := shown[i]
	var value: Variant
	match c.kind:
		"bool":
			value = item.is_checked(i)
		"enum":
			var index := int(item.get_range(i))
			value = c.options[index].value if index < c.options.size() else item.get_metadata(0).res.get(c.name)
		_:
			var parsed := _parse(item.get_text(i), c)
			if parsed.is_empty():
				_refresh_rows.call_deferred([])
				_fill_row.call_deferred(item, item.get_metadata(0))
				_toast("%s: \"%s\" isn't a %s" % [c.name, item.get_text(i), type_string(c.type)])
				return
			value = parsed[0]
	# The other selected cells of the column take the same value.
	var changes: Array[Dictionary] = []
	for other in _selected_items():
		if other == item or other.is_selected(i):
			changes.append({"res": other.get_metadata(0).res, "field": c.name, "value": value})
	if changes.is_empty():
		changes.append({"res": item.get_metadata(0).res, "field": c.name, "value": value})
	tree.grab_focus.call_deferred()
	_apply.call_deferred(changes, "Set %s of %d %s" % [c.name, changes.size(), "row" if changes.size() == 1 else "rows"])


## Text to a value of the column: [value], or [] when it isn't one.
func _parse(text: String, c: Dictionary) -> Array:
	var t := text.strip_edges()
	match c.kind:
		"text":
			return [text]
		"number":
			if c.type == TYPE_INT and t.is_valid_int():
				return [t.to_int()]
			if c.type == TYPE_FLOAT and t.is_valid_float():
				return [t.to_float()]
		"bool":
			if t.to_lower() in ["true", "yes", "1", "x"]:
				return [true]
			if t.to_lower() in ["false", "no", "0", ""]:
				return [false]
		"enum":
			for o in c.options:
				if o.name.to_lower() == t.to_lower() or str(o.value) == t:
					return [o.value]
		"value":
			var value: Variant = str_to_var(t)
			if c.type == TYPE_COLOR and t.begins_with("#"):
				value = Color.html(t) if Color.html_is_valid(t) else null
			if typeof(value) == c.type:
				return [value]
		"link":
			if t.is_empty():
				return [null]
			if t.begins_with("res://") and ResourceLoader.exists(t):
				var res := load(t)
				if res != null and _accepts(c, res.get_class()):
					return [res]
	return []


func _accepts(c: Dictionary, cls: String) -> bool:
	var want: String = c.hint_string
	return want.is_empty() or cls == want or ClassDB.is_parent_class(cls, want)


# ---------------------------------------------------------------------------------------
# Drag and drop on link cells

func _drop_target(at: Vector2, data: Variant) -> Dictionary:
	if typeof(data) != TYPE_DICTIONARY:
		return {}
	var item := tree.get_item_at_position(at)
	var i := tree.get_column_at_position(at)
	if item == null or i < 0 or i >= shown.size() or shown[i].kind != "link" or shown[i].read_only:
		return {}
	var res: Resource
	if data.get("type") == "files" and data.files.size() == 1 and ResourceLoader.exists(data.files[0]):
		if not _accepts(shown[i], _file_class(data.files[0])):
			return {}
		res = load(data.files[0])
	elif data.get("type") == "resource" and data.resource is Resource:
		res = data.resource
	if res == null or not _accepts(shown[i], res.get_class()):
		return {}
	return {"item": item, "column": i, "res": res}


func _can_drop(at: Vector2, data: Variant) -> bool:
	return not _drop_target(at, data).is_empty()


func _drop(at: Vector2, data: Variant) -> void:
	var target := _drop_target(at, data)
	if target.is_empty():
		return
	var item: TreeItem = target.item
	var c := shown[target.column]
	var changes: Array[Dictionary] = [{"res": item.get_metadata(0).res, "field": c.name, "value": target.res}]
	if item.is_selected(target.column):
		for other in _selected_items():
			if other != item and other.is_selected(target.column):
				changes.append({"res": other.get_metadata(0).res, "field": c.name, "value": target.res})
	_apply(changes, "Link %s of %d %s" % [c.name, changes.size(), "row" if changes.size() == 1 else "rows"])


# ---------------------------------------------------------------------------------------
# Copy and paste

func _on_tree_input(event: InputEvent) -> void:
	if event is InputEventKey and event.pressed and event.is_command_or_control_pressed() and not event.echo:
		if event.keycode == KEY_C:
			_copy_selected()
			tree.accept_event()
		elif event.keycode == KEY_V:
			_paste()
			tree.accept_event()


## The selected cells as tab-separated lines: the selected rows, from the first to the last
## column selected in any of them.
func _copy_selected() -> void:
	var items := _selected_items()
	if items.is_empty():
		return
	var first := shown.size()
	var last := -1
	for item in items:
		for i in shown.size():
			if item.is_selected(i):
				first = mini(first, i)
				last = maxi(last, i)
	var lines := PackedStringArray()
	for item in items:
		lines.append(_row_line(item.get_metadata(0), first, last))
	DisplayServer.clipboard_set("\n".join(lines))
	_toast("Copied %d × %d cells" % [items.size(), last - first + 1])


## Every shown row and column, with the column names first (paste it back to change by id).
func _copy_table() -> void:
	var lines := PackedStringArray()
	lines.append("\t".join(shown.map(func(c: Dictionary) -> String: return c.name)))
	for row in shown_rows:
		lines.append(_row_line(row, 0, shown.size() - 1))
	DisplayServer.clipboard_set("\n".join(lines))
	_toast("Copied %d rows with their column names" % shown_rows.size())


func _row_line(row: Dictionary, first: int, last: int) -> String:
	var cells := PackedStringArray()
	for i in range(first, last + 1):
		var value: Variant = row.res.get(shown[i].name)
		var text: String = value.resource_path if value is Resource else _text(value, shown[i])
		cells.append(text.replace("\t", " ").replace("\n", " "))
	return "\t".join(cells)


## Pastes tab-separated cells. With the column names as the first line, each line goes to
## the row with its id and each cell to its column; otherwise they go into the shown rows and
## columns from the selected cell on. Cells that didn't change, read-only ones and ones that
## aren't a value of their column are left as they are.
func _paste() -> void:
	var lines := DisplayServer.clipboard_get().replace("\r", "").split("\n")
	while not lines.is_empty() and lines[lines.size() - 1].is_empty():
		lines.remove_at(lines.size() - 1)
	if lines.is_empty():
		return
	var grid := []
	for line in lines:
		grid.append(line.split("\t"))
	var targets := []  # [row, [columns]] per grid line
	var header: PackedStringArray = grid[0]
	if key_field in header and header.size() > 1 and Array(header).all(func(h: String) -> bool: return _has_column(h)):
		var by_key := {}
		for row in rows:
			by_key[str(row.res.get(key_field))] = row
		var cols := Array(header).map(func(h: String) -> Dictionary: return _column(h))
		for r in range(1, grid.size()):
			var key: String = grid[r][header.find(key_field)] if grid[r].size() > header.find(key_field) else ""
			targets.append([by_key.get(key), cols])
		grid = grid.slice(1)
	else:
		var start_item := _context_item if _context_item != null else tree.get_selected()
		if start_item == null:
			_toast("Select the cell to paste into first")
			return
		var start_row := shown_rows.find(start_item.get_metadata(0))
		var start_column := _context_column if _context_column >= 0 else maxi(0, tree.get_selected_column())
		var cols := shown.slice(start_column)
		for r in grid.size():
			targets.append([shown_rows[start_row + r] if start_row + r < shown_rows.size() else null, cols])
	var changes: Array[Dictionary] = []
	var skipped := 0
	for r in grid.size():
		var row: Variant = targets[r][0]
		var cols: Array = targets[r][1]
		if row == null:
			skipped += 1
			continue
		for i in mini(grid[r].size(), cols.size()):
			var c: Dictionary = cols[i]
			var text: String = grid[r][i]
			var current: Variant = row.res.get(c.name)
			if c.read_only or text == _text(current, c) or (current is Resource and text == current.resource_path):
				continue
			var parsed := _parse(text, c)
			if parsed.is_empty():
				skipped += 1
			else:
				changes.append({"res": row.res, "field": c.name, "value": parsed[0]})
	var changed := _apply(changes, "Paste %d cells" % changes.size())
	_toast("Pasted %d changed cells%s" % [changed, ", %d left out (no row, or not a value of the column)" % skipped if skipped > 0 else ""])


# ---------------------------------------------------------------------------------------
# Context menu

func _on_item_mouse_selected(at: Vector2, mouse_button: int) -> void:
	if mouse_button != MOUSE_BUTTON_RIGHT:
		_context_item = null
		_context_column = -1
		return
	_context_item = tree.get_item_at_position(at)
	_context_column = tree.get_column_at_position(at)
	if _context_item == null:
		return
	context_menu.clear()
	context_menu.add_item("Inspect", MENU_INSPECT)
	context_menu.add_item("Show in FileSystem", MENU_SHOW_FILE)
	if _context_column >= 0 and _context_column < shown.size() and shown[_context_column].kind == "link":
		var value: Variant = _context_item.get_metadata(0).res.get(shown[_context_column].name)
		if value is Resource:
			context_menu.add_item("Inspect %s" % _link_text(value), MENU_INSPECT_LINK)
			if not shown[_context_column].read_only:
				context_menu.add_item("Clear %s" % shown[_context_column].name, MENU_CLEAR_LINK)
	context_menu.add_separator()
	context_menu.add_item("Copy cells", MENU_COPY)
	context_menu.set_item_shortcut(context_menu.item_count - 1, _shortcut(KEY_C))
	context_menu.add_item("Copy table (with column names)", MENU_COPY_TABLE)
	context_menu.add_item("Paste", MENU_PASTE)
	context_menu.set_item_shortcut(context_menu.item_count - 1, _shortcut(KEY_V))
	context_menu.add_separator()
	context_menu.add_item("Change...", MENU_CHANGE)
	context_menu.add_item("Duplicate", MENU_DUPLICATE)
	context_menu.position = Vector2i(tree.get_screen_position() + at)
	context_menu.reset_size()
	context_menu.popup()


func _shortcut(key: Key) -> Shortcut:
	var shortcut := Shortcut.new()
	var event := InputEventKey.new()
	event.keycode = key
	event.command_or_control_autoremap = true
	shortcut.events = [event]
	return shortcut


func _on_context_menu(id: int) -> void:
	var row: Dictionary = _context_item.get_metadata(0) if _context_item != null else {}
	match id:
		MENU_INSPECT:
			EditorInterface.inspect_object(row.res, "", true)
		MENU_SHOW_FILE:
			EditorInterface.get_file_system_dock().navigate_to_path(row.path)
		MENU_INSPECT_LINK:
			EditorInterface.inspect_object(row.res.get(shown[_context_column].name), "", true)
		MENU_CLEAR_LINK:
			_apply([{"res": row.res, "field": shown[_context_column].name, "value": null}], "Clear %s" % shown[_context_column].name)
		MENU_COPY:
			_copy_selected()
		MENU_COPY_TABLE:
			_copy_table()
		MENU_PASTE:
			_paste()
		MENU_CHANGE:
			_open_change_dialog()
		MENU_DUPLICATE:
			_duplicate_selected()
	_context_item = null
	_context_column = -1


# ---------------------------------------------------------------------------------------
# Change... (many rows at once)

func _make_change_dialog() -> void:
	change_dialog = ConfirmationDialog.new()
	change_dialog.title = "Change a field in many rows"
	change_dialog.ok_button_text = "Change"
	var grid := GridContainer.new()
	grid.columns = 2
	change_dialog.add_child(grid)
	for label_text in ["Field", "How", "Value", "Rows"]:
		var label := Label.new()
		label.text = label_text
		grid.add_child(label)
		match label_text:
			"Field":
				change_field = OptionButton.new()
				change_field.item_selected.connect(func(_i: int) -> void: _update_change_how())
				grid.add_child(change_field)
			"How":
				change_how = OptionButton.new()
				change_how.add_item("Set to", HOW_SET)
				change_how.add_item("Add", HOW_ADD)
				change_how.add_item("Multiply by", HOW_MULTIPLY)
				grid.add_child(change_how)
			"Value":
				change_value = LineEdit.new()
				change_value.custom_minimum_size.x = 260 * EditorInterface.get_editor_scale()
				change_value.placeholder_text = "50, -10, 1.1, a name, true ..."
				change_value.text_submitted.connect(func(_t: String) -> void:
					change_dialog.hide()
					_on_change_confirmed())
				grid.add_child(change_value)
			"Rows":
				change_rows = OptionButton.new()
				grid.add_child(change_rows)
	change_dialog.confirmed.connect(_on_change_confirmed)
	add_child(change_dialog)


func _open_change_dialog() -> void:
	if kind.is_empty():
		return
	change_fields.assign(columns.filter(func(c: Dictionary) -> bool: return not c.read_only and c.kind in ["number", "text", "bool", "enum", "value"] and c.name != key_field))
	change_field.clear()
	var focus := tree.get_selected_column() if _context_column < 0 else _context_column
	var focus_name: String = shown[focus].name if focus >= 0 and focus < shown.size() else ""
	if focus_name.is_empty() or _is_pinned(focus_name):
		# The first column after the id and name (the group tab's first field).
		for c in shown:
			if not _is_pinned(c.name):
				focus_name = c.name
				break
	for i in change_fields.size():
		change_field.add_item("%s  (%s)" % [change_fields[i].name, change_fields[i].group])
		if change_fields[i].name == focus_name:
			change_field.select(i)
	var selected := _selected_items().size()
	change_rows.clear()
	change_rows.add_item("The %d selected rows" % selected)
	change_rows.add_item("All %d shown rows" % shown_rows.size())
	change_rows.set_item_disabled(0, selected == 0)
	change_rows.select(0 if selected > 1 else 1)
	_update_change_how()
	change_dialog.popup_centered()
	change_value.grab_focus()


func _update_change_how() -> void:
	var numeric: bool = change_field.selected >= 0 and change_fields[change_field.selected].kind == "number"
	change_how.set_item_disabled(HOW_ADD, not numeric)
	change_how.set_item_disabled(HOW_MULTIPLY, not numeric)
	if not numeric:
		change_how.select(HOW_SET)


func _on_change_confirmed() -> void:
	if change_field.selected < 0:
		return
	var c := change_fields[change_field.selected]
	var how := change_how.get_selected_id()
	var targets: Array = _selected_resources() if change_rows.selected == 0 else shown_rows.map(func(row: Dictionary) -> Resource: return row.res)
	var amount := change_value.text.strip_edges()
	var parsed := _parse(amount, c)
	if how != HOW_SET:
		parsed = [amount.to_float()] if amount.is_valid_float() else []
	if parsed.is_empty():
		_toast("%s: \"%s\" isn't a %s" % [c.name, amount, type_string(c.type) if how == HOW_SET else "number"])
		return
	var changes: Array[Dictionary] = []
	for res in targets:
		var value: Variant = parsed[0]
		if how == HOW_ADD:
			value = res.get(c.name) + value
		elif how == HOW_MULTIPLY:
			value = res.get(c.name) * value
		if c.type == TYPE_INT:
			value = roundi(value)
		changes.append({"res": res, "field": c.name, "value": value})
	var changed := _apply(changes, "%s %s of %d rows" % [change_how.get_item_text(change_how.selected), c.name, changes.size()])
	_toast("Changed %s in %d of %d rows" % [c.name, changed, changes.size()])


# ---------------------------------------------------------------------------------------
# Duplicate, save

## Copies the selected rows into new files next to them, each with the next free id of the
## kind (and of the kinds sharing its numbers), named `<id>-<name>.tres`.
func _duplicate_selected() -> void:
	var sources: Array = _selected_resources()
	if sources.is_empty():
		_toast("Select the rows to duplicate first")
		return
	var used := {}
	var classes: Array = [kind["class"]] + SHARED_IDS.get(kind["class"], [])
	for k in kinds:
		if k["class"] in classes:
			for path in k.paths:
				var res := load(path) as Resource
				if res != null:
					used[res.get(key_field)] = true
	var next: int = used.keys().max() + 1
	var made: Array[Resource] = []
	for source in sources:
		var copy: Resource = source.duplicate()
		for c in columns:
			if typeof(copy.get(c.name)) in [TYPE_ARRAY, TYPE_DICTIONARY]:
				copy.set(c.name, copy.get(c.name).duplicate())
		copy.set(key_field, next)
		var title: String = str(source.get("name")) if "name" in source else "new"
		var slug := RegEx.create_from_string("[^a-z0-9]+").sub(title.to_lower(), "-", true).strip_edges().trim_prefix("-").trim_suffix("-")
		var path: String = source.resource_path.get_base_dir().path_join("%d-%s.tres" % [next, slug if not slug.is_empty() else "new"])
		var error := ResourceSaver.save(copy, path)
		if error != OK:
			push_error("rose: can't write %s (%s)" % [path, error_string(error)])
			continue
		copy.take_over_path(path)
		kind.paths.append(path)
		rows.append({"res": copy, "path": path})
		made.append(copy)
		next += 1
	EditorInterface.get_resource_filesystem().scan()
	kind.label = "%s  (%d)" % [_common_folder(kind.paths), kind.paths.size()]
	kind_button.set_item_text(kinds.find(kind), kind.label)
	_rebuild()
	for item in _selected_items():
		item.deselect(0)
	for res in made:
		if _item_of.has(res):
			_item_of[res].select(0)
			tree.scroll_to_item(_item_of[res])
	if not made.is_empty():
		EditorInterface.inspect_object(made[0], "", true)
		_toast("Made %d new %s: %s" % [made.size(), kind["class"], ", ".join(made.map(func(r: Resource) -> String: return r.resource_path.get_file()))])


func save_changes() -> void:
	for path in dirty.keys():
		var error := ResourceSaver.save(dirty[path], path)
		if error != OK:
			push_error("rose: can't save %s (%s)" % [path, error_string(error)])
	var saved := dirty.size()
	dirty.clear()
	for item in _item_of.values():
		_fill_row(item, item.get_metadata(0))
	_update_save_button()
	if saved > 0:
		_toast("Saved %d resources. Run --export-data so the game gets them." % saved)


func _update_save_button() -> void:
	save_button.text = "Save (%d)" % dirty.size() if not dirty.is_empty() else "Saved"
	save_button.disabled = dirty.is_empty()


func _toast(text: String) -> void:
	EditorInterface.get_editor_toaster().push_toast(text)
