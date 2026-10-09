## Shown while our character lies fallen, as iROSE's restart window: get up at the save
## point or at this zone's revive point. Until then a party member can still bring us back
## with Resurrection. After a long wait the character gets up by itself.
extends PanelContainer

var net: RoseNet
var info: Label
var save_button: Button
var here_button: Button


func _ready() -> void:
	custom_minimum_size = Vector2(300, 0)
	var margin := MarginContainer.new()
	for side in ["left", "right", "top", "bottom"]:
		margin.add_theme_constant_override("margin_" + side, 10)
	add_child(margin)
	var box := VBoxContainer.new()
	box.add_theme_constant_override("separation", 6)
	margin.add_child(box)

	var title := Label.new()
	title.text = "You have fallen"
	title.theme_type_variation = "HeaderLabel"
	title.horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER
	box.add_child(title)
	info = Label.new()
	info.horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER
	box.add_child(info)

	save_button = Button.new()
	save_button.focus_mode = Control.FOCUS_NONE
	save_button.pressed.connect(func(): net.revive(true))
	box.add_child(save_button)
	here_button = Button.new()
	here_button.text = "Get up in this zone"
	here_button.focus_mode = Control.FOCUS_NONE
	here_button.pressed.connect(func(): net.revive(false))
	box.add_child(here_button)


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
