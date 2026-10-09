## Bank (storage): four pages of 30 slots, opened at an NPC whose dialog offers it. Right-click
## a bank item to take the stack back (Shift takes one); while it is open, right-clicking a
## bag item stores the stack (Shift stores one). Items can also be dragged between the two.
extends PanelContainer

const InventoryWindow := preload("res://scripts/inventory_window.gd")
const PAGES := ["1", "2", "3", "4"]
const PAGE_SIZE := 30
const BANK_RANGE := 15.0  # metres; walking further away closes the bank

var net: RoseNet
var online: Node  # online.gd, for our character
var inventory_window: Control
var npc_id := -1  # the NPC's entity id
var npc: Node3D
var page := 0
var tabs: TabBar
var slots: Array = []
var _last := []


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
	var title := Label.new()
	title.text = "Storage"
	title.theme_type_variation = "HeaderLabel"
	title.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	top.add_child(title)
	var close := Button.new()
	close.text = "Close"
	close.focus_mode = Control.FOCUS_NONE
	close.pressed.connect(close_bank)
	top.add_child(close)

	tabs = TabBar.new()
	for name in PAGES:
		tabs.add_tab(name)
	tabs.focus_mode = Control.FOCUS_NONE
	tabs.tab_changed.connect(func(t): page = t; _refresh(true))
	box.add_child(tabs)
	var grid := GridContainer.new()
	grid.columns = 6
	box.add_child(grid)
	for i in PAGE_SIZE:
		var slot = InventoryWindow.Slot.new(self, "bank", i)
		grid.add_child(slot)
		slots.append(slot)

	var hint := Label.new()
	hint.text = "Right-click takes the stack, Shift one.\nRight-click a bag item to store it."
	hint.add_theme_font_size_override("font_size", 12)
	hint.theme_type_variation = "MutedLabel"
	box.add_child(hint)


func open_bank(entity_id: int, npc_node: Node3D) -> void:
	npc_id = entity_id
	npc = npc_node
	visible = true
	_refresh(true)
	if inventory_window:
		inventory_window.visible = true
		# Just left of the inventory window.
		offset_right = inventory_window.offset_right - inventory_window.get_combined_minimum_size().x - 12


func close_bank() -> void:
	visible = false
	npc_id = -1
	npc = null


func _refresh(force: bool) -> void:
	var bank: Array = net.get_bank()
	if not force and bank == _last:
		return
	_last = bank
	for i in slots.size():
		var index: int = page * PAGE_SIZE + i
		slots[i].show_item(bank[index] if index < bank.size() else null)


func deposit(bag_page: int, index: int, quantity: int) -> void:
	net.bank_deposit(npc_id, bag_page, index, quantity)


func withdraw(slot_index: int, quantity: int) -> void:
	net.bank_withdraw(npc_id, page * PAGE_SIZE + slot_index, quantity)


## Slot callbacks (see inventory_window.gd's Slot).
func activate(slot) -> void:
	if slot.item == null:
		return
	withdraw(slot.index, 1 if Input.is_key_pressed(KEY_SHIFT) else slot.item.get("quantity", 1))


func select(_slot) -> void:
	pass


func dropped(from, to) -> void:
	if from.kind == "bank":
		net.bank_move(page * PAGE_SIZE + from.index, page * PAGE_SIZE + to.index)
	else:
		deposit(inventory_window.page, from.index, from.item.get("quantity", 1))


func _process(_delta: float) -> void:
	if not visible:
		return
	_refresh(false)
	var me: Node3D = online.me if online else null
	if npc == null or not is_instance_valid(npc) or me == null or me.position.distance_to(npc.position) > BANK_RANGE:
		close_bank()


func _unhandled_key_input(event: InputEvent) -> void:
	if visible and event is InputEventKey and event.pressed and event.keycode == KEY_ESCAPE:
		close_bank()
