## A themed bar with its numbers on it (HP 134 / 134). Life bars keep a lighter "damage
## taken" part that catches up a moment after a hit.
class_name UiBar
extends Control

## hp, mp, stamina, xp or cast: picks the bar colour from the theme.
var kind := "hp"
var value := 0.0
var max_value := 1.0
## Text on the bar; "" shows "value / max".
var text := ""
var show_text := true
var height_scale := 1.0
var _lag := 0.0
var _lag_wait := 0.0


func _init(bar_kind := "hp") -> void:
	kind = bar_kind
	mouse_filter = Control.MOUSE_FILTER_IGNORE


func _ready() -> void:
	UI.theme_changed.connect(_restyle)
	_restyle()


func _restyle() -> void:
	custom_minimum_size.y = roundf(UI.num("bars.height", 14) * height_scale)
	queue_redraw()


func set_values(now: float, most: float, label := "") -> void:
	most = maxf(most, 1.0)
	if now < value and kind == "hp":
		_lag = maxf(_lag, value / max_value)
		_lag_wait = 0.45
	if now != value or most != max_value or label != text:
		value = now
		max_value = most
		text = label
		queue_redraw()


func _process(delta: float) -> void:
	if _lag <= 0.0:
		return
	if _lag_wait > 0.0:
		_lag_wait -= delta
		return
	_lag = move_toward(_lag, value / max_value, delta * 0.8)
	if _lag <= value / max_value + 0.001:
		_lag = 0.0
	queue_redraw()


func _draw() -> void:
	var back := get_theme_stylebox("background", "ProgressBar")
	draw_style_box(back, Rect2(Vector2.ZERO, size))
	var fill: UiBox = get_theme_stylebox("fill", "ProgressBar").duplicate()
	var c := get_theme_color(kind, "Bars")
	var part := clampf(value / max_value, 0.0, 1.0)
	if _lag > part:
		var lag: UiBox = fill.duplicate()
		var lc := get_theme_color("hp_lag", "Bars")
		lag.colors = PackedColorArray([lc, lc])
		lag.stops = PackedFloat32Array([0.0, 1.0])
		lag.sheen = 0.0
		draw_style_box(lag, Rect2(Vector2.ZERO, Vector2(size.x * _lag, size.y)))
	if part > 0.0:
		fill.colors = PackedColorArray([c.lightened(0.12), c])
		fill.stops = PackedFloat32Array([0.0, 1.0])
		draw_style_box(fill, Rect2(Vector2.ZERO, Vector2(maxf(size.x * part, minf(size.y, size.x * part * 4.0)), size.y)))
	if show_text:
		var label := text if text != "" else "%d / %d" % [int(value), int(max_value)]
		var font := get_theme_font("font", "BarLabel")
		var font_size := get_theme_font_size("font_size", "BarLabel")
		var w := font.get_string_size(label, HORIZONTAL_ALIGNMENT_LEFT, -1, font_size).x
		var at := Vector2((size.x - w) * 0.5, (size.y + font.get_ascent(font_size) - font.get_descent(font_size)) * 0.5)
		draw_string_outline(font, at, label, HORIZONTAL_ALIGNMENT_LEFT, -1, font_size, 3, get_theme_color("font_outline_color", "BarLabel"))
		draw_string(font, at, label, HORIZONTAL_ALIGNMENT_LEFT, -1, font_size, get_theme_color("font_color", "BarLabel"))
