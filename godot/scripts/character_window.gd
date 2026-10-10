## Character window (C): level, experience, basic stats with buttons to spend stat points,
## and the ability values the server calculated from them.
extends PanelContainer

const STATS := [["str", "Strength"], ["dex", "Dexterity"], ["int", "Intelligence"],
	["con", "Concentration"], ["cha", "Charm"], ["sen", "Sense"]]
const VALUES := [["attack", "Attack"], ["defence", "Defence"], ["hit", "Hit"], ["avoid", "Avoid"],
	["critical", "Critical"], ["resistance", "Magic resist"], ["attack_speed", "Attack speed"],
	["stamina", "Stamina (of 5000)"]]

const HintLabel := preload("res://scripts/ui/hint_label.gd")

## What each basic stat and value does, shown when pointing at it. The numbers follow the
## server's formulas (crates/rose-game-irose/src/data/ability_values.rs; wiki/rules/stats.md).
const HINTS := {
	"str": "Raises attack with melee weapons, max HP, defence and how much you can carry.\nEach point: +0.75 attack with one- and two-handed melee weapons (a little more with a strong weapon), +2 max HP, +0.35 defence and +6 max weight.",
	"dex": "Raises attack with bows, crossbows, dual swords and katars, avoid and run speed.\nEach point: +0.62 attack with bows and crossbows, +0.45 with dual swords, +0.55 with katars and +0.76 avoid. Every 5 points make you run about 1% faster.",
	"int": "Raises max MP, magic resistance, attack with staffs and wands, magic skill damage and healing.\nEach point: +4 max MP, +0.6 magic resistance, +0.6 attack with wands, +0.4 with staffs, and heals you cast get about 0.3% stronger.",
	"con": "Raises hit, critical, attack with guns and launchers, and how fast HP and MP come back.\nEach point: +0.8 hit (+0.5 with no weapon), +0.2 critical and +0.5 attack with guns and launchers. Every 10 points give about 3 more HP and 3 more MP every 4 seconds while sitting.",
	"cha": "Raises quest rewards and what monsters drop for you. No combat value uses it.\nMany quest rewards of experience and Zuly grow with Charm, and equipment you get from kills is more likely to have a higher grade or an extra option.",
	"sen": "Raises critical, attack with bows, guns, launchers and wands, and the damage of skills.\nEach point: +1 critical. Ranged weapons and wands also hit harder with more Sense, the more so the stronger the weapon, and attack and magic skills do more damage.",
	"attack": "How hard your normal attacks hit. It comes from your weapon, your level and your stats: STR for melee weapons, DEX for bows, INT for staffs and wands.\nDamage weighs it against the target's defence, or its magic resistance for staffs and wands.",
	"defence": "Lowers the damage you take from physical attacks.\nIt comes from your armour and shield, STR and level.",
	"hit": "How likely your attacks are to land. It is weighed against the target's avoid.\nIt comes from CON and your weapon's quality and durability.",
	"avoid": "How likely you are to dodge an attack. It also takes a little off the damage of attacks that land.\nIt comes from DEX, level and the durability and grade of what you wear.",
	"critical": "How likely a hit is to be critical, for much more damage.\nIt comes from SEN and CON. At level 1, 17 critical is about a 25% chance, 50 about 39%.",
	"resistance": "Lowers the damage you take from magic: staffs, wands, magic monsters and spells.\nIt comes from your armour, INT and level.",
	"attack_speed": "How fast you swing. At 100 an attack plays at its normal speed; higher is faster, so 200 swings twice as often.\nIt comes from your weapon (lower weapon speed is faster) and some passive skills.",
	"stamina": "Some scrolls and charms spend stamina instead of MP.\nYou earn it from monster kills (much more at low levels) and from Vital Jam and Stamina items. It stops at 5000.",
	"stat_points": "You get stat points every level. Press + next to a stat to spend them.\nRaising a stat by 1 costs its current value divided by 5, so higher stats cost more.",
	"skill_points": "You get skill points as you level up. Spend them in the skill window (K) to learn skills and raise their level.",
}

## Job ids (the Job ability value) to names.
const JOBS := {0: "Visitor", 111: "Soldier", 121: "Knight", 122: "Champion", 211: "Muse", 221: "Mage",
	222: "Cleric", 311: "Hawker", 321: "Raider", 322: "Scout", 411: "Dealer", 421: "Bourgeois", 422: "Artisan"}

var net: RoseNet
var header: Label
var points: Label
var skill_points: Label
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
	header.theme_type_variation = "HeaderLabel"
	box.add_child(header)
	var point_row := HBoxContainer.new()
	point_row.add_theme_constant_override("separation", 16)
	box.add_child(point_row)
	points = _hint_label("Stat points", HINTS["stat_points"])
	point_row.add_child(points)
	skill_points = _hint_label("Skill points", HINTS["skill_points"])
	point_row.add_child(skill_points)
	box.add_child(HSeparator.new())

	var grid := GridContainer.new()
	grid.columns = 3
	box.add_child(grid)
	for i in STATS.size():
		var key: String = STATS[i][0]
		var name_label := _hint_label(STATS[i][1], HINTS[key])
		name_label.text = STATS[i][1]
		name_label.size_flags_horizontal = Control.SIZE_EXPAND_FILL
		grid.add_child(name_label)
		var value := _hint_label(STATS[i][1], HINTS[key])
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
		var title: String = "Stamina" if entry[0] == "stamina" else entry[1]
		var name_label := _hint_label(title, HINTS[entry[0]])
		name_label.text = entry[1]
		name_label.size_flags_horizontal = Control.SIZE_EXPAND_FILL
		values.add_child(name_label)
		var value := _hint_label(title, HINTS[entry[0]])
		value.horizontal_alignment = HORIZONTAL_ALIGNMENT_RIGHT
		values.add_child(value)
		value_labels[entry[0]] = value


func _hint_label(title: String, hint: String) -> Label:
	var label := HintLabel.new()
	label.hint_title = title
	label.hint = hint
	return label


func _process(_delta: float) -> void:
	if not visible or net == null:
		return
	var c: Dictionary = net.get_character()
	if c.is_empty():
		return
	header.text = "%s   Level %d   %s" % [c["name"], c["level"], JOBS.get(c.get("job", 0), "Visitor")]
	points.text = "Stat points %d" % c["stat_points"]
	skill_points.text = "Skill points %d" % c["skill_points"]
	for entry in STATS:
		var key: String = entry[0]
		var cost: int = c[key + "_cost"]
		stat_labels[key].text = str(c[key])
		var button: Button = stat_buttons[key]
		button.disabled = cost < 0 or c["stat_points"] < cost
		button.tooltip_text = "Costs %d stat points" % cost if cost >= 0 else "At the maximum"
	for key in value_labels:
		value_labels[key].text = str(c.get(key, ""))
