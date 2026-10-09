## Personal shops (the Vending action), as iROSE's private store.
## Setting up: pick bag items, how many of each and a price for one, give the shop a title
## and open it; the character sits down and others see the title over it.
## A shop can also buy: search for an item by name, add it to the buy list with how many
## and the price for one, and others sell theirs to you (you pay when they do).
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
var _want_rows := []  # [row, item dict, quantity, price]
var want_box: VBoxContainer  # setup: the search and the buy list
var search_box: LineEdit
var search_results: VBoxContainer
var wants_list: VBoxContainer
var _last := {}
var list_scroll: ScrollContainer


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
	title_label.theme_type_variation = "HeaderLabel"
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

	list_scroll = ScrollContainer.new()
	list_scroll.custom_minimum_size = Vector2(450, 300)
	list_scroll.horizontal_scroll_mode = ScrollContainer.SCROLL_MODE_DISABLED
	box.add_child(list_scroll)
	rows = VBoxContainer.new()
	rows.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	list_scroll.add_child(rows)

	# Setup only: what the shop wants to buy.
	want_box = VBoxContainer.new()
	box.add_child(want_box)
	var want_title := Label.new()
	want_title.text = "Buy list (search for an item, then Add)"
	want_box.add_child(want_title)
	search_box = LineEdit.new()
	search_box.placeholder_text = "Item name"
	search_box.text_changed.connect(_search)
	want_box.add_child(search_box)
	search_results = VBoxContainer.new()
	want_box.add_child(search_results)
	var wants_scroll := ScrollContainer.new()
	wants_scroll.custom_minimum_size = Vector2(450, 110)
	wants_scroll.horizontal_scroll_mode = ScrollContainer.SCROLL_MODE_DISABLED
	want_box.add_child(wants_scroll)
	wants_list = VBoxContainer.new()
	wants_list.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	wants_scroll.add_child(wants_list)

	action_button = Button.new()
	action_button.focus_mode = Control.FOCUS_NONE
	action_button.pressed.connect(_action)
	box.add_child(action_button)
	hint = Label.new()
	hint.add_theme_font_size_override("font_size", 12)
	hint.modulate = Color(1, 1, 1, 0.6)
	box.add_child(hint)


func is_typing() -> bool:
	return title_box.has_focus() or search_box.has_focus()


## The Vending action: set up our shop, or show it when it is already open.
func open_setup() -> void:
	if not net.get_personal_store(online.my_id).is_empty():
		_show_mine()
		return
	mode = "setup"
	store_entity = -1
	# Both lists have to fit on a 720-pixel screen.
	list_scroll.custom_minimum_size.y = 170
	title_label.text = "Open a shop"
	title_box.visible = true
	want_box.visible = true
	action_button.visible = true
	action_button.text = "Open shop"
	hint.text = "Tick what to sell, how many and the price for one. Up to %d items to sell and %d to buy." % [MAX_ITEMS, MAX_ITEMS]
	_clear()
	_setup_rows = []
	_want_rows = []
	for child in wants_list.get_children():
		child.queue_free()
	for child in search_results.get_children():
		child.queue_free()
	search_box.text = ""
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
	reset_size()


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
	list_scroll.custom_minimum_size.y = 300
	title_box.visible = false
	want_box.visible = false
	action_button.visible = false
	hint.text = "Pick how many and press Buy, or Sell what the shop is buying."
	_last = {}
	_refresh()
	visible = true
	reset_size()


func _show_mine() -> void:
	mode = "mine"
	store_entity = online.my_id
	list_scroll.custom_minimum_size.y = 300
	title_box.visible = false
	want_box.visible = false
	action_button.visible = true
	action_button.text = "Close shop"
	hint.text = "You can't move, fight or use skills while the shop is open."
	_last = {}
	_refresh()
	visible = true
	reset_size()


