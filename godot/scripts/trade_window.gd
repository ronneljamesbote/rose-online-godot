## Trading with another player: right-click them and pick Trade. Both sides put up to ten
## bag items and some Zuly on the table (right-click a bag item while this is open, Shift
## for one of a stack; right-click it here to take it back), lock their offer, then press
## Trade. Changing an offer unlocks both sides. Also shows trade requests to accept.
extends PanelContainer

const InventoryWindow := preload("res://scripts/inventory_window.gd")
const MAX_ITEMS := 10
const TRADE_RANGE := 15.0  # metres; walking further away ends the trade

var net: RoseNet
var online: Node  # online.gd, for our character and the other player's
var inventory_window: Control
var request_panel: PanelContainer  # added to the HUD layer by online.gd
var request_label: Label
var title: Label
var my_slots: Array = []
var their_slots: Array = []
var my_money: SpinBox
var their_money: Label
var my_state: Label
var their_state: Label
var lock_button: Button
var trade_button: Button
var trade := {}
var _request_id := -1
var _offer: Array = []  # [page, index, quantity] per item we put up
var _setting_money := false


func _ready() -> void:
	var margin := MarginContainer.new()
	for side in ["left", "right", "top", "bottom"]:
		margin.add_theme_constant_override("margin_" + side, 10)
	add_child(margin)
	var box := VBoxContainer.new()
	box.add_theme_constant_override("separation", 6)
	margin.add_child(box)
	title = Label.new()
	title.theme_type_variation = "HeaderLabel"
	box.add_child(title)

	var columns := HBoxContainer.new()
	columns.add_theme_constant_override("separation", 16)
	box.add_child(columns)
	var mine := _column(columns, "You", "trade", my_slots)
	my_money = SpinBox.new()
	my_money.max_value = 1e12
	my_money.step = 1
	my_money.suffix = "Zuly"
	my_money.value_changed.connect(func(_v):
		if not _setting_money:
			_send())
	mine.add_child(my_money)
	my_state = Label.new()
	mine.add_child(my_state)
	var theirs := _column(columns, "", "trade_theirs", their_slots)
	their_money = Label.new()
	theirs.add_child(their_money)
	their_state = Label.new()
	theirs.add_child(their_state)

	var buttons := HBoxContainer.new()
	buttons.alignment = BoxContainer.ALIGNMENT_CENTER
	buttons.add_theme_constant_override("separation", 8)
	box.add_child(buttons)
	lock_button = Button.new()
	lock_button.focus_mode = Control.FOCUS_NONE
	lock_button.pressed.connect(func(): net.trade_lock(not trade.get("mine", {}).get("locked", false)))
	buttons.add_child(lock_button)
	trade_button = Button.new()
	trade_button.text = "Trade"
	trade_button.focus_mode = Control.FOCUS_NONE
	trade_button.pressed.connect(func(): net.trade_accept())
	buttons.add_child(trade_button)
	var cancel := Button.new()
	cancel.text = "Cancel"
	cancel.focus_mode = Control.FOCUS_NONE
	cancel.pressed.connect(func(): net.trade_cancel())
	buttons.add_child(cancel)

	request_panel = PanelContainer.new()
	request_panel.visible = false
	var rm := MarginContainer.new()
	for side in ["left", "right", "top", "bottom"]:
		rm.add_theme_constant_override("margin_" + side, 10)
	request_panel.add_child(rm)
	var rb := VBoxContainer.new()
	rm.add_child(rb)
	request_label = Label.new()
	rb.add_child(request_label)
	var answers := HBoxContainer.new()
	answers.alignment = BoxContainer.ALIGNMENT_CENTER
	rb.add_child(answers)
	for answer in [["Trade", true], ["No thanks", false]]:
		var b := Button.new()
		b.text = answer[0]
		b.focus_mode = Control.FOCUS_NONE
		b.pressed.connect(func(): answer_request(answer[1]))
		answers.add_child(b)


