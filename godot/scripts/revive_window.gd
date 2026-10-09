## Shown while our character lies fallen, as iROSE's restart window: get up at the save
## point or at this zone's revive point. Until then a party member can still bring us back
## with Resurrection. After a long wait the character gets up by itself.
extends PanelContainer

var net: RoseNet
var info: Label
var save_button: Button
var here_button: Button


func _ready() -> void:
	# A full-screen layer: the world goes grey behind a card with the choices.
	theme_type_variation = "Clear"
	mouse_filter = Control.MOUSE_FILTER_IGNORE
	var shade := ColorRect.new()
	shade.material = ShaderMaterial.new()
	shade.material.shader = preload("res://shaders/ui_fallen.gdshader")
	shade.mouse_filter = Control.MOUSE_FILTER_IGNORE
	add_child(shade)
	var center := CenterContainer.new()
	center.mouse_filter = Control.MOUSE_FILTER_IGNORE
	add_child(center)
	var box := VBoxContainer.new()
	box.add_theme_constant_override("separation", 14)
	center.add_child(box)

	var title := Label.new()
	title.text = "You have fallen"
	title.theme_type_variation = "HeaderLabel"
	title.add_theme_font_size_override("font_size", 40)
	title.add_theme_color_override("font_color", Color(1, 0.93, 0.93))
	title.add_theme_color_override("font_outline_color", Color(0.15, 0.02, 0.05, 0.85))
	title.add_theme_constant_override("outline_size", 10)
	title.horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER
	box.add_child(title)
	info = Label.new()
	info.horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER
	info.add_theme_color_override("font_color", Color(1, 1, 1, 0.9))
	info.add_theme_color_override("font_outline_color", Color(0, 0, 0, 0.8))
	info.add_theme_constant_override("outline_size", 5)
	box.add_child(info)

	var card := PanelContainer.new()
	card.theme_type_variation = "QuestionPanel"
	card.size_flags_horizontal = Control.SIZE_SHRINK_CENTER
	card.custom_minimum_size.x = 340
	box.add_child(card)
	var buttons := VBoxContainer.new()
	buttons.add_theme_constant_override("separation", 8)
	card.add_child(buttons)
	save_button = Button.new()
	save_button.theme_type_variation = "ButtonPrimary"
	save_button.focus_mode = Control.FOCUS_NONE
	save_button.pressed.connect(func(): net.revive(true))
	buttons.add_child(save_button)
	here_button = Button.new()
	here_button.text = "Get up in this zone"
	here_button.focus_mode = Control.FOCUS_NONE
	here_button.pressed.connect(func(): net.revive(false))
	buttons.add_child(here_button)
	var hint := Label.new()
	hint.text = "A party member can still bring you back with Resurrection."
	hint.theme_type_variation = "MutedLabel"
	hint.horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER
	buttons.add_child(hint)


## Show or hide from our character's state (net.get_character()).
func refresh(c: Dictionary) -> void:
	visible = c.get("fallen", false)
	if not visible:
		return
	var lines := []
	var lost: int = c.get("penalty_xp", 0)
	if lost > 0:
		lines.append("Lost %d experience" % lost)
	var wait: int = c.get("auto_revive_in", 0)
	lines.append("Getting up by yourself in %d:%02d" % [wait / 60, wait % 60])
	info.text = "\n".join(lines)
	var save: String = c.get("save_zone", "")
	save_button.visible = save != ""
	save_button.text = "Get up at the save point (%s)" % save
