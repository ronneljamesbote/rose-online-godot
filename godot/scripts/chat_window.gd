## The chat box, bottom left. Enter starts typing and Enter sends; Escape stops typing.
## Like iROSE, a line starting with ! shouts to the zone, # talks to the party and
## @name whispers to one player; anything else is heard by players nearby.
extends PanelContainer

const MAX_LINES := 100
const COLOURS := {  # rose-offline-client's chat colours
	"nearby": Color8(255, 255, 255),
	"shout": Color8(189, 250, 255),
	"party": Color8(255, 237, 140),
	"whisper": Color8(201, 255, 144),
	"system": Color8(255, 224, 229),
}

var net: RoseNet
var log_label: RichTextLabel
var input: LineEdit
var _lines := 0
var _last_whisper_from := ""
var _history: Array = []  # [text, channel], to colour the log again when the theme changes


func _ready() -> void:
	custom_minimum_size = Vector2(420, 0)
	mouse_filter = Control.MOUSE_FILTER_IGNORE
	theme_type_variation = "ChatPanel"
	var box := VBoxContainer.new()
	box.mouse_filter = Control.MOUSE_FILTER_IGNORE
	add_child(box)
	log_label = RichTextLabel.new()
	log_label.custom_minimum_size = Vector2(408, 150)
	log_label.scroll_following = true
	log_label.selection_enabled = false
	log_label.add_theme_font_size_override("normal_font_size", 14)
	UI.theme_changed.connect(_restyle)
	_restyle()
	box.add_child(log_label)
	input = LineEdit.new()
	input.placeholder_text = "Enter to chat   ! shout   # party   @name whisper"
	input.max_length = 220
	input.context_menu_enabled = false
	input.text_submitted.connect(_submit)
	input.text_changed.connect(_colour_input)
	box.add_child(input)


## Start typing (Enter outside the box).
func focus_input() -> void:
	input.grab_focus()


func is_typing() -> bool:
	return input.has_focus()


func _submit(text: String) -> void:
	if text.strip_edges() != "":
		say(text)
	input.clear()
	_colour_input("")
	input.release_focus()


## Send a line ("/r text" answers the last whisper).
func say(text: String) -> void:
	if text.begins_with("/r ") and _last_whisper_from != "":
		text = "@%s %s" % [_last_whisper_from, text.substr(3)]
	net.send_chat(text)


## A message from RoseNet.poll_chat().
func add_message(m: Dictionary) -> void:
	var channel: String = m["channel"]
	var prefix := ""
	match channel:
		"shout":
			prefix = "[Shout] %s> " % m["from"]
		"party":
			prefix = "[Party] %s> " % m["from"]
		"whisper":
			if m["mine"]:
				prefix = "[To %s] " % m["to"]
			else:
				prefix = "[From %s] " % m["from"]
				_last_whisper_from = m["from"]
		_:
			prefix = "%s> " % m["from"]
	add_line(prefix + String(m["text"]), channel)


## Add a line to the log in a channel's colour. Text is added as plain text, so nothing a
## player types is read as markup.
func add_line(text: String, channel := "system") -> void:
	_history.append([text, channel])
	if _history.size() > MAX_LINES:
		_history.pop_front()
	if _lines > 0:
		log_label.newline()
	log_label.push_color(_colour(channel))
	log_label.add_text(text)
	log_label.pop()
	_lines += 1
	if _lines > MAX_LINES:
		log_label.remove_paragraph(0)
		_lines -= 1


## A channel's colour in the current theme.
func _colour(channel: String) -> Color:
	var key: String = {"nearby": "say", "shout": "shout", "party": "party", "whisper": "whisper"}.get(channel, "system")
	return UI.color("chat." + key, COLOURS.get(channel, Color.WHITE))


func _restyle() -> void:
	var outline := UI.color("chat.outline", Color(0, 0, 0, 0))
	log_label.add_theme_color_override("font_outline_color", outline)
	log_label.add_theme_constant_override("outline_size", 3 if outline.a > 0.0 else 0)
	if not _history.is_empty():
		var lines := _history
		_history = []
		log_label.clear()
		_lines = 0
		for line in lines:
			add_line(line[0], line[1])


func _colour_input(text: String) -> void:
	var channel := "nearby"
	if text.begins_with("!"):
		channel = "shout"
	elif text.begins_with("#"):
		channel = "party"
	elif text.begins_with("@") or text.begins_with("/r "):
		channel = "whisper"
	input.add_theme_color_override("font_color", _colour(channel) if channel != "nearby" else UI.color("field.text"))


func _input(event: InputEvent) -> void:
	if not input.has_focus():
		return
	if event is InputEventKey and event.pressed and event.keycode == KEY_ESCAPE:
		input.release_focus()
		get_viewport().set_input_as_handled()
	elif event is InputEventMouseButton and event.pressed and not input.get_global_rect().has_point(event.position):
		# Clicking the world stops typing (the click still goes through).
		input.release_focus()
