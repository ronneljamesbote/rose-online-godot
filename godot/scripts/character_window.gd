## Character window (C): level, experience, basic stats with buttons to spend stat points,
## and the ability values the server calculated from them.
extends PanelContainer

const STATS := [["str", "Strength"], ["dex", "Dexterity"], ["int", "Intelligence"],
	["con", "Concentration"], ["cha", "Charm"], ["sen", "Sense"]]
const VALUES := [["attack", "Attack"], ["defence", "Defence"], ["hit", "Hit"], ["avoid", "Avoid"],
	["critical", "Critical"], ["resistance", "Magic resist"], ["attack_speed", "Attack speed"]]

var net: RoseNet
var header: Label
var points: Label
var stat_labels := {}
var stat_buttons := {}
var value_labels := {}


func _ready() -> void:
	custom_minimum_size = Vector2(260, 0)
	var margin := MarginContainer.new()
	for side in ["left", "right", "top", "bottom"]:
		margin.add_theme_constant_override("margin_" + side, 12)
	add_child(margin)
	var box := VBoxContainer.new()
	box.add_theme_constant_override("separation", 4)
	margin.add_child(box)

	header = Label.new()
	header.add_theme_font_size_override("font_size", 18)
	box.add_child(header)
	points = Label.new()
	box.add_child(points)
	box.add_child(HSeparator.new())

	var grid := GridContainer.new()
	grid.columns = 3
	box.add_child(grid)
	for i in STATS.size():
		var key: String = STATS[i][0]
		var name_label := Label.new()
		name_label.text = STATS[i][1]
		name_label.size_flags_horizontal = Control.SIZE_EXPAND_FILL
		grid.add_child(name_label)
		var value := Label.new()
		value.horizontal_alignment = HORIZONTAL_ALIGNMENT_RIGHT
		value.custom_minimum_size.x = 40
		grid.add_child(value)
		stat_labels[key] = value
		var plus := Button.new()
		plus.text = "+"
		plus.focus_mode = Control.FOCUS_NONE
		plus.pressed.connect(func(): net.add_basic_stat(i))
		grid.add_child(plus)
		stat_buttons[key] = plus
	box.add_child(HSeparator.new())

	var values := GridContainer.new()
	values.columns = 2
	box.add_child(values)
	for entry in VALUES:
		var name_label := Label.new()
		name_label.text = entry[1]
		name_label.size_flags_horizontal = Control.SIZE_EXPAND_FILL
		values.add_child(name_label)
		var value := Label.new()
		value.horizontal_alignment = HORIZONTAL_ALIGNMENT_RIGHT
		values.add_child(value)
		value_labels[entry[0]] = value


func _process(_delta: float) -> void:
	if not visible or net == null:
		return
	var c: Dictionary = net.get_character()
	if c.is_empty():
		return
	header.text = "%s   Level %d" % [c["name"], c["level"]]
	points.text = "Stat points %d   Skill points %d" % [c["stat_points"], c["skill_points"]]
	for entry in STATS:
		var key: String = entry[0]
		var cost: int = c[key + "_cost"]
		stat_labels[key].text = str(c[key])
		var button: Button = stat_buttons[key]
		button.disabled = cost < 0 or c["stat_points"] < cost
		button.tooltip_text = "Costs %d stat points" % cost if cost >= 0 else "At the maximum"
	for key in value_labels:
		value_labels[key].text = str(c.get(key, ""))
