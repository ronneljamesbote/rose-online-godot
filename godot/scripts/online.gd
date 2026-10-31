## Online play: connects to the SpacetimeDB server and shows every player, monster and
## town NPC in the zone, each moving along the server's motion paths, with hits and damage numbers.
extends Node3D

signal joined(me: Node3D)
signal zone_needed(zone_id: int)
signal warp_gate_near(zone_id: int)  # the zone behind a gate we stand near, 0 when none
## The server turned the connection down or closed it.
signal connection_failed(reason: String)

const NetEntity := preload("res://scripts/net_entity.gd")
const CharacterWindow := preload("res://scripts/character_window.gd")
const InventoryWindow := preload("res://scripts/inventory_window.gd")
const StoreWindow := preload("res://scripts/store_window.gd")
const BankWindow := preload("res://scripts/bank_window.gd")
const PartyWindow := preload("res://scripts/party_window.gd")
const CraftWindow := preload("res://scripts/craft_window.gd")
const ItemWorkWindow := preload("res://scripts/item_work_window.gd")
const TradeWindow := preload("res://scripts/trade_window.gd")
const SkillWindow := preload("res://scripts/skill_window.gd")
const ConversationWindow := preload("res://scripts/conversation_window.gd")
const QuestWindow := preload("res://scripts/quest_window.gd")
const ChatWindow := preload("res://scripts/chat_window.gd")
const CharacterCreate := preload("res://scripts/character_create.gd")
const Hotbar := preload("res://scripts/hotbar.gd")
const ReviveWindow := preload("res://scripts/revive_window.gd")
const FriendsWindow := preload("res://scripts/friends_window.gd")
const ShopWindow := preload("res://scripts/shop_window.gd")
const PlayerFrame := preload("res://scripts/ui/hud/player_frame.gd")
const TargetFrame := preload("res://scripts/ui/hud/target_frame.gd")
const PartyFrames := preload("res://scripts/ui/hud/party_frames.gd")
const EffectRows := preload("res://scripts/ui/hud/effect_rows.gd")
const Minimap := preload("res://scripts/ui/hud/minimap.gd")
const HudMenu := preload("res://scripts/ui/hud/menu_bar.gd")
const QuestTracker := preload("res://scripts/ui/hud/quest_tracker.gd")
const Notices := preload("res://scripts/ui/hud/notices.gd")
const CartGauge := preload("res://scripts/ui/hud/cart_gauge.gd")
const XpLine := preload("res://scripts/ui/hud/xp_line.gd")
const HoverTooltip := preload("res://scripts/ui/hud/hover_tooltip.gd")
const ZoneMap := preload("res://scripts/ui/zone_map.gd")
const OptionsWindow := preload("res://scripts/ui/options_window.gd")
const WorldOverlay := preload("res://scripts/ui/world_overlay.gd")
const TargetMarker := preload("res://scripts/target_marker.gd")
const PICK_RADIUS_PX := 60.0
const ITEM_PICK_RADIUS_PX := 30.0
const PICKUP_RANGE := 2.5  # metres; the server allows 4
const NOTICE_SECONDS := 8.0
const WARP_MARGIN := 3.0  # metres around a warp gate's model that count as walking into it
const TALK_RANGE := 3.0  # metres from an NPC to talk to it
const TALK_STOPPED_RANGE := 12.0  # or this close, when a wall stopped us on the way
const NPC_NAME_RANGE := 30.0
const MONSTER_NAME_RANGE := 15.0  # monster names show within this many metres, or when targeted
const CLICK_MARKER_EFFECT := "3DDATA/EFFECT/FLAT_POINT_01.EFT"  # FILE_EFFECT.STB row 296, the target effect TRose.exe loads

var zone: Node
var net: RoseNet
var entities := {}  # entity id -> NetEntity
var me: Node3D
var my_id := -1
var my_target := -1
var player_name := ""
var ranged := false  # equip the bow and arrows from the bag once signed in (--weapon=bow)
var _ranged_equipped := false
var _next_equip_try_ms := 0
var hud: Label  # connection and server messages
var ui_root: UiRoot
var world_overlay: Node  # name tags, item labels, bubbles, damage numbers
var _under := {}  # under_mouse() for _under_frame and _under_at
var _under_frame := -1
var _under_at := Vector2.ZERO
var player_frame: Control
var target_frame: Control
var party_frames: Control
var effects_rows: Control
var minimap: Control
var quest_tracker: Control
var menu_bar: Control
var cart_gauge: Control
var xp_line: Control
var map_window: Control
var options_window: Control
var _next_party_ms := 0
## --ui-sample: fills the buff rows and party frames with made-up entries, for screenshots.
var ui_sample := false
var ui_sample_screen := ""  # --ui-screen=death shows the death screen
var _next_quests_ms := 0
var character_window: PanelContainer
var inventory_window: PanelContainer
var store_window: PanelContainer
var bank_window: PanelContainer
var party_window: PanelContainer
var craft_window: PanelContainer
var work_window: PanelContainer
var trade_window: PanelContainer
var player_menu: PopupMenu
var ride_panel: PanelContainer
var ride_label: Label
var _ride_offer := -1
var _menu_player := -1
var skill_window: PanelContainer
var conversation_window: PanelContainer
var quest_window: PanelContainer
var hotbar: PanelContainer
var revive_window: PanelContainer
var friends_window: PanelContainer
var shop_window: PanelContainer
var _next_effects_ms := 0
var notice_box: Control
var chat_window: PanelContainer
var character_create: CanvasLayer  # while the account has no character
var create_preset := ""  # --create=NAME,female,FACE,HAIR fills the creation screen
var create_after := -1.0  # and --create-after=SECONDS presses Create
var _failed := false
var _layer: CanvasLayer
var ground := {}  # drop id -> Node3D
var _pickup := -1  # drop we are walking to
var _talk := -1  # NPC we are walking to
var _click_marker: Node3D  # the marker where we last clicked the ground
var target_marker: Decal  # the ring under the selected target
var _chasing := false  # heading for my_target (attacking it, casting at it, or selecting it)
var _steer_target = null  # Vector3: where the held left button last sent us, or null
var _warps: Array = []  # this zone's warp gates: id, position, size
var _warps_zone: Node = null
var _next_warp_ms := 0
var _warp_armed := false  # false until we stand outside every gate (we may arrive inside one)
var _near_gate_zone := 0  # the zone behind the warp gate we stand near (warp_gate_near)
var _level := 0
var _zone_requested := 0
var log_damage := false
var log_positions := false  # print every player's position once a second
var log_perf := false  # print frame rate, entities drawn and frame times every 2 seconds
var _next_perf_ms := 0
var _perf_entities_us := 0  # time spent in net.get_entities since the last perf line
var _perf_frames := 0
var _next_log_ms := 0
var _killed := {}  # entity ids whose last hit killed them
var _next_signs_ms := 0


