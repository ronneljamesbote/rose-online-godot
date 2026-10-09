## Party members under our frame: name, level and life bar each. Hidden without a party.
extends VBoxContainer

var _rows := {}  # identity -> [panel, name label, bar]


func _ready() -> void:
	mouse_filter = Control.MOUSE_FILTER_IGNORE
	add_theme_constant_override("separation", 4)
	custom_minimum_size.x = 200


## party: RoseNet.get_party().
func refresh(party: Dictionary) -> void:
	var members: Array = party.get("members", []).filter(func(m): return not m.get("me", false))
	var seen := {}
	for m in members:
		var key: String = m.get("identity", "")
		seen[key] = true
		if not _rows.has(key):
			var panel := PanelContainer.new()
			panel.theme_type_variation = "HudPanel"
			var column := VBoxContainer.new()
			column.add_theme_constant_override("separation", 2)
			panel.add_child(column)
			var name_label := Label.new()
			name_label.theme_type_variation = "SmallLabel"
			column.add_child(name_label)
			var bar := UiBar.new("hp")
			bar.height_scale = 0.85
			column.add_child(bar)
			add_child(panel)
			_rows[key] = [panel, name_label, bar]
		var row: Array = _rows[key]
		row[1].text = "%s%s  Lv %d%s" % ["★ " if m.get("leader", false) else "", m.get("name", "?"), m.get("level", 0), "" if m.get("online", true) else "  (away)"]
		row[2].set_values(int(m.get("hp", 0)), int(m.get("max_hp", 1)), "%d / %d" % [m.get("hp", 0), m.get("max_hp", 1)])
		row[0].modulate.a = 1.0 if m.get("online", true) else 0.55
	for key in _rows.keys():
		if not seen.has(key):
			_rows[key][0].queue_free()
			_rows.erase(key)
	visible = not _rows.is_empty()
