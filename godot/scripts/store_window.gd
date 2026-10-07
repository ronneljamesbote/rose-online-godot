## NPC store: the store's tabs of items with their prices. Right-click an item to buy one,
## Shift+right-click to buy ten. While it is open, right-clicking an item in the inventory
## (or dragging it here) sells it.
extends PanelContainer

const InventoryWindow := preload("res://scripts/inventory_window.gd")
const COLUMNS := 8
const SLOTS := 48
const STORE_RANGE := 15.0  # metres; walking further away closes the store

var net: RoseNet
var online: Node  # online.gd, for our character
var inventory_window: Control
var npc_id := -1  # the store NPC's entity id
var npc: Node3D
var store := {}
var tab := 0
var title: Label
var tabs: TabBar
var slots: Array = []
var hint: Label


func _ready() -> void:
	var margin := MarginContainer.new()
	for side in ["left", "right", "top", "bottom"]:
		margin.add_theme_constant_override("margin_" + side, 10)
	add_child(margin)
	var box := VBoxContainer.new()
	box.add_theme_constant_override("separation", 6)
	margin.add_child(box)

	var top := HBoxContainer.new()
	box.add_child(top)
	title = Label.new()
	title.add_theme_font_size_override("font_size", 18)
	title.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	top.add_child(title)
	var close := Button.new()
	close.text = "Close"
	close.focus_mode = Control.FOCUS_NONE
	close.pressed.connect(close_store)
	top.add_child(close)

	tabs = TabBar.new()
	tabs.focus_mode = Control.FOCUS_NONE
	tabs.clip_tabs = false
	tabs.tab_changed.connect(func(t): tab = t; _show_tab())
	box.add_child(tabs)
	var grid := GridContainer.new()
	grid.columns = COLUMNS
	box.add_child(grid)
	for i in SLOTS:
		var slot = InventoryWindow.Slot.new(self, "store", i)
		grid.add_child(slot)
		slots.append(slot)

	hint = Label.new()
	hint.text = "Right-click buys 1, Shift+right-click 10.\nRight-click a bag item to sell 1, Shift for the stack."
	hint.add_theme_font_size_override("font_size", 12)
	hint.modulate = Color(1, 1, 1, 0.6)
	box.add_child(hint)


## Opens the store of this NPC entity. False when it has none.
func open_store(entity_id: int, npc_node: Node3D) -> bool:
	store = net.get_store(entity_id)
	if store.is_empty() or store["tabs"].is_empty():
		return false
	npc_id = entity_id
	npc = npc_node
	title.text = store["name"]
	tabs.clear_tabs()
	for t in store["tabs"]:
		tabs.add_tab(t["name"])
	tab = 0
	tabs.current_tab = 0
	_show_tab()
	visible = true
	if inventory_window:
		inventory_window.visible = true
		inventory_window.refresh_prices()
		# Just left of the inventory window.
		offset_right = inventory_window.offset_right - inventory_window.get_combined_minimum_size().x - 12
	return true


func close_store() -> void:
	visible = false
	npc_id = -1
	npc = null
	if inventory_window:
		inventory_window.refresh_prices()


func _show_tab() -> void:
	var items: Array = store["tabs"][tab]["items"] if tab < store["tabs"].size() else []
	for i in slots.size():
		var slot = slots[i]
		if i < items.size():
			var entry: Dictionary = items[i]
			var item: Dictionary = entry["item"].duplicate()
			item["tooltip"] = "%d Zuly\n%s" % [entry["price"], item.get("tooltip", "")]
			slot.show_item(item)
			slot.set_meta("entry", entry)
		else:
			slot.show_item(null)
			slot.remove_meta("entry")


## Slot callbacks (see inventory_window.gd's Slot).
func activate(slot) -> void:
	if not slot.has_meta("entry"):
		return
	var entry: Dictionary = slot.get_meta("entry")
	var quantity := 10 if Input.is_key_pressed(KEY_SHIFT) and entry["stackable"] else 1
	buy(entry["index"], quantity)


func select(_slot) -> void:
	pass


func dropped(from, _to) -> void:
	# A bag item dragged onto the store: sell the whole stack.
	sell(inventory_window.page, from.index, from.item.get("quantity", 1))


func buy(index: int, quantity: int) -> void:
	var store_tab: int = store["tabs"][tab]["tab"]
	net.store_transaction(npc_id, [[store_tab, index, quantity]], [])


func sell(page: int, index: int, quantity: int) -> void:
	net.store_transaction(npc_id, [], [[page, index, quantity]])


func _process(_delta: float) -> void:
	if not visible:
		return
	var me: Node3D = online.me if online else null
	if npc == null or not is_instance_valid(npc) or me == null or me.position.distance_to(npc.position) > STORE_RANGE:
		close_store()


func _unhandled_key_input(event: InputEvent) -> void:
	if visible and event is InputEventKey and event.pressed and event.keycode == KEY_ESCAPE:
		close_store()
