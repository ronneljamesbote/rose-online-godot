## Inventory window (I): equipped items and ammo, the four inventory pages, and money.
## Right-click (or double-click) an item to equip or use it, or an equipped item to take it
## off. Drag items between slots of a page; select one and press Drop to drop it. While a
## store is open, right-click sells instead (Shift sells the whole stack). In repair mode (an
## NPC's repair menu, or right-clicking a hammer) right-click repairs the item instead.
extends PanelContainer

const SLOT_SIZE := 44
const PAGES := ["Equip", "Use", "Etc", "Ride"]
const EQUIPPED := ["Face", "Head", "Body", "Back", "Hands", "Feet", "Weapon", "Off-hand", "Necklace", "Ring", "Earring"]
const AMMO := ["Arrows", "Bullets", "Shells"]
const VEHICLE := ["Frame", "Engine", "Wheels", "Arms"]


## One item slot: icon, quantity, tooltip; reports clicks and drags back to the window.
class Slot:
	extends Panel
	var window: Control
	var kind := ""  # "page", "equipped", "ammo", "vehicle", "store", "bank", "work", "trade", "trade_theirs", "skill" or "hotbar"
	var index := 0
	var item = null
	var icon: TextureRect
	var count: Label
	var key_label: Label  # the hotbar key (1-8), when the slot has one
	var shade: Control  # the cooldown sweep and seconds over a skill's icon
	var cooldown_end_ms := 0.0  # server clock (RoseNet.server_time_ms)
	var cooldown_total := 0.0  # seconds
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
		shade = Control.new()
		shade.set_anchors_and_offsets_preset(Control.PRESET_FULL_RECT)
		shade.mouse_filter = Control.MOUSE_FILTER_IGNORE
		shade.draw.connect(_draw_cooldown)
		add_child(shade)
		count = Label.new()
		count.set_anchors_and_offsets_preset(Control.PRESET_BOTTOM_RIGHT)
		count.grow_horizontal = Control.GROW_DIRECTION_BEGIN
		count.grow_vertical = Control.GROW_DIRECTION_BEGIN
		count.offset_right = -3
		count.theme_type_variation = "SlotCount"
		count.mouse_filter = Control.MOUSE_FILTER_IGNORE
		add_child(count)
		if caption != "":
			# Faint on an empty slot; on a filled one it sits on a small chip over the icon.
			key_label = Label.new()
			key_label.text = caption
			key_label.theme_type_variation = "SlotKey"
			key_label.position = Vector2(2, 1)
			key_label.mouse_filter = Control.MOUSE_FILTER_IGNORE
			add_child(key_label)

	func show_item(new_item) -> void:
		item = new_item
		if item == null:
			icon.texture = null
			count.text = ""
			tooltip_text = ""
		else:
			icon.texture = item.get("icon")
			# Broken gear (life 0) is tinted red until repaired.
			icon.modulate = Color(1.0, 0.35, 0.35) if item.has("life") and int(item["life"]) == 0 else Color.WHITE
			var quantity: int = item.get("quantity", 1)
			count.text = str(quantity) if quantity > 1 else ""
			tooltip_text = "%s\n%s" % [item.get("name", "?"), item.get("tooltip", "")]
		cooldown_end_ms = float(item.get("cooldown_end_ms", 0.0)) if item != null else 0.0
		cooldown_total = float(item.get("cooldown_total", 0.0)) if item != null else 0.0
		set_process(cooldown_end_ms > 0.0)
		shade.queue_redraw()
		if key_label != null:
			key_label.theme_type_variation = "SlotKey" if item == null else "SlotKeyFilled"
			key_label.reset_size()
		queue_redraw()

	## Seconds left on the skill's cooldown, 0 when it is ready.
	func cooldown_left() -> float:
		var net = window.get("net")
		if cooldown_end_ms <= 0.0 or net == null:
			return 0.0
		return maxf(0.0, (cooldown_end_ms - net.server_time_ms()) / 1000.0)

	func _process(_delta: float) -> void:
		shade.queue_redraw()
		if cooldown_left() <= 0.0:
			set_process(false)

	## A dark sweep over the part of the cooldown still to run (clockwise from the top,
	## shrinking as it runs) and the seconds left in the middle.
	func _draw_cooldown() -> void:
		var left := cooldown_left()
		if left <= 0.0:
			return
		var rect := Rect2(Vector2(2, 2), size - Vector2(4, 4))
		var part := clampf(left / cooldown_total, 0.0, 1.0) if cooldown_total > 0.0 else 1.0
		var centre := rect.get_center()
		var reach := rect.size.length()
		var points := PackedVector2Array([centre])
		var start := -PI / 2 + TAU * (1.0 - part)
		var steps := maxi(2, int(ceil(part * 48)))
		for i in steps + 1:
			var a := start + TAU * part * i / steps
			points.append(centre + Vector2(cos(a), sin(a)) * reach)
		var box := PackedVector2Array([rect.position, Vector2(rect.end.x, rect.position.y), rect.end, Vector2(rect.position.x, rect.end.y)])
		var colour: Color = UI.color("slot.cooldown", Color(0, 0, 0, 0.6))
		for piece in Geometry2D.intersect_polygons(points, box):
			shade.draw_colored_polygon(piece, colour)
		if cooldown_total < 1.0:
			return  # the global cooldown: too short for a number
		var text := str(ceili(left)) if left >= 1.0 else "%.1f" % left
		var font := get_theme_font("font", "SlotCount")
		var font_size := get_theme_font_size("font_size", "SlotCount") + 2
		var width := font.get_string_size(text, HORIZONTAL_ALIGNMENT_LEFT, -1, font_size).x
		var at := Vector2(centre.x - width / 2, centre.y + font.get_ascent(font_size) / 2 - 1)
		shade.draw_string_outline(font, at, text, HORIZONTAL_ALIGNMENT_LEFT, -1, font_size, get_theme_constant("outline_size", "SlotCount"), get_theme_color("font_outline_color", "SlotCount"))
		shade.draw_string(font, at, text, HORIZONTAL_ALIGNMENT_LEFT, -1, font_size, get_theme_color("font_color", "SlotCount"))

	func _draw() -> void:
		if selected:
			draw_rect(Rect2(Vector2.ZERO, size), UI.color("colors.accent"), false, 2.0)

	func _gui_input(event: InputEvent) -> void:
		if event is InputEventMouseButton and event.pressed:
			if event.button_index == MOUSE_BUTTON_RIGHT or (event.button_index == MOUSE_BUTTON_LEFT and event.double_click):
				window.activate(self)
			elif event.button_index == MOUSE_BUTTON_LEFT:
				window.select(self)
			accept_event()

	func _get_drag_data(_at: Vector2) -> Variant:
		if item == null or not (kind == "page" or kind == "skill" or kind == "hotbar" or kind == "bank"):
			return null
		var preview := TextureRect.new()
		preview.texture = icon.texture
		preview.custom_minimum_size = Vector2(SLOT_SIZE, SLOT_SIZE)
		preview.expand_mode = TextureRect.EXPAND_IGNORE_SIZE
		set_drag_preview(preview)
		return self

	func _can_drop_data(_at: Vector2, data: Variant) -> bool:
		if not (data is Slot):
			return false
		if kind == "hotbar":
			return data.kind == "page" or data.kind == "skill" or data.kind == "hotbar"
		if kind == "bank":
			return data.kind == "page" or data.kind == "bank"
		if kind == "work" or kind == "trade":
			return data.kind == "page"
		if kind == "trade_theirs":
			return false
		if data.kind == "bank":
			return kind == "page"
		return data.kind == "page" and (kind == "page" or kind == "equipped" or kind == "ammo" or kind == "vehicle" or kind == "store")

	func _drop_data(_at: Vector2, data: Variant) -> void:
		window.dropped(data, self)


