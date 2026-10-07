## Inventory window (I): equipped items and ammo, the four inventory pages, and money.
## Right-click (or double-click) an item to equip or use it, or an equipped item to take it
## off. Drag items between slots of a page; select one and press Drop to drop it. While a
## store is open, right-click sells instead (Shift sells the whole stack).
extends PanelContainer

const SLOT_SIZE := 44
const PAGES := ["Equip", "Use", "Etc", "Ride"]
const EQUIPPED := ["Face", "Head", "Body", "Back", "Hands", "Feet", "Weapon", "Off-hand", "Necklace", "Ring", "Earring"]
const AMMO := ["Arrows", "Bullets", "Shells"]


## One item slot: icon, quantity, tooltip; reports clicks and drags back to the window.
class Slot:
	extends Panel
	var window: Control
	var kind := ""  # "page", "equipped" or "ammo"
	var index := 0
	var item = null
	var icon: TextureRect
	var count: Label
	var selected := false

	func _init(owner_window: Control, slot_kind: String, slot_index: int, caption := "") -> void:
		window = owner_window
		kind = slot_kind
		index = slot_index
		custom_minimum_size = Vector2(SLOT_SIZE, SLOT_SIZE)
		mouse_filter = Control.MOUSE_FILTER_STOP
		icon = TextureRect.new()
		icon.set_anchors_and_offsets_preset(Control.PRESET_FULL_RECT)
		icon.offset_left = 2
		icon.offset_top = 2
		icon.offset_right = -2
		icon.offset_bottom = -2
		icon.expand_mode = TextureRect.EXPAND_IGNORE_SIZE
		icon.stretch_mode = TextureRect.STRETCH_KEEP_ASPECT_CENTERED
		icon.mouse_filter = Control.MOUSE_FILTER_IGNORE
		add_child(icon)
		count = Label.new()
		count.set_anchors_and_offsets_preset(Control.PRESET_BOTTOM_RIGHT)
		count.grow_horizontal = Control.GROW_DIRECTION_BEGIN
		count.grow_vertical = Control.GROW_DIRECTION_BEGIN
		count.add_theme_font_size_override("font_size", 12)
		count.add_theme_color_override("font_shadow_color", Color.BLACK)
		count.add_theme_constant_override("shadow_offset_x", 1)
		count.add_theme_constant_override("shadow_offset_y", 1)
		count.mouse_filter = Control.MOUSE_FILTER_IGNORE
		add_child(count)
		if caption != "":
			var name_label := Label.new()
			name_label.text = caption
			name_label.add_theme_font_size_override("font_size", 9)
			name_label.modulate = Color(1, 1, 1, 0.45)
			name_label.position = Vector2(3, 1)
			name_label.mouse_filter = Control.MOUSE_FILTER_IGNORE
			add_child(name_label)

	func show_item(new_item) -> void:
		item = new_item
		if item == null:
			icon.texture = null
			count.text = ""
			tooltip_text = ""
		else:
			icon.texture = item.get("icon")
			var quantity: int = item.get("quantity", 1)
			count.text = str(quantity) if quantity > 1 else ""
			tooltip_text = "%s\n%s" % [item.get("name", "?"), item.get("tooltip", "")]
		queue_redraw()

	func _draw() -> void:
		if selected:
			draw_rect(Rect2(Vector2.ZERO, size), Color(1.0, 0.85, 0.3), false, 2.0)

	func _gui_input(event: InputEvent) -> void:
		if event is InputEventMouseButton and event.pressed:
			if event.button_index == MOUSE_BUTTON_RIGHT or (event.button_index == MOUSE_BUTTON_LEFT and event.double_click):
				window.activate(self)
			elif event.button_index == MOUSE_BUTTON_LEFT:
				window.select(self)
			accept_event()

	func _get_drag_data(_at: Vector2) -> Variant:
		if item == null or kind != "page":
			return null
		var preview := TextureRect.new()
		preview.texture = icon.texture
		preview.custom_minimum_size = Vector2(SLOT_SIZE, SLOT_SIZE)
		preview.expand_mode = TextureRect.EXPAND_IGNORE_SIZE
		set_drag_preview(preview)
		return self

	func _can_drop_data(_at: Vector2, data: Variant) -> bool:
		return data is Slot and data.kind == "page" and (kind == "page" or kind == "equipped" or kind == "ammo" or kind == "store")

	func _drop_data(_at: Vector2, data: Variant) -> void:
		window.dropped(data, self)


var net: RoseNet
var store_window: Control  # store_window.gd; while it is open, right-click sells
var money_label: Label
var tabs: TabBar
var page := 0
var page_slots: Array = []
var equipped_slots: Array = []
var ammo_slots: Array = []
var selected: Slot
var drop_button: Button
var _last_inventory := {}


