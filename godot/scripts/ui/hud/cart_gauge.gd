## Fuel while driving a cart or castle gear.
extends PanelContainer

var bar: UiBar


func _ready() -> void:
	theme_type_variation = "HudPanel"
	mouse_filter = Control.MOUSE_FILTER_IGNORE
	var column := VBoxContainer.new()
	add_child(column)
	var title := Label.new()
	title.text = "Fuel"
	title.theme_type_variation = "SmallLabel"
	column.add_child(title)
	bar = UiBar.new("stamina")
	bar.custom_minimum_size.x = 180
	column.add_child(bar)


func refresh(c: Dictionary) -> void:
	visible = c.get("driving", false)
	if visible:
		var fuel := int(c.get("fuel", 0))
		bar.set_values(fuel, 1000, "%d%%" % (fuel / 10))
