## Our character, top left: portrait with level, name and job, and HP, MP and Stamina bars
## with their numbers.
extends PanelContainer

const Portrait := preload("res://scripts/ui/hud/portrait.gd")
const JOBS := {0: "Visitor", 111: "Soldier", 121: "Knight", 122: "Champion", 211: "Muse", 221: "Mage",
	222: "Cleric", 311: "Hawker", 321: "Raider", 322: "Scout", 411: "Dealer", 421: "Bourgeois", 422: "Artisan"}

var portrait: Control
var name_label: Label
var job_label: Label
var hp: UiBar
var mp: UiBar
var stamina: UiBar


func _ready() -> void:
	theme_type_variation = "HudPanel"
	mouse_filter = Control.MOUSE_FILTER_STOP
	var row := HBoxContainer.new()
	row.add_theme_constant_override("separation", 10)
	add_child(row)
	portrait = Portrait.new(54)
	portrait.size_flags_vertical = Control.SIZE_SHRINK_CENTER
	row.add_child(portrait)
	var column := VBoxContainer.new()
	column.add_theme_constant_override("separation", 3)
	column.custom_minimum_size.x = 190
	row.add_child(column)
	var names := HBoxContainer.new()
	column.add_child(names)
	name_label = Label.new()
	name_label.theme_type_variation = "HeaderLabel"
	name_label.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	names.add_child(name_label)
	job_label = Label.new()
	job_label.theme_type_variation = "MutedLabel"
	names.add_child(job_label)
	hp = UiBar.new("hp")
	column.add_child(hp)
	mp = UiBar.new("mp")
	column.add_child(mp)
	stamina = UiBar.new("stamina")
	stamina.height_scale = 0.8
	column.add_child(stamina)


func refresh(c: Dictionary, me: Node3D) -> void:
	var name_text: String = c.get("name", "")
	name_label.text = name_text
	job_label.text = JOBS.get(int(c.get("job", 0)), "")
	portrait.set_values(name_text.substr(0, 1).to_upper(), int(c.get("level", 0)))
	if me:
		hp.set_values(me.hp, me.max_hp, "HP  %d / %d" % [me.hp, me.max_hp])
		mp.set_values(me.mp, me.max_mp, "MP  %d / %d" % [me.mp, me.max_mp])
	stamina.set_values(int(c.get("stamina", 0)), 5000, "Stamina  %d / 5000" % int(c.get("stamina", 0)))
