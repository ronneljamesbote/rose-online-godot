## "Open in Graphs" at the top of a dialog's or a quest step's Inspector.
@tool
extends EditorInspectorPlugin

var plugin: EditorPlugin


func _can_handle(object: Object) -> bool:
	return object is Resource and (object.is_class("RoseDialog") or object.is_class("RoseQuestTrigger")) and not object.resource_path.is_empty()


func _parse_begin(object: Object) -> void:
	var button := Button.new()
	button.text = "Open in Graphs"
	button.tooltip_text = "Show this dialog, or this step's folder, as boxes and wires in the Graphs screen"
	button.icon = EditorInterface.get_editor_theme().get_icon("GraphEdit", "EditorIcons")
	button.pressed.connect(func() -> void: plugin.show_in_graph(object))
	add_custom_control(button)