var net: RoseNet
var online: Node  # online.gd, for where we stand
var store_window: Control  # store_window.gd; while it is open, right-click sells
var bank_window: Control  # bank_window.gd; while it is open, right-click deposits
var work_window: Control  # item_work_window.gd; while it is open, right-click picks the item
var trade_window: Control  # trade_window.gd; while it is open, right-click offers the item
var money_label: Label
var tabs: TabBar
var page := 0
var page_slots: Array = []
var equipped_slots: Array = []
var ammo_slots: Array = []
var vehicle_slots: Array = []
var fuel_label: Label
var drive_button: Button
var selected: Slot
var drop_button: Button
var _last_inventory := {}
var repair_hint: Label
var repair_npc := -1  # NPC entity repairing for Zuly, -1 when none
var repair_npc_node: Node3D
var repair_tool := []  # [page, index] of the hammer in use, empty when none
const REPAIR_RANGE := 15.0  # metres, as the server's NPC range


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
	title.theme_type_variation = "HeaderLabel"
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
	# The cart or castle gear: its four parts, fuel (the engine's life) and getting on.
	var ride := HBoxContainer.new()
	box.add_child(ride)
	for i in VEHICLE.size():
		var slot := Slot.new(self, "vehicle", i, VEHICLE[i])
		ride.add_child(slot)
		vehicle_slots.append(slot)
	fuel_label = Label.new()
	fuel_label.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	fuel_label.horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER
	ride.add_child(fuel_label)
	drive_button = Button.new()
	drive_button.text = "Drive"
	drive_button.focus_mode = Control.FOCUS_NONE
	drive_button.pressed.connect(func(): net.drive_toggle())
	ride.add_child(drive_button)
	box.add_child(HSeparator.new())

	repair_hint = Label.new()
	repair_hint.theme_type_variation = "AccentLabel"
	repair_hint.autowrap_mode = TextServer.AUTOWRAP_WORD
	repair_hint.visible = false
	box.add_child(repair_hint)

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


var _next_drive_check_ms := 0


