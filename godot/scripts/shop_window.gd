## Personal shops (the Vending action), as iROSE's private store.
## Setting up: pick bag items, how many of each and a price for one, give the shop a title
## and open it; the character sits down and others see the title over it.
## Browsing: left-click a player with a shop sign to see what they sell and buy.
extends PanelContainer

const MAX_ITEMS := 30
const BROWSE_RANGE := 18.0  # metres; walking further away closes the window

var net: RoseNet
var online: Node  # online.gd
var title_label: Label
var title_box: LineEdit
var rows: VBoxContainer
var action_button: Button
var hint: Label
var mode := ""  # "setup", "mine" or "browse"
var store_entity := -1
var _setup_rows := []  # [check, quantity, price, page, index]
var _last := {}


func _ready() -> void:
	custom_minimum_size = Vector2(470, 0)
	var margin := MarginContainer.new()
	for side in ["left", "right", "top", "bottom"]:
		margin.add_theme_constant_override("margin_" + side, 10)
	add_child(margin)
	var box := VBoxContainer.new()
	box.add_theme_constant_override("separation", 6)
	margin.add_child(box)

	var top := HBoxContainer.new()
	box.add_child(top)
	title_label = Label.new()
	title_label.add_theme_font_size_override("font_size", 18)
	title_label.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	title_label.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
	top.add_child(title_label)
	var close := Button.new()
	close.text = "Close"
	close.focus_mode = Control.FOCUS_NONE
	close.pressed.connect(func(): visible = false)
	top.add_child(close)

	title_box = LineEdit.new()
	title_box.placeholder_text = "Shop title"
	title_box.max_length = 50
	box.add_child(title_box)

	var scroll := ScrollContainer.new()
	scroll.custom_minimum_size = Vector2(450, 300)
	scroll.horizontal_scroll_mode = ScrollContainer.SCROLL_MODE_DISABLED
	box.add_child(scroll)
	rows = VBoxContainer.new()
	rows.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	scroll.add_child(rows)

	action_button = Button.new()
	action_button.focus_mode = Control.FOCUS_NONE
	action_button.pressed.connect(_action)
	box.add_child(action_button)
	hint = Label.new()
	hint.add_theme_font_size_override("font_size", 12)
	hint.modulate = Color(1, 1, 1, 0.6)
	box.add_child(hint)


func is_typing() -> bool:
	return title_box.has_focus()


## The Vending action: set up our shop, or show it when it is already open.
func open_setup() -> void:
	if not net.get_personal_store(online.my_id).is_empty():
		_show_mine()
		return
	mode = "setup"
	store_entity = -1
	title_label.text = "Open a shop"
	title_box.visible = true
	action_button.text = "Open shop"
	hint.text = "Tick what to sell, how many and the price for one. Up to %d items." % MAX_ITEMS
	_clear()
	_setup_rows = []
	var inventory: Dictionary = net.get_inventory()
	var pages: Array = inventory.get("pages", [])
	for page in pages.size():
		for index in pages[page].size():
			var item = pages[page][index]
			if item == null or not item.get("tradeable", true):
				continue
			var row := _item_row(item)
			var check := CheckBox.new()
			check.focus_mode = Control.FOCUS_NONE
			row.add_child(check)
			var quantity := SpinBox.new()
			quantity.min_value = 1
			quantity.max_value = max(int(item["quantity"]), 1)
			quantity.value = quantity.max_value
			quantity.editable = int(item["quantity"]) > 1
			quantity.custom_minimum_size = Vector2(70, 0)
			quantity.tooltip_text = "How many to sell"
			row.add_child(quantity)
			var price := SpinBox.new()
			price.min_value = 1
			price.max_value = 4294967295
			price.value = 100
			price.suffix = "z"
			price.custom_minimum_size = Vector2(110, 0)
			price.tooltip_text = "Price for one"
			row.add_child(price)
			_setup_rows.append([check, quantity, price, page, index])
	visible = true


## A shop someone else (or we) opened on this entity.
func browse(entity_id: int) -> void:
	var store: Dictionary = net.get_personal_store(entity_id)
	if store.is_empty():
		return
	if store["mine"]:
		_show_mine()
		return
	mode = "browse"
	store_entity = entity_id
	title_box.visible = false
	action_button.visible = false
	hint.text = "Pick how many and press Buy."
	_last = {}
	_refresh()
	visible = true


