## Friends list (F), as iROSE's messenger: who is online and where, a box to add a friend
## by name, and Whisper / Remove on each row. Friend requests show in a small panel at the
## top to accept or decline.
extends PanelContainer

const CharacterWindow := preload("res://scripts/character_window.gd")

var net: RoseNet
var chat_window: PanelContainer
var rows: VBoxContainer
var name_box: LineEdit
var count_label: Label
var request_panel: PanelContainer  # added to the HUD layer by online.gd
var request_label: Label
var _request_id := -1
var _last := []


func _ready() -> void:
	custom_minimum_size = Vector2(300, 0)
	var margin := MarginContainer.new()
	for side in ["left", "right", "top", "bottom"]:
		margin.add_theme_constant_override("margin_" + side, 8)
	add_child(margin)
	var box := VBoxContainer.new()
	box.add_theme_constant_override("separation", 4)
	margin.add_child(box)

	var top := HBoxContainer.new()
	box.add_child(top)
	var title := Label.new()
	title.text = "Friends"
	title.theme_type_variation = "HeaderLabel"
	title.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	top.add_child(title)
	count_label = Label.new()
	count_label.modulate = Color(1, 1, 1, 0.6)
	top.add_child(count_label)

	rows = VBoxContainer.new()
	box.add_child(rows)

	var add := HBoxContainer.new()
	box.add_child(add)
	name_box = LineEdit.new()
	name_box.placeholder_text = "Name"
	name_box.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	name_box.text_submitted.connect(func(_t): _add())
	add.add_child(name_box)
	var add_button := Button.new()
	add_button.text = "Add friend"
	add_button.focus_mode = Control.FOCUS_NONE
	add_button.pressed.connect(_add)
	add.add_child(add_button)

	request_panel = PanelContainer.new()
	request_panel.visible = false
	var rbox := VBoxContainer.new()
	request_panel.add_child(rbox)
	request_label = Label.new()
	rbox.add_child(request_label)
	var buttons := HBoxContainer.new()
	buttons.alignment = BoxContainer.ALIGNMENT_CENTER
	rbox.add_child(buttons)
	for answer in [true, false]:
		var b := Button.new()
		b.text = "Accept" if answer else "Decline"
		b.focus_mode = Control.FOCUS_NONE
		b.pressed.connect(func():
			if _request_id >= 0:
				net.friend_answer(_request_id, answer)
			request_panel.visible = false)
		buttons.add_child(b)


func is_typing() -> bool:
	return name_box.has_focus()


func _add() -> void:
	var name := name_box.text.strip_edges()
	if name != "":
		net.friend_ask(name)
		name_box.text = ""
	name_box.release_focus()


func _process(_delta: float) -> void:
	if net == null:
		return
	var requests: Array = net.get_friend_requests()
	if requests.is_empty():
		request_panel.visible = false
		_request_id = -1
	elif int(requests[0][0]) != _request_id or not request_panel.visible:
		_request_id = int(requests[0][0])
		request_label.text = "%s wants to be friends" % requests[0][1]
		request_panel.visible = true
	if not visible:
		return
	var friends: Array = net.get_friends()
	if friends == _last:
		return
	_last = friends
	count_label.text = "%d / 35" % friends.size()
	for child in rows.get_children():
		child.queue_free()
	if friends.is_empty():
		var none := Label.new()
		none.text = "No friends yet"
		none.modulate = Color(1, 1, 1, 0.6)
		rows.add_child(none)
	for f in friends:
		var row := HBoxContainer.new()
		rows.add_child(row)
		var dot := Label.new()
		dot.text = "●"
		dot.modulate = Color(0.4, 1, 0.4) if f["online"] else Color(0.5, 0.5, 0.5)
		row.add_child(dot)
		var label := Label.new()
		label.size_flags_horizontal = Control.SIZE_EXPAND_FILL
		if f["online"]:
			label.text = "%s  Lv %d %s\n%s" % [f["name"], f["level"], CharacterWindow.JOBS.get(int(f["job"]), "Visitor"), f["zone"]]
		else:
			label.text = "%s  Lv %d\noffline" % [f["name"], f["level"]]
			label.modulate = Color(1, 1, 1, 0.6)
		label.add_theme_font_size_override("font_size", 13)
		row.add_child(label)
		var whisper := Button.new()
		whisper.text = "Whisper"
		whisper.focus_mode = Control.FOCUS_NONE
		whisper.disabled = not f["online"]
		var friend_name: String = f["name"]
		whisper.pressed.connect(func():
			chat_window.input.text = "@%s " % friend_name
			chat_window.focus_input()
			chat_window.input.caret_column = chat_window.input.text.length())
		row.add_child(whisper)
		var remove := Button.new()
		remove.text = "Remove"
		remove.focus_mode = Control.FOCUS_NONE
		remove.pressed.connect(func(): net.friend_remove(friend_name))
		row.add_child(remove)