## token is the account website's game token; without one the identity in token_path is used
## (servers that let anyone in).
func start(zone_node: Node, uri: String, token_path: String, name_text: String, use_bow: bool, token := "") -> bool:
	zone = zone_node
	player_name = name_text
	ranged = use_bow
	net = RoseNet.new()
	net.name = "Net"
	add_child(net)

	var layer := CanvasLayer.new()
	add_child(layer)
	_layer = layer
	# Name tags, item labels, bubbles and damage numbers sit under the interface.
	world_overlay = WorldOverlay.new()
	world_overlay.online = self
	add_child(world_overlay)
	target_marker = TargetMarker.new()
	add_child(target_marker)
	ui_root = UiRoot.new()
	ui_root.name = "UiRoot"
	layer.add_child(ui_root)
	# Connection and server messages, top centre.
	hud = Label.new()
	hud.theme_type_variation = "HeaderLabel"
	hud.add_theme_color_override("font_color", Color.WHITE)
	hud.add_theme_color_override("font_outline_color", Color(0, 0, 0, 0.85))
	hud.add_theme_constant_override("outline_size", 5)
	hud.horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER
	hud.set_anchors_and_offsets_preset(Control.PRESET_CENTER_TOP)
	hud.grow_horizontal = Control.GROW_DIRECTION_BOTH
	hud.offset_top = 96
	hud.mouse_filter = Control.MOUSE_FILTER_IGNORE
	ui_root.add_child(hud)

	# HUD widgets; each can be dragged by the grip that shows on hover.
	player_frame = PlayerFrame.new()
	_widget(player_frame, "player", Vector2(0, 0), Vector2(12, 30))
	party_frames = PartyFrames.new()
	_widget(party_frames, "party_frames", Vector2(0, 0), Vector2(12, 130))
	target_frame = TargetFrame.new()
	target_frame.visible = false
	_widget(target_frame, "target", Vector2(0.5, 0), Vector2(0, 12))
	effects_rows = EffectRows.new()
	_widget(effects_rows, "effects", Vector2(0, 0), Vector2(310, 32))
	minimap = Minimap.new()
	minimap.online = self
	_widget(minimap, "minimap", Vector2(1, 0), Vector2(-12, 12))
	quest_tracker = QuestTracker.new()
	quest_tracker.open_quests.connect(toggle_quest_window)
	_widget(quest_tracker, "quest_tracker", Vector2(1, 0), Vector2(-12, 236))
	menu_bar = HudMenu.new()
	menu_bar.open.connect(_on_menu)
	_widget(menu_bar, "menu", Vector2(1, 1), Vector2(-12, -12))
	# What the mouse is over in the world, bottom right above the menu bar.
	var hover_tip := HoverTooltip.new()
	hover_tip.online = self
	ui_root.add_child(hover_tip)
	UI.cursor.online = self
	cart_gauge = CartGauge.new()
	cart_gauge.visible = false
	_widget(cart_gauge, "cart", Vector2(0.5, 1), Vector2(0, -112))

	# The hotbar with the experience line under it.
	var bottom := VBoxContainer.new()
	bottom.add_theme_constant_override("separation", 6)
	bottom.mouse_filter = Control.MOUSE_FILTER_IGNORE
	hotbar = Hotbar.new()
	hotbar.net = net
	hotbar.online = self
	hotbar.size_flags_horizontal = Control.SIZE_SHRINK_CENTER
	bottom.add_child(hotbar)
	xp_line = XpLine.new()
	bottom.add_child(xp_line)
	_widget(bottom, "hotbar", Vector2(0.5, 1), Vector2(0, -10))

	# The chat box bottom left, with messages (pickups, refused actions) above it.
	chat_window = ChatWindow.new()
	chat_window.net = net
	chat_window.current_target = func() -> int: return my_target
	_widget(chat_window, "chat", Vector2(0, 1), Vector2(12, -12))
	notice_box = Notices.new()
	_widget(notice_box, "notices", Vector2(0, 1), Vector2(12, -250))

	character_window = CharacterWindow.new()
	character_window.net = net
	character_window.visible = false
	_window(character_window, "character", "Character", Vector2(0, 0.35), Vector2(230, 0), true).adopt_header = false

	inventory_window = InventoryWindow.new()
	inventory_window.net = net
	inventory_window.online = self
	inventory_window.visible = false
	_window(inventory_window, "inventory", "Inventory", Vector2(1, 0.55), Vector2(-12, 0), true)

	store_window = StoreWindow.new()
	store_window.net = net
	store_window.online = self
	store_window.inventory_window = inventory_window
	store_window.visible = false
	_window(store_window, "store", "Store", Vector2(0.5, 0.5), Vector2(-160, 0)).close_method = "close_store"
	inventory_window.store_window = store_window

	bank_window = BankWindow.new()
	bank_window.net = net
	bank_window.online = self
	bank_window.inventory_window = inventory_window
	bank_window.visible = false
	_window(bank_window, "storage", "Storage", Vector2(0.5, 0.5), Vector2(-160, 0)).close_method = "close_bank"
	inventory_window.bank_window = bank_window

	party_window = PartyWindow.new()
	party_window.net = net
	party_window.visible = false
	_window(party_window, "party", "Party", Vector2(1, 1), Vector2(-12, -70), true)
	_popup(party_window.invite_panel, 60)

	craft_window = CraftWindow.new()
	craft_window.net = net
	craft_window.visible = false
	_window(craft_window, "craft", "Crafting", Vector2(0.5, 0.5), Vector2.ZERO)

	work_window = ItemWorkWindow.new()
	work_window.net = net
	work_window.online = self
	work_window.inventory_window = inventory_window
	work_window.visible = false
	_window(work_window, "item_work", "Refine", Vector2(0.5, 0.5), Vector2(-160, 0)).close_method = "close_window"
	inventory_window.work_window = work_window

	trade_window = TradeWindow.new()
	trade_window.net = net
	trade_window.online = self
	trade_window.inventory_window = inventory_window
	trade_window.visible = false
	_window(trade_window, "trade", "Trade", Vector2(0, 0.5), Vector2(20, 0)).close_method = "cancel_trade"
	_popup(trade_window.request_panel, 140)
	inventory_window.trade_window = trade_window

	# Right-click on another player.
	player_menu = PopupMenu.new()
	player_menu.add_item("Invite to party", 0)
	player_menu.add_item("Trade", 1)
	player_menu.add_item("Add friend", 2)
	player_menu.add_item("Visit shop", 3)
	player_menu.add_item("Offer a ride", 4)
	player_menu.id_pressed.connect(func(id):
		if _menu_player < 0:
			return
		if id == 0:
			net.party_invite(_menu_player)
		elif id == 1:
			net.trade_ask(_menu_player)
		elif id == 2 and entities.has(_menu_player):
			net.friend_ask(entities[_menu_player].label.text)
		elif id == 3:
			shop_window.browse(_menu_player)
		elif id == 4:
			net.ride_offer_to(_menu_player))
	layer.add_child(player_menu)

	skill_window = SkillWindow.new()
	skill_window.net = net
	skill_window.online = self
	skill_window.visible = false
	_window(skill_window, "skills", "Skills", Vector2(0, 0.35), Vector2(530, 0), true)

	conversation_window = ConversationWindow.new()
	conversation_window.net = net
	conversation_window.online = self
	conversation_window.visible = false
	_window(conversation_window, "conversation", "Conversation", Vector2(0, 0.5), Vector2(40, 0)).close_method = "close_conversation"

	quest_window = QuestWindow.new()
	quest_window.net = net
	quest_window.visible = false
	# On the right, so it stays clear of conversations on the left.
	_window(quest_window, "quests", "Quests", Vector2(1, 0.72), Vector2(-384, 0), true)

	revive_window = ReviveWindow.new()
	revive_window.net = net
	revive_window.visible = false
	revive_window.set_anchors_and_offsets_preset(Control.PRESET_FULL_RECT)
	ui_root.add_child(revive_window)

	shop_window = ShopWindow.new()
	shop_window.net = net
	shop_window.online = self
	shop_window.visible = false
	_window(shop_window, "shop", "Shop", Vector2(0, 0), Vector2(12, 150))

	friends_window = FriendsWindow.new()
	friends_window.net = net
	friends_window.chat_window = chat_window
	friends_window.visible = false
	_window(friends_window, "friends", "Friends", Vector2(1, 0.55), Vector2(-384, 0), true)
	_popup(friends_window.request_panel, 200)

	map_window = ZoneMap.new()
	map_window.online = self
	map_window.visible = false
	_window(map_window, "map", "Map", Vector2(0.5, 0.5), Vector2.ZERO, true)

	options_window = OptionsWindow.new()
	options_window.visible = false
	_window(options_window, "options", "Options", Vector2(0.5, 0.5), Vector2.ZERO)

	# A driver offers us a ride.
	ride_panel = PanelContainer.new()
	ride_panel.visible = false
	var ride_box := HBoxContainer.new()
	ride_box.add_theme_constant_override("separation", 8)
	ride_panel.add_child(ride_box)
	ride_label = Label.new()
	ride_box.add_child(ride_label)
	for answer in [["Get on", true], ["No thanks", false]]:
		var b := Button.new()
		b.text = answer[0]
		b.focus_mode = Control.FOCUS_NONE
		if answer[1]:
			b.theme_type_variation = "ButtonPrimary"
		b.pressed.connect(func():
			if _ride_offer >= 0:
				net.ride_answer(_ride_offer, answer[1])
			_ride_offer = -1
			ride_panel.visible = false)
		ride_box.add_child(b)
	_popup(ride_panel, 260)
	for frame in ui_root.get_children():
		if frame is UiWindow:
			frame.restore_open()

	hud.text = "Connecting to %s..." % uri
	if token != "":
		print("rose net: connecting to ", uri, " with an account token")
		return net.connect_with_token(uri, token)
	print("rose net: connecting to ", uri, " (identity in ", token_path, ")")
	return net.connect_to(uri, token_path)


