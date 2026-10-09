## The menu, bottom right: one button per window with its key.
extends PanelContainer

## icon, name, key, method on online.gd / main.gd that toggles the window
const ENTRIES := [
	["character", "Character", "C"],
	["inventory", "Inventory", "I"],
	["skills", "Skills", "K"],
	["quests", "Quests", "Q"],
	["party", "Party", "P"],
	["friends", "Friends", "F"],
	["map", "Map", "M"],
	["options", "Options", "O"],
]

signal open(which: String)


func _ready() -> void:
	theme_type_variation = "HudPanel"
	mouse_filter = Control.MOUSE_FILTER_STOP
	var row := HBoxContainer.new()
	row.add_theme_constant_override("separation", 2)
	add_child(row)
	for entry in ENTRIES:
		var b := _MenuButton.new()
		b.icon_name = entry[0]
		b.tooltip_text = "%s (%s)" % [entry[1], entry[2]]
		b.theme_type_variation = "FlatButton"
		b.focus_mode = Control.FOCUS_NONE
		b.custom_minimum_size = Vector2(34, 34)
		var which: String = entry[0]
		b.pressed.connect(func(): open.emit(which))
		row.add_child(b)


## A flat button with a small line icon drawn in the theme's menu colour.
class _MenuButton extends Button:
	var icon_name := ""

	func _draw() -> void:
		var c := UI.color("menu.icon")
		var o := size * 0.5
		var w := 1.6
		match icon_name:
			"character":
				draw_arc(o + Vector2(0, -4), 4.5, 0, TAU, 20, c, w, true)
				draw_arc(o + Vector2(0, 11), 9.0, PI * 1.15, PI * 1.85, 16, c, w, true)
			"inventory":
				var bag := Rect2(o + Vector2(-7, -4), Vector2(14, 12))
				draw_rect(bag, c, false, w)
				draw_arc(o + Vector2(0, -4), 4.0, PI, TAU, 12, c, w, true)
				draw_line(o + Vector2(-7, 0), o + Vector2(7, 0), c, w, true)
			"skills":
				var star := PackedVector2Array()
				for i in 11:
					var r := 8.5 if i % 2 == 0 else 3.8
					star.append(o + Vector2(0, -r).rotated(TAU * i / 10.0))
				draw_polyline(star, c, w, true)
			"quests":
				draw_rect(Rect2(o + Vector2(-6, -8), Vector2(12, 16)), c, false, w)
				for y in [-3.0, 1.0, 5.0]:
					draw_line(o + Vector2(-3, y), o + Vector2(3, y), c, w, true)
				draw_line(o + Vector2(-3, -6), o + Vector2(1, -6), c, w, true)
			"party":
				draw_arc(o + Vector2(-4, -3), 3.5, 0, TAU, 16, c, w, true)
				draw_arc(o + Vector2(5, -2), 3.0, 0, TAU, 16, c, w, true)
				draw_arc(o + Vector2(-4, 10), 7.0, PI * 1.15, PI * 1.85, 12, c, w, true)
				draw_arc(o + Vector2(5, 10), 6.0, PI * 1.2, PI * 1.8, 12, c, w, true)
			"friends":
				var heart := PackedVector2Array()
				for i in 33:
					var t := TAU * i / 32.0
					heart.append(o + Vector2(16 * pow(sin(t), 3), -(13 * cos(t) - 5 * cos(2 * t) - 2 * cos(3 * t) - cos(4 * t))) * 0.45 + Vector2(0, -1))
				draw_polyline(heart, c, w, true)
			"map":
				var pts := PackedVector2Array([o + Vector2(-8, -6), o + Vector2(-3, -8), o + Vector2(3, -6), o + Vector2(8, -8),
					o + Vector2(8, 6), o + Vector2(3, 8), o + Vector2(-3, 6), o + Vector2(-8, 8), o + Vector2(-8, -6)])
				draw_polyline(pts, c, w, true)
				draw_line(o + Vector2(-3, -8), o + Vector2(-3, 6), c, w * 0.8, true)
				draw_line(o + Vector2(3, -6), o + Vector2(3, 8), c, w * 0.8, true)
			"options":
				draw_arc(o, 3.5, 0, TAU, 16, c, w, true)
				for i in 8:
					var d := Vector2(0, -1).rotated(TAU * i / 8.0)
					draw_line(o + d * 5.5, o + d * 8.5, c, w * 1.4, true)
				draw_arc(o, 6.0, 0, TAU, 24, c, w, true)