func _show_mine() -> void:
	mode = "mine"
	store_entity = online.my_id
	title_box.visible = false
	action_button.visible = true
	action_button.text = "Close shop"
	hint.text = "You can't move, fight or use skills while the shop is open."
	_last = {}
	_refresh()
	visible = true


func _action() -> void:
	if mode == "setup":
		var listings := []
		for r in _setup_rows:
			if r[0].button_pressed:
				listings.append([r[3], r[4], int(r[1].value), int(r[2].value)])
		if listings.is_empty():
			online._notice("Tick at least one item to sell")
			return
		if listings.size() > MAX_ITEMS:
			online._notice("A shop holds at most %d items" % MAX_ITEMS)
			return
		net.store_open(title_box.text, listings)
		title_box.release_focus()
		visible = false
	elif mode == "mine":
		net.store_close()
		visible = false


func _clear() -> void:
	for child in rows.get_children():
		child.queue_free()


func _item_row(item: Dictionary) -> HBoxContainer:
	var row := HBoxContainer.new()
	rows.add_child(row)
	var icon := TextureRect.new()
	icon.custom_minimum_size = Vector2(32, 32)
	icon.expand_mode = TextureRect.EXPAND_IGNORE_SIZE
	icon.stretch_mode = TextureRect.STRETCH_KEEP_ASPECT_CENTERED
	if item.has("icon"):
		icon.texture = item["icon"]
	icon.tooltip_text = item.get("tooltip", "")
	row.add_child(icon)
	var label := Label.new()
	label.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	label.clip_text = true
	label.text = item.get("name", "?")
	label.tooltip_text = item.get("tooltip", "")
	label.mouse_filter = Control.MOUSE_FILTER_PASS
	row.add_child(label)
	if int(item.get("quantity", 1)) > 1:
		var count := Label.new()
		count.text = "x%d" % item["quantity"]
		count.modulate = Color(1, 1, 1, 0.75)
		row.add_child(count)
	return row


func _refresh() -> void:
	var store: Dictionary = net.get_personal_store(store_entity)
	if store.is_empty():
		if mode == "browse":
			online._notice("That shop has closed")
		visible = false
		return
	if store == _last:
		return
	_last = store
	title_label.text = "%s\n%s" % [store["title"], store["owner"]]
	_clear()
	if store["items"].is_empty():
		var none := Label.new()
		none.text = "Sold out"
		rows.add_child(none)
	for item in store["items"]:
		var row := _item_row(item)
		var price := Label.new()
		price.text = "%s z" % _zuly(int(item["price"]))
		price.custom_minimum_size = Vector2(64, 0)
		price.horizontal_alignment = HORIZONTAL_ALIGNMENT_RIGHT
		row.add_child(price)
		if mode != "browse":
			continue
		var quantity := SpinBox.new()
		quantity.min_value = 1
		quantity.max_value = max(int(item["quantity"]), 1)
		quantity.editable = int(item["quantity"]) > 1
		quantity.custom_minimum_size = Vector2(70, 0)
		row.add_child(quantity)
		var buy := Button.new()
		buy.text = "Buy"
		buy.focus_mode = Control.FOCUS_NONE
		var id: int = item["id"]
		buy.pressed.connect(func(): net.store_buy(store_entity, id, int(quantity.value)))
		row.add_child(buy)


static func _zuly(amount: int) -> String:
	var s := str(amount)
	var out := ""
	while s.length() > 3:
		out = "," + s.substr(s.length() - 3) + out
		s = s.substr(0, s.length() - 3)
	return s + out


var _next_refresh_ms := 0


func _process(_delta: float) -> void:
	if not visible or mode == "setup" or net == null:
		return
	if Time.get_ticks_msec() < _next_refresh_ms:
		return
	_next_refresh_ms = Time.get_ticks_msec() + 300
	if mode == "browse" and online.me and online.entities.has(store_entity):
		if online.entities[store_entity].position.distance_to(online.me.position) > BROWSE_RANGE:
			visible = false
			return
	_refresh()