## Puts a window in a frame (title bar, drag, scale, close) on the interface.
func _window(content: Control, id: String, title: String, anchor: Vector2, offset: Vector2, remember := false) -> UiWindow:
	var frame := UiWindow.wrap(content, id, title, anchor, offset)
	frame.remember_open = remember
	ui_root.add_child(frame)
	return frame


## Puts a HUD piece on the interface; the player can drag it.
func _widget(content: Control, id: String, anchor: Vector2, offset: Vector2) -> UiWidget:
	var widget := UiWidget.wrap(content, id, anchor, offset)
	ui_root.add_child(widget)
	return widget


## A question that pops up at the top centre (party invites, trade requests).
func _popup(panel: Control, from_top: float) -> void:
	if panel.get_parent():
		panel.get_parent().remove_child(panel)
	panel.theme_type_variation = "QuestionPanel"
	panel.set_anchors_and_offsets_preset(Control.PRESET_CENTER_TOP)
	panel.grow_horizontal = Control.GROW_DIRECTION_BOTH
	panel.offset_top = from_top
	ui_root.add_child(panel)


## The menu bar's buttons.
func _on_menu(which: String) -> void:
	match which:
		"character": toggle_character_window()
		"inventory": toggle_inventory_window()
		"skills": toggle_skill_window()
		"quests": toggle_quest_window()
		"party": toggle_party_window()
		"friends": toggle_friends_window()
		"map": toggle_map_window()
		"options": toggle_options_window()


