## Our buffs and debuffs: two rows of square icons with the time left under each, buffs
## on top and debuffs below. No panel behind them, in every theme.
extends VBoxContainer

const ICON := 30.0

var buffs: HBoxContainer
var debuffs: HBoxContainer


func _ready() -> void:
	mouse_filter = Control.MOUSE_FILTER_IGNORE
	add_theme_constant_override("separation", 4)
	buffs = HBoxContainer.new()
	buffs.add_theme_constant_override("separation", 4)
	buffs.mouse_filter = Control.MOUSE_FILTER_IGNORE
	add_child(buffs)
	debuffs = HBoxContainer.new()
	debuffs.add_theme_constant_override("separation", 4)
	debuffs.mouse_filter = Control.MOUSE_FILTER_IGNORE
	add_child(debuffs)
	custom_minimum_size = Vector2(ICON * 6, ICON * 2 + 30)


## effects: RoseNet.get_status_effects() of our character.
func refresh(effects: Array) -> void:
	var good := effects.filter(func(e): return not e.get("bad", false))
	var bad := effects.filter(func(e): return e.get("bad", false))
	_fill(buffs, good, false)
	_fill(debuffs, bad, true)


func _fill(row: HBoxContainer, effects: Array, bad: bool) -> void:
	while row.get_child_count() > effects.size():
		var last := row.get_child(row.get_child_count() - 1)
		row.remove_child(last)
		last.queue_free()
	while row.get_child_count() < effects.size():
		row.add_child(_Effect.new())
	for i in effects.size():
		row.get_child(i).show_effect(effects[i], bad)


class _Effect extends Control:
	var texture: Texture2D
	var seconds := 0.0
	var bad := false

	func _init() -> void:
		custom_minimum_size = Vector2(ICON, ICON + 14)
		mouse_filter = Control.MOUSE_FILTER_STOP

	func show_effect(effect: Dictionary, is_bad: bool) -> void:
		texture = effect.get("icon")
		seconds = float(effect.get("seconds", 0))
		bad = is_bad
		var tip: String = effect.get("name", "")
		var description: String = effect.get("description", "")
		if description != "":
			tip += "\n" + description
		tooltip_text = "%s\n%s left" % [tip, _time(seconds)]
		queue_redraw()

	static func _time(s: float) -> String:
		if s >= 3600.0:
			return "%dh" % int(s / 3600.0)
		if s >= 60.0:
			return "%dm" % int(s / 60.0)
		return "%ds" % int(s)

	func _draw() -> void:
		var rect := Rect2(Vector2.ZERO, Vector2(ICON, ICON))
		draw_rect(rect.grow(1.0), Color(0, 0, 0, 0.45))
		if texture:
			draw_texture_rect(texture, rect, false)
		var edge := UiBox.new()
		edge.radius = 3.0
		edge.colors = PackedColorArray([Color(0, 0, 0, 0)])
		edge.border_width = 1.5
		edge.border_color = get_theme_color("bad", "Ui") if bad else Color(1, 1, 1, 0.55)
		draw_style_box(edge, rect)
		var font := get_theme_font("bold_font", "Fonts")
		var text := _time(seconds)
		var fs := 11
		var w := font.get_string_size(text, HORIZONTAL_ALIGNMENT_LEFT, -1, fs).x
		var at := Vector2((ICON - w) * 0.5, ICON + 12.0)
		draw_string_outline(font, at, text, HORIZONTAL_ALIGNMENT_LEFT, -1, fs, 3, Color(0, 0, 0, 0.8))
		draw_string(font, at, text, HORIZONTAL_ALIGNMENT_LEFT, -1, fs, Color.WHITE)
