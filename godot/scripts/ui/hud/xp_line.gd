## Experience under the hotbar: a thin bar with the numbers and unspent points.
extends VBoxContainer

var bar: UiBar


func _ready() -> void:
	mouse_filter = Control.MOUSE_FILTER_IGNORE
	bar = UiBar.new("xp")
	bar.custom_minimum_size.x = 420
	add_child(bar)


func refresh(c: Dictionary) -> void:
	if c.is_empty():
		return
	var needed: int = max(int(c["xp_needed"]), 1)
	var text := "XP  %d / %d  (%.1f%%)" % [c["xp"], needed, 100.0 * c["xp"] / needed]
	if int(c.get("stat_points", 0)) > 0:
		text += "   ·   %d stat points (C)" % c["stat_points"]
	if int(c.get("skill_points", 0)) > 0:
		text += "   ·   %d skill points (K)" % c["skill_points"]
	bar.set_values(float(c["xp"]), float(needed), text)