func _ready() -> void:
	var margin := MarginContainer.new()
	for side in ["left", "right", "top", "bottom"]:
		margin.add_theme_constant_override("margin_" + side, 10)
	add_child(margin)
	var box := VBoxContainer.new()
	box.add_theme_constant_override("separation", 6)
	margin.add_child(box)

	var title := Label.new()
	title.text = "Inventory"
	title.add_theme_font_size_override("font_size", 18)
	box.add_child(title)

	var equipped_grid := GridContainer.new()
	equipped_grid.columns = 7
	box.add_child(equipped_grid)
	for i in EQUIPPED.size():
		var slot := Slot.new(self, "equipped", i, EQUIPPED[i])
		equipped_grid.add_child(slot)
		equipped_slots.append(slot)
	for i in AMMO.size():
		var slot := Slot.new(self, "ammo", i, AMMO[i])
		equipped_grid.add_child(slot)
		ammo_slots.append(slot)
	box.add_child(HSeparator.new())

	tabs = TabBar.new()
	for name in PAGES:
		tabs.add_tab(name)
	tabs.focus_mode = Control.FOCUS_NONE
	tabs.tab_changed.connect(func(t): page = t; _clear_selection(); _refresh(true))
	box.add_child(tabs)
	var grid := GridContainer.new()
	grid.columns = 6
	box.add_child(grid)
	for i in 30:
		var slot := Slot.new(self, "page", i)
		grid.add_child(slot)
		page_slots.append(slot)

	var bottom := HBoxContainer.new()
	box.add_child(bottom)
	money_label = Label.new()
	money_label.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	bottom.add_child(money_label)
	drop_button = Button.new()
	drop_button.text = "Drop"
	drop_button.focus_mode = Control.FOCUS_NONE
	drop_button.disabled = true
	drop_button.pressed.connect(_drop_selected)
	bottom.add_child(drop_button)


func _process(_delta: float) -> void:
	if visible and net != null:
		_refresh(false)


func _refresh(force: bool) -> void:
	var inventory: Dictionary = net.get_inventory()
	if inventory.is_empty() or (not force and inventory == _last_inventory):
		return
	_last_inventory = inventory
	money_label.text = "%d Zuly" % inventory["money"]
	var items: Array = inventory["pages"][page]
	var selling: bool = store_window != null and store_window.visible
	for i in page_slots.size():
		var item = items[i]
		if selling and item != null:
			var price: int = net.sell_price(page, i)
			if price >= 0:
				item = item.duplicate()
				item["tooltip"] = "Sells for %d Zuly each\n%s" % [price, item.get("tooltip", "")]
		page_slots[i].show_item(item)
	for i in equipped_slots.size():
		equipped_slots[i].show_item(inventory["equipped"][i])
	for i in ammo_slots.size():
		ammo_slots[i].show_item(inventory["ammo"][i])
	if selected and selected.item == null:
		_clear_selection()


## Show or hide sell prices in the tooltips (a store opened or closed).
func refresh_prices() -> void:
	if net != null:
		_refresh(true)


func select(slot: Slot) -> void:
	_clear_selection()
	if slot.item == null or slot.kind != "page":
		return
	selected = slot
	slot.selected = true
	slot.queue_redraw()
	drop_button.disabled = false


func _clear_selection() -> void:
	if selected:
		selected.selected = false
		selected.queue_redraw()
	selected = null
	if drop_button:
		drop_button.disabled = true


## Right-click: equip or use an inventory item, take off an equipped one.
func activate(slot: Slot) -> void:
	if slot.item == null:
		return
	match slot.kind:
		"equipped":
			net.unequip_item(slot.index)
		"ammo":
			net.unequip_ammo(slot.index)
		"page":
			if store_window != null and store_window.visible:
				var quantity: int = slot.item.get("quantity", 1) if Input.is_key_pressed(KEY_SHIFT) else 1
				store_window.sell(page, slot.index, quantity)
			elif page == 1:
				net.use_item(page, slot.index)
			else:
				net.equip_item(page, slot.index)


func dropped(from: Slot, to: Slot) -> void:
	if to.kind == "page":
		net.move_item(page, from.index, to.index)
	else:
		net.equip_item(page, from.index)
	_clear_selection()


func _drop_selected() -> void:
	if selected == null or selected.item == null:
		return
	net.drop_item(page, selected.index, selected.item.get("quantity", 1))
	_clear_selection()


func _unhandled_key_input(event: InputEvent) -> void:
	if visible and event is InputEventKey and event.pressed and event.keycode == KEY_DELETE and selected:
		_drop_selected()
