## GM commands in the chat box, for dev servers. A line starting with / (other than /r)
## goes to the server's gm reducer instead of chat; while typing one, the matching
## commands show above the box and Tab completes the first. Answers come back as a notice
## with lines starting with "GM: " and are shown in the chat log.
##
## This folder is left out of the production export (export_presets.cfg), so a
## production client has no GM commands. chat_window.gd loads this script when it exists.
extends Node

const Commands := preload("res://gm/gm_commands.gd")
const MAX_SUGGESTIONS := 6

var chat: PanelContainer  # chat_window.gd
var hint: PanelContainer
var hint_label: RichTextLabel


func _init(chat_window: PanelContainer) -> void:
	chat = chat_window
	name = "GmChat"


func _ready() -> void:
	hint = PanelContainer.new()
	hint.theme_type_variation = "Tooltip"
	hint.mouse_filter = Control.MOUSE_FILTER_IGNORE
	hint.top_level = true
	hint.visible = false
	hint_label = RichTextLabel.new()
	hint_label.bbcode_enabled = true
	hint_label.fit_content = true
	hint_label.scroll_active = false
	hint_label.autowrap_mode = TextServer.AUTOWRAP_OFF
	hint_label.mouse_filter = Control.MOUSE_FILTER_IGNORE
	hint_label.add_theme_font_size_override("normal_font_size", 13)
	hint_label.add_theme_font_size_override("bold_font_size", 13)
	hint.add_child(hint_label)
	chat.add_child(hint)
	chat.input.text_changed.connect(_suggest)
	chat.input.focus_exited.connect(func(): hint.visible = false)
	chat.input.placeholder_text = "Enter to chat  ! shout  # party  @name whisper  / GM"


## A line for the server, or false to send it as chat.
func handles(text: String) -> bool:
	if not text.begins_with("/") or text.begins_with("/r "):
		return false
	chat.add_line(text, "gm")
	if not chat.net.has_method("gm_command"):
		chat.add_line("GM: this client was built without GM commands (build rose-gd with --features dev)", "gm")
		return true
	var target: int = chat.current_target.call() if chat.current_target.is_valid() else -1
	chat.net.gm_command(text, target)
	return true


## A notice from the server: GM answers (a line each) go to the chat log.
func take_notice(text: String) -> bool:
	if not text.begins_with("GM: "):
		return false
	for line in text.split("\n", false):
		chat.add_line(line, "gm")
	return true


func _suggest(text: String) -> void:
	if not text.begins_with("/") or text.begins_with("/r ") or text.length() > 60:
		hint.visible = false
		return
	var found: Array = Commands.matching(text)
	if found.is_empty():
		hint.visible = false
		return
	var muted := UI.color("colors.muted", Color(1, 1, 1, 0.6)).to_html()
	var lines := []
	for c in found.slice(0, MAX_SUGGESTIONS):
		lines.append("[b]/%s[/b] %s  [color=#%s]%s[/color]" % [c[0], c[1], muted, c[2]])
	if found.size() > MAX_SUGGESTIONS:
		lines.append("[color=#%s]… %d more (/help)[/color]" % [muted, found.size() - MAX_SUGGESTIONS])
	hint_label.add_theme_color_override("default_color", UI.color("tooltip.text", Color.WHITE))
	hint_label.text = "\n".join(lines)
	hint.visible = true
	hint.reset_size()
	var input_rect: Rect2 = chat.input.get_global_rect()
	hint.global_position = input_rect.position - Vector2(0, hint.get_combined_minimum_size().y + 4)


func _input(event: InputEvent) -> void:
	# Tab completes the first matching command.
	if not chat.input.has_focus() or not (event is InputEventKey) or not event.pressed or event.keycode != KEY_TAB:
		return
	var text: String = chat.input.text
	if text.begins_with("/") and not text.contains(" "):
		var found: Array = Commands.matching(text)
		if not found.is_empty():
			chat.input.text = "/%s " % found[0][0]
			chat.input.caret_column = chat.input.text.length()
			_suggest(chat.input.text)
	get_viewport().set_input_as_handled()
