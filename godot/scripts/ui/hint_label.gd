## A label that explains itself when pointed at: a tooltip with a title and a few wrapped
## lines, in the theme's tooltip style. Set hint_title and hint (empty hint = no tooltip).
extends Label

const WIDTH := 290.0

var hint_title := ""
var hint := "":
	set(value):
		hint = value
		tooltip_text = value
		mouse_filter = Control.MOUSE_FILTER_PASS if value != "" else Control.MOUSE_FILTER_IGNORE


func _make_custom_tooltip(for_text: String) -> Object:
	var column := VBoxContainer.new()
	column.add_theme_constant_override("separation", 4)
	if hint_title != "":
		var title := Label.new()
		title.theme_type_variation = "HeaderLabel"
		title.text = hint_title
		title.add_theme_color_override("font_color", UI.color("colors.accent", UI.color("tooltip.text")))
		title.add_theme_color_override("font_outline_color", Color(0, 0, 0, 0))
		column.add_child(title)
	for paragraph in for_text.split("\n"):
		var body := Label.new()
		body.theme_type_variation = "TooltipLabel"
		body.text = paragraph
		body.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
		body.custom_minimum_size.x = WIDTH
		body.add_theme_constant_override("line_spacing", -2)
		body.add_theme_color_override("font_color", UI.color("tooltip.text"))
		body.add_theme_color_override("font_outline_color", Color(0, 0, 0, 0))
		column.add_child(body)
	return column