## Esc: closes the window on top. False when none was open.
func close_top_window() -> bool:
	for i in range(ui_root.get_child_count() - 1, -1, -1):
		var child := ui_root.get_child(i)
		if child is UiWindow and child.visible:
			child.close()
			return true
	return false


## The main scene loaded the zone our character is in: rebuild everything on it.
func use_zone(zone_node: Node) -> void:
	zone = zone_node
	for entity in entities.values():
		entity.queue_free()
	entities.clear()
	for item in ground.values():
		item.queue_free()
	ground.clear()
	_pickup = -1
	_talk = -1
	store_window.close_store()
	bank_window.close_bank()
	work_window.close_window()
	conversation_window.close_conversation()
	me = null
	_zone_requested = 0


func toggle_character_window() -> void:
	character_window.visible = not character_window.visible


func toggle_inventory_window() -> void:
	inventory_window.visible = not inventory_window.visible


func toggle_skill_window() -> void:
	skill_window.visible = not skill_window.visible


func toggle_friends_window() -> void:
	friends_window.visible = not friends_window.visible


func toggle_quest_window() -> void:
	quest_window.visible = not quest_window.visible


func toggle_party_window() -> void:
	party_window.visible = not party_window.visible


func toggle_map_window() -> void:
	map_window.visible = not map_window.visible


func toggle_options_window() -> void:
	options_window.visible = not options_window.visible


## Use a skill from a skill slot: on the current target when it needs one (the server
## falls back to ourselves for skills that may), at the target's feet for area skills.
func use_skill(page: int, index: int, skill: Dictionary) -> void:
	if me == null:
		return
	if skill.get("passive", false):
		_notice("%s works on its own" % skill.get("name", "That skill"))
		return
	if skill.get("type", "") == "Create Window":
		var kind: String = net.craft_skill_kind(page, index)
		if kind == "refine" or kind == "disassemble":
			store_window.close_store()
			bank_window.close_bank()
			work_window.open_skill(kind, page, index, skill)
		elif kind == "" or not craft_window.open_skill(page, index, skill):
			_notice("%s isn't in the game yet" % skill.get("name", "That skill"))
		return
	var command: String = skill.get("command", "")
	if command == "PrivateStore":
		shop_window.open_setup()
		return
	if command == "AutoTarget":
		my_target = nearest_monster()
		return
	if command == "SelfTarget":
		my_target = my_id
		return
	# Basic actions (party invite, trade) work on the current target too.
	var target := my_target if skill.get("target", false) or command != "" else -1
	var at: Vector3 = me.position
	if skill.get("area", false) and my_target >= 0 and entities.has(my_target):
		at = entities[my_target].position
	_pickup = -1
	_talk = -1
	if target >= 0:
		_chasing = true
	net.cast_skill(page, index, target, at.x, at.z)
	if log_damage:
		print("rose net: use skill ", skill.get("name", "?"), " on ", target)


## Our personal shop is open: we stay put until it closes.
func _in_shop() -> bool:
	if me != null and me.shop_title != "":
		_notice("Close your shop first (X stands up)")
		return true
	return false


## On someone's back seat: we go where they drive (X gets off).
func _riding(tell := false) -> bool:
	if me != null and me.riding >= 0:
		if tell:
			_notice("You are riding with someone: X gets off")
		return true
	return false


## Driving with an empty tank: the vehicle won't move until refuelled.
func _out_of_fuel() -> bool:
	var c: Dictionary = net.get_character()
	if c.get("driving", false) and int(c.get("fuel", 0)) <= 0:
		_notice("Out of fuel: use Engine Fuel or get off")
		return true
	return false


## Walks to a point. Returns false if we can't move now (in a shop, out of fuel, a passenger).
func move_to(target: Vector3) -> bool:
	if me == null or _in_shop() or _out_of_fuel() or _riding(true):
		return false
	# Sticky target (Options > Gameplay): walking keeps the target selected, it only stops
	# the attack. Otherwise walking drops it, as in iROSE.
	if not UI.settings.get("sticky_target", false):
		my_target = -1
	_chasing = false
	_pickup = -1
	_talk = -1
	net.move_to(target.x, target.z)
	# A locked-in skill: the server walks us there once it takes effect, so don't run ahead.
	if not me.cast_locked:
		me.predict_move(target)
	return true


## A left click on the ground: walk there and drop iROSE's marker on the spot, a glowing
## crystal that falls in and fades out. It goes at once when we arrive or stop, or when
## another click sends us somewhere else.
func click_ground(target: Vector3) -> void:
	if not move_to(target):
		return
	_clear_click_marker()
	var fx := preload("res://scripts/fx.gd").find(self)
	if fx:
		_click_marker = fx.spawn_effect(CLICK_MARKER_EFFECT, target)


## Holding the left button: run on toward a new spot. The marker slides along with it
## instead of dropping in again. Returns false if we can't move now.
func steer_to(target: Vector3) -> bool:
	# Pressed against a wall: a spot further into it would only run us into it again
	# (and each new move restarts the server's path), so wait for one that leads away.
	if me and not me.last_wall.is_empty():
		var heading := Vector3(target.x - me.position.x, 0, target.z - me.position.z).normalized()
		if heading.dot(me.last_wall["direction"]) > 0.3:
			return false
	if not move_to(target):
		return false
	_steer_target = target
	if _click_marker and is_instance_valid(_click_marker):
		_click_marker.global_position = target
	return true


## The held button is let go: drop the marker where we will stop.
func end_steer() -> void:
	if _steer_target == null:
		return
	var target: Vector3 = _steer_target
	_steer_target = null
	if me == null or not me.is_moving or _going_for_target() or _pickup >= 0 or _talk >= 0:
		return
	_clear_click_marker()
	var fx := preload("res://scripts/fx.gd").find(self)
	if fx:
		_click_marker = fx.spawn_effect(CLICK_MARKER_EFFECT, target)


func _update_click_marker() -> void:
	if _click_marker == null:
		return
	if not is_instance_valid(_click_marker):
		_click_marker = null
	elif me == null or me.dead or not me.is_moving or _going_for_target() or _pickup >= 0 or _talk >= 0:
		_clear_click_marker()


