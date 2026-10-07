## Party frame, bottom right: each member's name, level and HP, the leader marked with a
## star. The leader right-clicks a member to hand over the lead or remove them, and sets
## how experience and drops are shared. Also shows party invitations to accept or decline.
extends PanelContainer

var net: RoseNet
var box: VBoxContainer
var rows: VBoxContainer
var xp_rule: OptionButton
var item_rule: OptionButton
var menu: PopupMenu
var invite_panel: PanelContainer  # added to the HUD layer by online.gd
var invite_label: Label
var _invite_id := -1
var _menu_member := ""
var _last := {}


func _ready() -> void:
	custom_minimum_size = Vector2(230, 0)
	var margin := MarginContainer.new()
	for side in ["left", "right", "top", "bottom"]:
		margin.add_theme_constant_override("margin_" + side, 8)
	add_child(margin)
	box = VBoxContainer.new()
	box.add_theme_constant_override("separation", 4)
	margin.add_child(box)

	var top := HBoxContainer.new()
	box.add_child(top)
	var title := Label.new()
	title.text = "Party"
	title.add_theme_font_size_override("font_size", 16)
	title.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	top.add_child(title)
	var leave := Button.new()
	leave.text = "Leave"
	leave.focus_mode = Control.FOCUS_NONE
	leave.pressed.connect(func(): net.party_leave())
	top.add_child(leave)

	rows = VBoxContainer.new()
	rows.add_theme_constant_override("separation", 4)
	box.add_child(rows)

	xp_rule = _rule(["XP shared equally", "XP shared by level"])
	item_rule = _rule(["Drops: picker, Zuly split", "Drops: in turn"])

	menu = PopupMenu.new()
	menu.add_item("Make leader", 0)
	menu.add_item("Remove from party", 1)
	menu.id_pressed.connect(_on_menu)
	add_child(menu)

	invite_panel = PanelContainer.new()
	invite_panel.visible = false
	var im := MarginContainer.new()
	for side in ["left", "right", "top", "bottom"]:
		im.add_theme_constant_override("margin_" + side, 10)
	invite_panel.add_child(im)
	var ib := VBoxContainer.new()
	im.add_child(ib)
	invite_label = Label.new()
	ib.add_child(invite_label)
	var buttons := HBoxContainer.new()
	buttons.alignment = BoxContainer.ALIGNMENT_CENTER
	ib.add_child(buttons)
	for answer in [["Accept", true], ["Decline", false]]:
		var b := Button.new()
		b.text = answer[0]
		b.focus_mode = Control.FOCUS_NONE
		b.pressed.connect(func(): answer_invite(answer[1]))
		buttons.add_child(b)


func _rule(names: Array) -> OptionButton:
	var o := OptionButton.new()
	for n in names:
		o.add_item(n)
	o.focus_mode = Control.FOCUS_NONE
	o.item_selected.connect(func(_i): net.party_set_rules(xp_rule.selected, item_rule.selected))
	box.add_child(o)
	return o


func answer_invite(accept: bool) -> void:
	if _invite_id >= 0:
		net.party_answer(_invite_id, accept)
	_invite_id = -1
	invite_panel.visible = false


func _process(_delta: float) -> void:
	if net == null:
		return
	var invites: Array = net.get_party_invites()
	if invites.is_empty():
		_invite_id = -1
		invite_panel.visible = false
	elif _invite_id != invites[0][0]:
		_invite_id = invites[0][0]
		invite_label.text = "%s invites you to a party" % invites[0][1]
		invite_panel.visible = true

	var party: Dictionary = net.get_party()
	visible = not party.is_empty()
	if party == _last:
		return
	_last = party
	if party.is_empty():
		return
	for child in rows.get_children():
		child.queue_free()
	for m in party["members"]:
		rows.add_child(_member_row(m, party["leader"]))
	xp_rule.selected = party["xp_sharing"]
	item_rule.selected = party["item_sharing"]
	xp_rule.disabled = not party["leader"]
	item_rule.disabled = not party["leader"]


func _member_row(m: Dictionary, i_lead: bool) -> Control:
	var row := VBoxContainer.new()
	row.add_theme_constant_override("separation", 1)
	var label := Label.new()
	label.text = "%s%s  Lv %d%s" % ["★ " if m["leader"] else "", m["name"], m["level"], "" if m["online"] else "  (offline)"]
	label.add_theme_font_size_override("font_size", 13)
	if not m["online"]:
		label.modulate = Color(1, 1, 1, 0.5)
	row.add_child(label)
	var bar := ProgressBar.new()
	bar.custom_minimum_size = Vector2(0, 8)
	bar.show_percentage = false
	bar.max_value = m["max_hp"]
	bar.value = m["hp"] if m["online"] else 0
	var fill := StyleBoxFlat.new()
	fill.bg_color = Color(0.8, 0.2, 0.2)
	bar.add_theme_stylebox_override("fill", fill)
	row.add_child(bar)
	if i_lead and not m["me"]:
		row.mouse_filter = Control.MOUSE_FILTER_STOP
		row.gui_input.connect(func(event):
			if event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_RIGHT:
				_menu_member = m["identity"]
				menu.position = Vector2i(get_viewport().get_mouse_position())
				menu.popup())
	return row


func _on_menu(id: int) -> void:
	if _menu_member != "":
		net.party_member_action(_menu_member, "lead" if id == 0 else "kick")