func _action() -> void:
	if mode == "setup":
		var listings := []
		for r in _setup_rows:
			if r[0].button_pressed:
				listings.append([r[3], r[4], int(r[1].value), int(r[2].value)])
		var wanted := []
		for w in _want_rows:
			wanted.append([w[1]["type"], int(w[1]["number"]), int(w[2].value), int(w[3].value)])
		if listings.is_empty() and wanted.is_empty():
			online._notice("Tick something to sell or add something to buy")
			return
		if listings.size() > MAX_ITEMS or wanted.size() > MAX_ITEMS:
			online._notice("A shop sells and buys at most %d items each" % MAX_ITEMS)
			return
		net.store_open(title_box.text, listings, wanted)
		search_box.release_focus()
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
	if store["items"].is_empty() and store.get("wants", []).is_empty():
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
	var wants: Array = store.get("wants", [])
	if not wants.is_empty():
		var header := Label.new()
		header.text = "Buying"
		header.theme_type_variation = "HeaderLabel"
		rows.add_child(header)
	for want in wants:
		var row := _item_row(want)
		var price := Label.new()
		price.text = "%s z" % _zuly(int(want["price"]))
		price.custom_minimum_size = Vector2(64, 0)
		price.horizontal_alignment = HORIZONTAL_ALIGNMENT_RIGHT
		row.add_child(price)
		if mode != "browse":
			continue
		var have: int = want.get("have", 0)
		if have <= 0:
			var none := Label.new()
			none.text = "you have none"
			none.modulate = Color(1, 1, 1, 0.5)
			row.add_child(none)
			continue
		var quantity := SpinBox.new()
		quantity.min_value = 1
		quantity.max_value = max(mini(have, int(want["quantity"])), 1)
		quantity.editable = quantity.max_value > 1
		quantity.custom_minimum_size = Vector2(70, 0)
		row.add_child(quantity)
		var sell := Button.new()
		sell.text = "Sell"
		sell.focus_mode = Control.FOCUS_NONE
		var want_id: int = want["id"]
		var page: int = want["page"]
		var index: int = want["index"]
		sell.pressed.connect(func(): net.store_sell(store_entity, want_id, page, index, int(quantity.value)))
		row.add_child(sell)


## Setup: items matching the search, each with an Add button.
func _search(text: String) -> void:
	for child in search_results.get_children():
		child.queue_free()
	for item in net.find_items(text).slice(0, 4):
		var row := HBoxContainer.new()
		search_results.add_child(row)
		var icon := TextureRect.new()
		icon.custom_minimum_size = Vector2(24, 24)
		icon.expand_mode = TextureRect.EXPAND_IGNORE_SIZE
		icon.stretch_mode = TextureRect.STRETCH_KEEP_ASPECT_CENTERED
		if item.has("icon"):
			icon.texture = item["icon"]
		row.add_child(icon)
		var label := Label.new()
		label.text = item.get("name", "?")
		label.tooltip_text = item.get("tooltip", "")
		label.mouse_filter = Control.MOUSE_FILTER_PASS
		label.size_flags_horizontal = Control.SIZE_EXPAND_FILL
		row.add_child(label)
		var add := Button.new()
		add.text = "Add"
		add.focus_mode = Control.FOCUS_NONE
		add.pressed.connect(func(): add_want(item))
		row.add_child(add)


## Setup: put an item on the buy list.
func add_want(item: Dictionary) -> void:
	if _want_rows.size() >= MAX_ITEMS:
		online._notice("A shop buys at most %d items" % MAX_ITEMS)
		return
	var row := HBoxContainer.new()
	wants_list.add_child(row)
	var label := Label.new()
	label.text = "Buy " + String(item.get("name", "?"))
	label.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	label.clip_text = true
	row.add_child(label)
	var quantity := SpinBox.new()
	quantity.min_value = 1
	var stackable: bool = String(item.get("type", "")) in ["Consumable", "Gem", "Material", "Quest"]
	quantity.max_value = 999 if stackable else 1
	quantity.editable = stackable
	quantity.custom_minimum_size = Vector2(70, 0)
	quantity.tooltip_text = "How many to buy"
	row.add_child(quantity)
	var price := SpinBox.new()
	price.min_value = 1
	price.max_value = 4294967295
	price.value = 100
	price.suffix = "z"
	price.custom_minimum_size = Vector2(110, 0)
	price.tooltip_text = "Price for one"
	row.add_child(price)
	var remove := Button.new()
	remove.text = "X"
	remove.focus_mode = Control.FOCUS_NONE
	var entry := [row, item, quantity, price]
	remove.pressed.connect(func():
		_want_rows.erase(entry)
		row.queue_free())
	row.add_child(remove)
	_want_rows.append(entry)


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