## True while a click on a target, not on the ground, decides where we go: the click
## marker isn't ours then. With a sticky target, a target we only keep doesn't count.
func _going_for_target() -> bool:
	if UI.settings.get("sticky_target", false):
		return _chasing and my_target >= 0
	return my_target >= 0


## Drop the target (Esc with no window open).
func clear_target() -> void:
	my_target = -1
	_chasing = false


func _clear_click_marker() -> void:
	if _click_marker and is_instance_valid(_click_marker):
		_click_marker.queue_free()
	_click_marker = null


func attack(id: int) -> void:
	if me == null or not entities.has(id) or _in_shop():
		return
	my_target = id
	_chasing = true
	_pickup = -1
	_talk = -1
	net.attack(id)
	if log_damage:
		print("rose net: attack ", entities[id].label.text, " (entity ", id, ")")


func stop() -> void:
	my_target = -1
	_chasing = false
	net.stop()


## Sit down, or stand up (MP only comes back while sitting). A passenger gets off instead.
func sit() -> void:
	if me == null:
		return
	if _riding():
		net.ride_leave()
		return
	my_target = -1
	_chasing = false
	_pickup = -1
	_talk = -1
	net.sit()


## What a left click at this screen position would act on, in the order a click checks:
## {"kind": "item" | "monster" | "enemy" | "npc" | "player", "id": ...}, or {} for the
## ground. The cursor and the hover tooltip use it too, so they always show what a click
## does. Remembered for the rest of the frame.
func under_mouse(camera: Camera3D, screen_position: Vector2) -> Dictionary:
	var frame := Engine.get_process_frames()
	if frame == _under_frame and screen_position == _under_at:
		return _under
	_under_frame = frame
	_under_at = screen_position
	_under = {}
	var label: int = world_overlay.item_label_at(screen_position) if world_overlay else -1
	if label >= 0 and ground.has(label):
		_under = {"kind": "item", "id": label}
		return _under
	for kind in ["monster", "enemy", "npc", "friend"]:
		var id := _pick_entity(camera, screen_position, kind)
		if id >= 0:
			_under = {"kind": "player" if kind == "friend" else kind, "id": id}
			return _under
	var item := pick_item(camera, screen_position)
	if item >= 0:
		_under = {"kind": "item", "id": item}
	return _under


## True while an NPC's repair menu or a hammer waits for an item to repair.
func is_repairing() -> bool:
	return inventory_window != null and (inventory_window.repair_npc >= 0 or not inventory_window.repair_tool.is_empty())


## The monster under the mouse, or -1. Entities have no colliders yet, so this picks the
## one whose body centre is nearest the click on screen.
func pick_monster(camera: Camera3D, screen_position: Vector2) -> int:
	return _pick_entity(camera, screen_position, "monster")


## The town NPC under the mouse, or -1.
func pick_npc(camera: Camera3D, screen_position: Vector2) -> int:
	return _pick_entity(camera, screen_position, "npc")


## Another player under the mouse, or -1.
func pick_player(camera: Camera3D, screen_position: Vector2) -> int:
	return _pick_entity(camera, screen_position, "player")


## Another player under the mouse, fallen ones too (to heal or resurrect them), or -1.
func pick_friend(camera: Camera3D, screen_position: Vector2) -> int:
	return _pick_entity(camera, screen_position, "friend")


## Make an entity the target for skills without attacking it.
func select(id: int) -> void:
	if me == null or not entities.has(id):
		return
	my_target = id
	_chasing = true
	_pickup = -1
	_talk = -1
	# Clicking a player with a shop sign shows what they sell.
	if id != my_id and entities[id].get("shop_title") != null and entities[id].shop_title != "":
		shop_window.browse(id)


## A player we may fight here (PvP zones) under the mouse, or -1.
func pick_enemy_player(camera: Camera3D, screen_position: Vector2) -> int:
	return _pick_entity(camera, screen_position, "enemy")


## Right-click on another player: what we can do with them.
func open_player_menu(id: int, at: Vector2) -> void:
	_menu_player = id
	player_menu.position = Vector2i(at)
	player_menu.popup()


func _pick_entity(camera: Camera3D, screen_position: Vector2, kind := "monster") -> int:
	var best := -1
	var best_distance := PICK_RADIUS_PX
	for id in entities:
		var entity: Node3D = entities[id]
		var is_player: bool = not entity.is_npc and not entity.is_monster and entity != me
		var wanted: bool = entity.is_npc if kind == "npc" else (is_player if kind in ["player", "friend"] else entity.is_monster)
		if kind == "enemy":
			wanted = is_player and net.is_enemy_player(id)
		elif kind == "monster":
			wanted = entity.is_monster and not entity.is_summon
		if not wanted or not entity.visible or entity.dying or (entity.dead and kind != "friend"):
			continue
		var centre: Vector3 = entity.global_position + Vector3(0, entity.height * 0.5, 0)
		if camera.is_position_behind(centre):
			continue
		var d := camera.unproject_position(centre).distance_to(screen_position)
		if d < best_distance:
			best_distance = d
			best = id
	return best


## The ground item under the mouse, or -1.
func pick_item(camera: Camera3D, screen_position: Vector2) -> int:
	var best := -1
	var best_distance := ITEM_PICK_RADIUS_PX
	for id in ground:
		var item: Node3D = ground[id]
		if camera.is_position_behind(item.global_position):
			continue
		var d := camera.unproject_position(item.global_position + Vector3(0, 0.2, 0)).distance_to(screen_position)
		if d < best_distance:
			best_distance = d
			best = id
	return best


## Walk to an NPC and talk to it when there.
func talk_to(id: int) -> void:
	if me == null or not entities.has(id):
		return
	var at: Vector3 = entities[id].position
	if Vector2(at.x - me.position.x, at.z - me.position.z).length() > TALK_RANGE:
		move_to(at)
	_talk = id


## Open an NPC's conversation, or its store when it has nothing to say.
func _open_npc(id: int, npc: Node3D) -> void:
	store_window.close_store()
	bank_window.close_bank()
	work_window.close_window()
	if conversation_window.open(id, npc):
		return
	if not store_window.open_store(id, npc):
		_notice("%s has nothing to say" % npc.label.text)


