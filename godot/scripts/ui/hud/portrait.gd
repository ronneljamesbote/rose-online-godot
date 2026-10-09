## Round portrait with an initial and a level badge, for the player and target frames.
extends Control

var letter := ""
var level := 0
var tint := Color(0, 0, 0, 0)  # target colour (enemy, friendly), drawn as the ring when set


func _init(diameter := 52.0) -> void:
	custom_minimum_size = Vector2(diameter, diameter)
	mouse_filter = Control.MOUSE_FILTER_IGNORE


func _ready() -> void:
	UI.theme_changed.connect(queue_redraw)


func set_values(new_letter: String, new_level: int, new_tint := Color(0, 0, 0, 0)) -> void:
	if new_letter != letter or new_level != level or new_tint != tint:
		letter = new_letter
		level = new_level
		tint = new_tint
		queue_redraw()


func _draw() -> void:
	var r := minf(size.x, size.y) * 0.5
	var c := Vector2(r, r)
	draw_circle(c + Vector2(0, 2), r, Color(0, 0, 0, 0.25), true, -1.0, true)
	# Radial gradient: rings from the edge colour to the centre colour, lit from above.
	var edge := UI.color("portrait.edge")
	var centre := UI.color("portrait.center")
	for i in 12:
		var t := float(i) / 11.0
		draw_circle(c - Vector2(0, r * 0.25 * t), r * (1.0 - t * 0.8), edge.lerp(centre, t), true, -1.0, true)
	var ring := tint if tint.a > 0.0 else UI.color("portrait.border_color")
	draw_arc(c, r - 1.0, 0, TAU, 48, ring, maxf(UI.num("portrait.border_width", 2), 1.5), true)
	var font := get_theme_font("bold_font", "Fonts")
	var fs := int(r * 0.9)
	var w := font.get_string_size(letter, HORIZONTAL_ALIGNMENT_LEFT, -1, fs).x
	var text_colour := UI.color("colors.title")
	draw_string(font, Vector2(c.x - w * 0.5, c.y + fs * 0.35), letter, HORIZONTAL_ALIGNMENT_LEFT, -1, fs, text_colour)
	if level > 0:
		var badge := str(level)
		var bfs := 11
		var bw := maxf(font.get_string_size(badge, HORIZONTAL_ALIGNMENT_LEFT, -1, bfs).x + 10.0, 20.0)
		var rect := Rect2(Vector2(c.x - bw * 0.5, size.y - 11.0), Vector2(bw, 16))
		var box := UiBox.make(UI.color("level.top"), UI.color("level.bottom"), 8)
		box.border_width = 1.5
		box.border_color = UI.color("level.border_color")
		box.shadow_color = Color(0, 0, 0, 0.3)
		box.shadow_size = 3
		box.shadow_offset = 1
		draw_style_box(box, rect)
		draw_string(font, Vector2(rect.position.x + (bw - font.get_string_size(badge, HORIZONTAL_ALIGNMENT_LEFT, -1, bfs).x) * 0.5, rect.position.y + 12.0), badge, HORIZONTAL_ALIGNMENT_LEFT, -1, bfs, UI.color("level.text"))