func _process(_delta: float) -> void:
	if repair_npc >= 0 and (not visible or not is_instance_valid(repair_npc_node) or _far_from_repair_npc()):
		end_repair()
	if not visible and not repair_tool.is_empty():
		end_repair()
	if visible and net != null:
		_refresh(false)
		if Time.get_ticks_msec() >= _next_drive_check_ms:
			_next_drive_check_ms = Time.get_ticks_msec() + 300
			drive_button.text = "Get off" if net.get_character().get("driving", false) else "Drive"


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
	var parts: Array = inventory.get("vehicle", [null, null, null, null])
	for i in vehicle_slots.size():
		vehicle_slots[i].show_item(parts[i])
	fuel_label.text = "Fuel %d%%" % (int(parts[1].get("life", 0)) / 10) if parts[1] != null else ""
	drive_button.disabled = parts[0] == null
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


## An NPC's repair menu: right-click items to repair them for Zuly until the window closes.
func start_npc_repair(npc_id: int, npc_node: Node3D) -> void:
	repair_tool = []
	repair_npc = npc_id
	repair_npc_node = npc_node
	visible = true
	var who: String = npc_node.label.text if npc_node.get("label") else "NPC"
	_show_repair_hint("Repair with %s: right-click a worn item (Esc to stop)" % who)


## A hammer was used: the next right-clicked item gets repaired with it.
func start_tool_repair(tool_page: int, tool_index: int, tool_name: String) -> void:
	repair_npc = -1
	repair_tool = [tool_page, tool_index]
	_show_repair_hint("%s: right-click the item to repair (Esc to cancel)" % tool_name)


func end_repair() -> void:
	repair_npc = -1
	repair_npc_node = null
	repair_tool = []
	if repair_hint:
		repair_hint.visible = false


func _show_repair_hint(text: String) -> void:
	repair_hint.text = text
	repair_hint.visible = true
	reset_size()


func _far_from_repair_npc() -> bool:
	var me = online.get("me") if online else null
	if me == null or not is_instance_valid(me):
		return false
	var d := Vector2(repair_npc_node.position.x - me.position.x, repair_npc_node.position.z - me.position.z).length()
	return d > REPAIR_RANGE


## Repair the item in this slot with the NPC or hammer in use.
func _repair(slot: Slot) -> bool:
	var kind := -1
	var at := slot.index
	var index := 0
	match slot.kind:
		"equipped":
			kind = 0
		"vehicle":
			kind = 1
		"page":
			kind = 2
			at = page
			index = slot.index
	if kind < 0:
		return false
	if repair_npc >= 0:
		net.repair_at_npc(repair_npc, kind, at, index)
	else:
		net.repair_with_item(repair_tool[0], repair_tool[1], kind, at, index)
		end_repair()
	return true


## Right-click: equip or use an inventory item, take off an equipped one.
func activate(slot: Slot) -> void:
	if slot.item == null:
		return
	if slot.kind == "page" and page == 1 and slot.item.get("class", "") == "Repair Tool":
		start_tool_repair(page, slot.index, slot.item.get("name", "Hammer"))
		return
	if (repair_npc >= 0 or not repair_tool.is_empty()) and _repair(slot):
		return
	match slot.kind:
		"equipped":
			net.unequip_item(slot.index)
		"ammo":
			net.unequip_ammo(slot.index)
		"vehicle":
			net.unequip_vehicle_part(slot.index)
		"page":
			if store_window != null and store_window.visible:
				var quantity: int = slot.item.get("quantity", 1) if Input.is_key_pressed(KEY_SHIFT) else 1
				store_window.sell(page, slot.index, quantity)
			elif bank_window != null and bank_window.visible:
				var quantity: int = 1 if Input.is_key_pressed(KEY_SHIFT) else slot.item.get("quantity", 1)
				bank_window.deposit(page, slot.index, quantity)
			elif trade_window != null and trade_window.visible:
				var quantity: int = 1 if Input.is_key_pressed(KEY_SHIFT) else slot.item.get("quantity", 1)
				trade_window.add(page, slot.index, quantity)
			elif work_window != null and work_window.visible:
				work_window.set_target(page, slot.index)
			elif slot.item.get("type", "") == "Gem" and slot.item.get("class", "") == "Jewel":
				net.insert_gem(page, slot.index)
			elif page == 1:
				net.use_item(page, slot.index)
			else:
				net.equip_item(page, slot.index)


func dropped(from: Slot, to: Slot) -> void:
	if from.kind == "bank":
		bank_window.withdraw(from.index, from.item.get("quantity", 1))
	elif to.kind == "page":
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
	if event is InputEventKey and event.pressed and event.keycode == KEY_ESCAPE and (repair_npc >= 0 or not repair_tool.is_empty()):
		end_repair()
		get_viewport().set_input_as_handled()
		return
	if visible and event is InputEventKey and event.pressed and event.keycode == KEY_DELETE and selected:
		_drop_selected()