## Nearest town NPC with a store, or -1.
func nearest_store() -> int:
	var best := -1
	var best_distance := INF
	if me == null:
		return best
	for id in entities:
		var entity: Node3D = entities[id]
		if entity.is_npc and entity.has_store:
			var d: float = entity.position.distance_to(me.position)
			if d < best_distance:
				best_distance = d
				best = id
	return best


## Walk to a ground item and pick it up when there.
func pickup(id: int) -> void:
	if me == null or not ground.has(id):
		return
	var at: Vector3 = ground[id].position
	if Vector2(at.x - me.position.x, at.z - me.position.z).length() > PICKUP_RANGE:
		move_to(at)
	_pickup = id


## Nearest ground item we may take, within 10 m, or -1.
func nearest_item() -> int:
	var best := -1
	var best_distance := 10.0
	if me == null:
		return best
	for id in ground:
		var item: Node3D = ground[id]
		var d: float = item.position.distance_to(me.position)
		if item.get_meta("mine", true) and d < best_distance:
			best_distance = d
			best = id
	return best


## Nearest live monster to our character, or -1.
func nearest_monster() -> int:
	var best := -1
	var best_distance := INF
	if me == null:
		return best
	for id in entities:
		var entity: Node3D = entities[id]
		if entity.is_monster and not entity.is_summon and not entity.dead and not entity.dying:
			var d: float = entity.position.distance_to(me.position)
			if d < best_distance:
				best_distance = d
				best = id
	return best