func _column(parent: Control, caption: String, kind: String, slots: Array) -> VBoxContainer:
	var column := VBoxContainer.new()
	column.add_theme_constant_override("separation", 4)
	parent.add_child(column)
	var label := Label.new()
	label.text = caption
	column.add_child(label)
	var grid := GridContainer.new()
	grid.columns = 5
	column.add_child(grid)
	for i in MAX_ITEMS:
		var slot = InventoryWindow.Slot.new(self, kind, i)
		grid.add_child(slot)
		slots.append(slot)
	return column


## The window's close button: calls the trade off.
func cancel_trade() -> void:
	net.trade_cancel()


func answer_request(accept: bool) -> void:
	if _request_id >= 0:
		net.trade_answer(_request_id, accept)
	_request_id = -1
	request_panel.visible = false


## Put a bag item on the table (from the inventory window).
func add(page: int, index: int, quantity: int) -> void:
	for o in _offer:
		if o[0] == page and o[1] == index:
			o[2] = quantity
			_send()
			return
	if _offer.size() >= MAX_ITEMS:
		return
	_offer.append([page, index, quantity])
	_send()


func _send() -> void:
	net.trade_offer(_offer, int(my_money.value))


## Slot callbacks (see inventory_window.gd's Slot): right-click takes our item back.
func activate(slot) -> void:
	if slot.kind == "trade" and slot.index < _offer.size():
		_offer.remove_at(slot.index)
		_send()


func select(_slot) -> void:
	pass


func dropped(from, _to) -> void:
	if from.kind == "page":
		add(inventory_window.page, from.index, from.item.get("quantity", 1))


func _process(_delta: float) -> void:
	if net == null:
		return
	var requests: Array = net.get_trade_requests()
	if requests.is_empty():
		_request_id = -1
		request_panel.visible = false
	elif _request_id != requests[0][0]:
		_request_id = requests[0][0]
		request_label.text = "%s wants to trade" % requests[0][1]
		request_panel.visible = true

	var t: Dictionary = net.get_trade()
	if t.is_empty():
		if visible:
			visible = false
			_offer = []
		trade = {}
		return
	if not visible:
		visible = true
		_offer = []
		_setting_money = true
		my_money.value = 0
		_setting_money = false
		if inventory_window:
			inventory_window.visible = true
	# Walking away ends the trade.
	var other = online.entities.get(int(t["with_entity"])) if online else null
	if online and online.me and other and online.me.position.distance_to(other.position) > TRADE_RANGE:
		net.trade_cancel()
	# Show the Zuly the server holds unless we are typing (a refused amount snaps back).
	if int(my_money.value) != t["mine"]["money"] and not my_money.get_line_edit().has_focus():
		_setting_money = true
		my_money.value = t["mine"]["money"]
		_setting_money = false
	if t == trade:
		return
	trade = t
	title.text = "Trade with %s" % t["with"]
	var mine: Dictionary = t["mine"]
	var theirs: Dictionary = t["theirs"]
	# What the server holds is our offer.
	_offer = mine["items"].map(func(item): return [item["page"], item["index"], item.get("quantity", 1)])
	for i in MAX_ITEMS:
		my_slots[i].show_item(mine["items"][i] if i < mine["items"].size() else null)
		their_slots[i].show_item(theirs["items"][i] if i < theirs["items"].size() else null)
	my_money.editable = not mine["locked"]
	their_slots[0].get_parent().get_parent().get_child(0).text = t["with"]
	their_money.text = "%d Zuly" % theirs["money"]
	my_state.text = _state(mine)
	their_state.text = _state(theirs)
	lock_button.text = "Unlock" if mine["locked"] else "Lock"
	trade_button.disabled = not (mine["locked"] and theirs["locked"]) or mine["accepted"]


func _state(side: Dictionary) -> String:
	if side["accepted"]:
		return "Ready to trade"
	return "Locked" if side["locked"] else "Choosing"
