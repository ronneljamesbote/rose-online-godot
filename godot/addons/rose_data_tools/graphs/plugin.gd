## The "Graphs" screen next to 2D, 3D, Script and Game data: an NPC dialog or a folder of
## quest steps as boxes and wires (see graph_view.gd and data/README.md). Double-clicking a
## dialog or step file, or "Open in Graphs" at the top of its Inspector, shows it here.
@tool
extends EditorPlugin

const GraphView := preload("res://addons/rose_data_tools/graphs/graph_view.gd")
const OpenButton := preload("res://addons/rose_data_tools/graphs/open_button.gd")

var view: GraphView
var inspector_plugin: OpenButton


func _enter_tree() -> void:
	view = GraphView.new()
	view.plugin = self
	view.hide()
	EditorInterface.get_editor_main_screen().add_child(view)
	inspector_plugin = OpenButton.new()
	inspector_plugin.plugin = self
	add_inspector_plugin(inspector_plugin)


func _exit_tree() -> void:
	if inspector_plugin != null:
		remove_inspector_plugin(inspector_plugin)
		inspector_plugin = null
	if view != null:
		view.queue_free()
		view = null


func _has_main_screen() -> bool:
	return true


func _make_visible(visible: bool) -> void:
	if view != null:
		view.visible = visible
		if visible:
			view.open()


func _get_plugin_name() -> String:
	return "Graphs"


func _get_plugin_icon() -> Texture2D:
	return EditorInterface.get_editor_theme().get_icon("GraphEdit", "EditorIcons")


## A dialog or step opened from the FileSystem dock comes here. (The Game data table and
## this screen show resources in the Inspector only, so they don't switch screens.)
func _handles(object: Object) -> bool:
	return object is Resource and (object.is_class("RoseDialog") or object.is_class("RoseQuestTrigger")) and not object.resource_path.is_empty()


func _edit(object: Object) -> void:
	if object != null and view != null:
		view.open_resource.call_deferred(object)


func show_in_graph(res: Resource) -> void:
	EditorInterface.set_main_screen_editor("Graphs")
	view.open_resource.call_deferred(res)


## Ctrl+S (and saving before a run) saves what was changed in the graph too.
func _save_external_data() -> void:
	if view != null:
		view.save_changes()


func _get_unsaved_status(for_scene: String) -> String:
	if for_scene.is_empty() and view != null and not view.dirty.is_empty():
		return "Save the %d dialogs and quest steps changed in the graph?" % view.dirty.size()
	return ""