func _process(_delta: float) -> void:
	if net == null:
		return
	if not net.is_online():
		var error := net.get_error()
		if error == "":
			error = net.get_connection_error()
		hud.text = "Not connected" + (": " + error if error != "" else "")
		if error != "" and not _failed:
			_failed = true
			connection_failed.emit(error)
		return
	var notice := net.get_server_notice()
	if notice != "":
		hud.text = notice
		return
	if my_id < 0:
		# A new account makes its character first.
		if net.needs_character():
			if character_create == null:
				character_create = CharacterCreate.new()
				character_create.net = net
				add_child(character_create)
				if create_preset != "":
					character_create.preset(create_preset)
				character_create.auto_create_after = create_after
				_layer.visible = false
				hud.text = ""
			return
		if not net.is_ready():
			return
		if character_create:
			character_create.queue_free()
			character_create = null
			_layer.visible = true
		my_id = net.my_entity_id()
		if player_name != "":
			net.set_player_name(player_name)
		print("rose net: signed in as entity ", my_id)
	if ranged and not _ranged_equipped and Time.get_ticks_msec() >= _next_equip_try_ms:
		_next_equip_try_ms = Time.get_ticks_msec() + 2000
		_ranged_equipped = _equip_bow()

	var character: Dictionary = net.get_character()
	var my_zone: int = character.get("zone", 0)
	if my_zone > 0 and my_zone != zone.get_meta("zone_id", 0) and my_zone != _zone_requested:
		_zone_requested = my_zone
		print("rose net: our character is in zone ", my_zone)
		zone_needed.emit(my_zone)
		return
	if _zone_requested > 0:
		return

	var perf_start := Time.get_ticks_usec()
	var states: Array = net.get_entities()
	_perf_entities_us += Time.get_ticks_usec() - perf_start
	_perf_frames += 1
	var by_id := {}
	for state in states:
		by_id[state["id"]] = state

	# Damage first, so a kill is known before the monster's removal is handled.
	for hit in net.poll_damage_events():
		_on_damage(hit)
	var gained := 0
	for ev in net.poll_xp_events():
		gained += ev["xp"]

	for id in entities.keys():
		if not by_id.has(id):
			var entity: Node3D = entities[id]
			entities.erase(id)
			if _killed.has(id):
				_killed.erase(id)
				entity.die_and_free()
			else:
				entity.queue_free()

	for state in states:
		var id: int = state["id"]
		if not entities.has(id):
			var entity := NetEntity.new()
			entity.name = "Entity%d" % id
			add_child(entity)
			entity.setup(zone, state, id == my_id)
			entities[id] = entity
			if id == my_id:
				entity.collided.connect(_on_collided)
			if state["kind"] == "player":
				print("rose net: ", state["name"], " is here (entity ", id, ")")
		var target_position = null
		var target_state = by_id.get(state.get("target", -1))
		if target_state != null:
			target_position = Vector3(target_state["x"], 0, target_state["z"])
		entities[id].update_state(state, target_position)
		# Town NPCs and players outside the sectors around us are known (the zone map shows
		# them) but not drawn; monsters out there are not sent at all.
		entities[id].visible = not state.get("far", false)

	if me == null and entities.has(my_id):
		me = entities[my_id]
		joined.emit(me)
	# The server's attack target wins (a monster that hits us back, a kill that ends it).
	if by_id.has(my_id):
		var server_target: int = by_id[my_id].get("target", -1)
		if server_target >= 0:
			my_target = server_target
			_chasing = true
	if not entities.has(my_target):
		my_target = -1
		_chasing = false

	_update_ground()
	_update_click_marker()
	_check_warps()
	if Time.get_ticks_msec() >= _next_effects_ms:
		_next_effects_ms = Time.get_ticks_msec() + 500
		_update_effects()
	for text in net.poll_notices():
		if not chat_window.take_notice(text):
			_notice(text)
		elif log_damage:
			print("rose net: notice: ", text)
	for m in net.poll_chat():
		chat_window.add_message(m)
		if log_damage:
			print("rose net: chat %s %s%s: %s" % [m["channel"], m["from"], " -> " + m["to"] if m["to"] != "" else "", m["text"]])
		# Players nearby see what was said over the speaker's head.
		if m["channel"] == "nearby" and entities.has(m["entity"]):
			entities[m["entity"]].say(m["text"])
	if me and _pickup >= 0:
		if not ground.has(_pickup):
			_pickup = -1
		else:
			var at: Vector3 = ground[_pickup].position
			if Vector2(at.x - me.position.x, at.z - me.position.z).length() <= PICKUP_RANGE:
				net.pickup_item(_pickup)
				_pickup = -1
	if me and _talk >= 0:
		if not entities.has(_talk):
			_talk = -1
		else:
			var npc: Node3D = entities[_talk]
			var d := Vector2(npc.position.x - me.position.x, npc.position.z - me.position.z).length()
			if d <= TALK_RANGE or (not me.is_moving and d <= TALK_STOPPED_RANGE):
				net.stop()
				_open_npc(_talk, npc)
				_talk = -1
	for window in net.poll_windows():
		var npc_id: int = window[1]
		if window[0] == "store" and entities.has(npc_id):
			if not store_window.open_store(npc_id, entities[npc_id]):
				_notice("%s has nothing to sell" % entities[npc_id].label.text)
		elif window[0] == "bank" and entities.has(npc_id):
			store_window.close_store()
			bank_window.open_bank(npc_id, entities[npc_id])
		elif (window[0] == "refine" or window[0] == "disassemble") and entities.has(npc_id):
			store_window.close_store()
			work_window.open_npc(window[0], npc_id, entities[npc_id])
		elif window[0] == "repair" and entities.has(npc_id):
			store_window.close_store()
			inventory_window.start_npc_repair(npc_id, entities[npc_id])

	# Name tags and item labels are drawn by world_overlay.gd.
	var pvp: bool = net.zone_pvp() > 0
	world_overlay.pvp = pvp

	if log_perf and Time.get_ticks_msec() >= _next_perf_ms:
		_next_perf_ms = Time.get_ticks_msec() + 2000
		var drawn := 0
		var monsters := 0
		for entity in entities.values():
			if entity.visible:
				drawn += 1
				if entity.is_monster:
					monsters += 1
		print("rose perf: fps %d  known %d  nodes %d  drawn %d  monsters drawn %d  get_entities %.2f ms  frame %.2f ms  objects %d  draw calls %d  scene nodes %d" % [
			Engine.get_frames_per_second(), states.size(), entities.size(), drawn, monsters,
			_perf_entities_us / 1000.0 / maxi(_perf_frames, 1),
			Performance.get_monitor(Performance.TIME_PROCESS) * 1000.0,
			Performance.get_monitor(Performance.RENDER_TOTAL_OBJECTS_IN_FRAME),
			Performance.get_monitor(Performance.RENDER_TOTAL_DRAW_CALLS_IN_FRAME),
			Performance.get_monitor(Performance.OBJECT_NODE_COUNT)])
		_perf_entities_us = 0
		_perf_frames = 0
	if log_positions and Time.get_ticks_msec() >= _next_log_ms:
		_next_log_ms = Time.get_ticks_msec() + 1000
		var line := []
		for entity in entities.values():
			if not entity.is_monster and not entity.is_npc:
				line.append("%s (%.2f, %.2f) %s" % [entity.label.text, entity.position.x, entity.position.z, entity.anim.current_animation])
		print("rose net: t=%d ms  " % Time.get_ticks_msec(), ", ".join(line))

	var c := character
	var level: int = c.get("level", 0)
	hud.text = ""
	if me and me.riding >= 0 and entities.has(me.riding):
		hud.text = "Riding with %s (X gets off)" % entities[me.riding].label.text
	minimap.pvp = pvp
	player_frame.refresh(c, me)
	cart_gauge.refresh(c)
	xp_line.refresh(c)
	if Time.get_ticks_msec() >= _next_party_ms:
		_next_party_ms = Time.get_ticks_msec() + 300
		var party: Dictionary = net.get_party()
		if ui_sample and party.is_empty():
			party = {"members": [
				{"identity": "a", "name": "Helper", "level": 31, "online": true, "leader": true, "hp": 520, "max_hp": 610},
				{"identity": "b", "name": "Lowbie", "level": 12, "online": true, "hp": 88, "max_hp": 240},
				{"identity": "c", "name": "Mairin", "level": 27, "online": false, "hp": 0, "max_hp": 450}]}
		party_frames.refresh(party)
	if Time.get_ticks_msec() >= _next_quests_ms:
		_next_quests_ms = Time.get_ticks_msec() + 1000
		quest_tracker.refresh(net.get_quests())
	var offers: Array = net.get_ride_offers()
	if offers.is_empty():
		_ride_offer = -1
		ride_panel.visible = false
	elif int(offers[0][0]) != _ride_offer:
		_ride_offer = int(offers[0][0])
		ride_label.text = "%s offers you a ride" % offers[0][1]
		ride_panel.visible = true
	if ui_sample_screen == "death":
		revive_window.refresh({"fallen": true, "penalty_xp": 1240, "auto_revive_in": 547, "save_zone": "Zant"})
	else:
		revive_window.refresh(c)
	if Time.get_ticks_msec() >= _next_signs_ms:
		_next_signs_ms = Time.get_ticks_msec() + 400
		var titles: Dictionary = net.get_store_titles()
		for id in entities:
			var entity: Node3D = entities[id]
			if entity.has_method("show_shop_sign") and not entity.is_monster and not entity.is_npc:
				entity.show_shop_sign(titles.get(id, ""))
	if me:
		if gained > 0:
			_float_text(me, "+%d XP" % gained, Color(0.6, 0.9, 1.0), 26)
		if _level > 0 and level > _level:
			_float_text(me, "Level up!", Color(1.0, 0.85, 0.2), 40, 1.6)
			var fx := preload("res://scripts/fx.gd").find(self)
			if fx:
				fx.level_up(me)
	if level > 0:
		_level = level
	if my_target >= 0 and my_target != my_id:
		var target: Node3D = entities[my_target]
		var enemy_player: bool = pvp and net.is_enemy_player(my_target)
		target_frame.refresh(target, target.is_monster or enemy_player)
		target_marker.follow(target, enemy_player)
	else:
		target_frame.refresh(null, false)
		target_marker.follow(null, false)


