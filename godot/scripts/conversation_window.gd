## NPC conversation: the NPC's message and the answers to pick from (click one, or press
## 1-9). What an answer does (quests, opening the store) comes from the NPC's script.
extends PanelContainer

const TALK_RANGE := 15.0  # metres; walking further away ends the conversation

var net: RoseNet
var online: Node  # online.gd, for our character
var npc: Node3D
var title: Label
var message: RichTextLabel
var answers: RichTextLabel
var _last := {}


func _ready() -> void:
	custom_minimum_size = Vector2(420, 0)
	var margin := MarginContainer.new()
	for side in ["left", "right", "top", "bottom"]:
		margin.add_theme_constant_override("margin_" + side, 12)
	add_child(margin)
	var box := VBoxContainer.new()
	box.add_theme_constant_override("separation", 8)
	margin.add_child(box)

	var top := HBoxContainer.new()
	box.add_child(top)
	title = Label.new()
	title.theme_type_variation = "HeaderLabel"
	title.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	top.add_child(title)
	var close := Button.new()
	close.text = "Close"
	close.focus_mode = Control.FOCUS_NONE
	close.pressed.connect(close_conversation)
	top.add_child(close)

	message = RichTextLabel.new()
	message.bbcode_enabled = true
	message.fit_content = true
	message.scroll_active = false
	message.custom_minimum_size = Vector2(396, 0)
	box.add_child(message)
	box.add_child(HSeparator.new())
	answers = RichTextLabel.new()
	answers.bbcode_enabled = true
	answers.fit_content = true
	answers.scroll_active = false
	answers.meta_underlined = false
	answers.custom_minimum_size = Vector2(396, 0)
	answers.add_theme_constant_override("line_separation", 6)
	answers.meta_clicked.connect(func(meta): choose(int(str(meta))))
	box.add_child(answers)


## Talk to this NPC entity. False when it has nothing to say.
func open(entity_id: int, npc_node: Node3D) -> bool:
	if not net.open_conversation(entity_id):
		return false
	npc = npc_node
	_last = {}
	_refresh()
	return visible


func close_conversation() -> void:
	net.close_conversation()
	visible = false
	npc = null


func choose(index: int) -> void:
	net.choose_response(index)
	_refresh()


func _refresh() -> void:
	var d: Dictionary = net.get_conversation()
	if not d.get("open", false):
		visible = false
		npc = null
		return
	if d == _last:
		return
	_last = d
	title.text = d["title"]
	message.text = d["message"]
	var lines := []
	var responses: Array = d["responses"]
	for i in responses.size():
		lines.append("[url=%d][color=#%s]%d.[/color] %s[/url]" % [i, UI.color("colors.accent").to_html(false), i + 1, responses[i]])
	answers.text = "\n".join(lines)
	answers.visible = not responses.is_empty()
	visible = true
	reset_size()


func _process(_delta: float) -> void:
	if not visible:
		return
	var me: Node3D = online.me if online else null
	if npc == null or not is_instance_valid(npc) or me == null or me.position.distance_to(npc.position) > TALK_RANGE:
		close_conversation()
		return
	_refresh()


func _unhandled_key_input(event: InputEvent) -> void:
	if not visible or not event is InputEventKey or not event.pressed or event.echo:
		return
	if event.keycode == KEY_ESCAPE:
		close_conversation()
		get_viewport().set_input_as_handled()
	elif event.keycode >= KEY_1 and event.keycode <= KEY_9:
		var index: int = event.keycode - KEY_1
		if index < _last.get("responses", []).size():
			choose(index)
			get_viewport().set_input_as_handled()
