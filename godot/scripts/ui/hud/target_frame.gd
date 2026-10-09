## What we have targeted, top centre: name (red for monsters and enemies), level and life.
extends PanelContainer

var name_label: Label
var level_label: Label
var hp: UiBar


func _ready() -> void:
	theme_type_variation = "HudPanel"
	mouse_filter = Control.MOUSE_FILTER_STOP
	custom_minimum_size.x = 300
	var column := VBoxContainer.new()
	column.add_theme_constant_override("separation", 4)
	add_child(column)
	var names := HBoxContainer.new()
	column.add_child(names)
	name_label = Label.new()
	name_label.theme_type_variation = "HeaderLabel"
	name_label.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	name_label.clip_text = true
	names.add_child(name_label)
	level_label = Label.new()
	level_label.theme_type_variation = "MutedLabel"
	names.add_child(level_label)
	hp = UiBar.new("hp")
	hp.height_scale = 1.15
	column.add_child(hp)


func refresh(target: Node3D, hostile: bool) -> void:
	if target == null:
		visible = false
		return
	visible = true
	name_label.text = target.label.text
	var colour := get_theme_color("enemy" if hostile else "friendly", "Ui")
	name_label.add_theme_color_override("font_color", colour if hostile or target.is_npc else get_theme_color("font_color", "HeaderLabel"))
	level_label.text = "Lv %d" % target.level if target.level > 0 else ""
	var max_hp: int = max(int(target.max_hp), 1)
	hp.set_values(target.hp, max_hp, "%d / %d" % [target.hp, max_hp])