## Walking into a warp gate's model asks the server to send us through (as the Bevy client
## does when its collision ray hits a warp object), at most once every five seconds.
func _check_warps() -> void:
	if me == null:
		return
	if _warps_zone != zone:
		_warps_zone = zone
		_warps = zone.get_warps()
		_warp_armed = false
		_near_gate_zone = 0
		print("rose net: %d warp gates in this zone" % _warps.size())
	_check_near_gate()
	if Time.get_ticks_msec() < _next_warp_ms:
		return
	var inside := -1
	for warp in _warps:
		# Gates often have a wall right in front of their model, so count the last few
		# metres before it as inside.
		var box := AABB(warp["position"], warp["size"]).grow(WARP_MARGIN)
		var feet := me.position + Vector3(0, 0.5, 0)
		if box.has_point(Vector3(feet.x, clamp(feet.y, box.position.y, box.end.y), feet.z)):
			inside = warp["id"]
			break
	if inside < 0:
		_warp_armed = true
	elif _warp_armed:
		_warp_armed = false
		_next_warp_ms = Time.get_ticks_msec() + 5000
		print("rose net: entering warp gate ", inside)
		net.use_warp_gate(inside)


## Within GATE_NEAR metres of a warp gate the zone behind it is read ahead (main.gd's
## zone_preload); further than GATE_FAR from every gate it is let go again. The gap keeps
## walking along the edge from starting and stopping it.
const GATE_NEAR := 40.0
const GATE_FAR := 60.0

func _check_near_gate() -> void:
	var nearest_id := -1
	var nearest := INF
	for warp in _warps:
		var box := AABB(warp["position"], warp["size"])
		var feet: Vector3 = me.position
		var closest := Vector3(clampf(feet.x, box.position.x, box.end.x), feet.y, clampf(feet.z, box.position.z, box.end.z))
		var distance := Vector2(feet.x - closest.x, feet.z - closest.z).length()
		if distance < nearest:
			nearest = distance
			nearest_id = warp["id"]
	var target := _near_gate_zone
	if nearest <= GATE_NEAR:
		target = RoseZone.warp_target_zone(nearest_id)
		if target == zone.get_meta("zone_id", 0):
			target = 0
	elif nearest > GATE_FAR:
		target = 0
	if target != _near_gate_zone:
		_near_gate_zone = target
		warp_gate_near.emit(target)


## Our buffs and debuffs as icons at the top of the screen, with what they are and how long they last.
func _update_effects() -> void:
	var effects: Array = net.get_status_effects(my_id) if my_id >= 0 else []
	if ui_sample:
		for e in [["Blessing", 1590, 312.0, false], ["Haste", 1588, 95.0, false], ["Shield", 1724, 41.0, false], ["Poisoned", 1785, 12.0, true], ["Slowed", 1735, 6.0, true]]:
			effects.append({"name": e[0], "icon": RoseData.item_icon(e[1]), "seconds": e[2], "bad": e[3]})
	effects_rows.refresh(effects)


## Show the server's ground items: the item's ground model with its name over it.
func _update_ground() -> void:
	var seen := {}
	for state in net.get_ground_items():
		var id: int = state["id"]
		seen[id] = true
		var item: Dictionary = state["item"]
		if not ground.has(id):
			var node := Node3D.new()
			var model := RoseFieldItem.new()
			model.build(item.get("model", ""))
			node.add_child(model)
			add_child(node)
			var x: float = state["x"]
			var z: float = state["z"]
			node.position = Vector3(x, zone.get_terrain_height(x, z), z)
			ground[id] = node
		ground[id].set_meta("mine", state["mine"])
		ground[id].set_meta("item", item)
		ground[id].set_meta("drop_id", id)
	for id in ground.keys():
		if not seen.has(id):
			if id == _pickup and me and me.position.distance_to(ground[id].position) < PICKUP_RANGE + 1.0:
				var fx := preload("res://scripts/fx.gd").find(self)
				if fx:
					fx.picked_up()
			ground[id].queue_free()
			ground.erase(id)


func _notice(text: String) -> void:
	notice_box.add(text)
	if log_damage:
		print("rose net: notice: ", text)


## Equip the first bow in the bag and the first arrows. True once both are on.
func _equip_bow() -> bool:
	var inventory: Dictionary = net.get_inventory()
	if inventory.is_empty():
		return false
	var weapon = inventory["equipped"][6]
	var has_bow: bool = weapon != null and weapon.get("class", "") == "Bow"
	var has_arrows: bool = inventory["ammo"][0] != null
	if not has_bow:
		var items: Array = inventory["pages"][0]
		for i in items.size():
			if items[i] != null and items[i].get("class", "") == "Bow":
				net.equip_item(0, i)
				break
	if not has_arrows:
		var materials: Array = inventory["pages"][2]
		for i in materials.size():
			if materials[i] != null and materials[i].get("class", "") == "Arrow":
				net.equip_item(2, i)
				break
	return has_bow and has_arrows


func _on_collided(at: Vector3) -> void:
	net.move_collision(at.x, at.z)
	if log_damage:
		print("rose net: hit a wall at (%.2f, %.2f)" % [at.x, at.z])


func _on_damage(hit: Dictionary) -> void:
	var defender: Node3D = entities.get(hit["defender"])
	var attacker: Node3D = entities.get(hit["attacker"])
	if log_damage:
		print("rose net: %s hits %s for %d%s%s" % [
			attacker.label.text if attacker else "?", defender.label.text if defender else "?",
			hit["amount"], " (critical)" if hit["critical"] else "", ", killed" if hit["killed"] else ""])
	if hit["killed"]:
		_killed[hit["defender"]] = true
	if defender == null:
		return
	defender.on_hit()
	_float_number(defender, hit)


## Damage number that rises and fades above the one that was hit.
func _float_number(defender: Node3D, hit: Dictionary) -> void:
	var colour := Color.WHITE
	if hit["defender"] == my_id:
		colour = Color(1.0, 0.35, 0.3)
	elif hit["critical"]:
		colour = Color(1.0, 0.85, 0.2)
	_float_text(defender, str(hit["amount"]) if hit["amount"] > 0 else "Miss", colour, 40 if hit["critical"] else 30)


## Text that rises and fades above an entity.
func _float_text(over: Node3D, text: String, colour: Color, size: int, seconds := 1.0) -> void:
	world_overlay.float_text(over, text, colour, size, seconds)
