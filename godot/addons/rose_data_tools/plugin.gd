## The "Game data" screen next to 2D, 3D and Script: the resources of one kind in
## res://data/ as a table (see table_view.gd and data/README.md).
@tool
extends EditorPlugin

const TableView := preload("res://addons/rose_data_tools/table_view.gd")

var table: TableView


func _enter_tree() -> void:
	table = TableView.new()
	table.plugin = self
	table.hide()
	EditorInterface.get_editor_main_screen().add_child(table)


func _exit_tree() -> void:
	if table != null:
		table.queue_free()
		table = null


func _has_main_screen() -> bool:
	return true


func _make_visible(visible: bool) -> void:
	if table != null:
		table.visible = visible
		if visible:
			table.open()


func _get_plugin_name() -> String:
	return "Game data"


func _get_plugin_icon() -> Texture2D:
	return EditorInterface.get_editor_theme().get_icon("GridContainer", "EditorIcons")


## Ctrl+S (and saving before a run) saves the rows changed in the table too.
func _save_external_data() -> void:
	if table != null:
		table.save_changes()


func _get_unsaved_status(for_scene: String) -> String:
	if for_scene.is_empty() and table != null and not table.dirty.is_empty():
		return "Save the %d game data resources changed in the table?" % table.dirty.size()
	return ""
