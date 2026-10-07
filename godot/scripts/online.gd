## Online play: connects to the SpacetimeDB server and shows every player and monster in
## the zone, each moving along the server's motion paths, with hits and damage numbers.
extends Node3D

signal joined(me: Node3D)
signal zone_needed(zone_id: int)

const NetEntity := preload("res://scripts/net_entity.gd")
const CharacterWindow := preload("res://scripts/character_window.gd")
const PICK_RADIUS_PX := 60.0
const MONSTER_NAME_RANGE := 15.0  # monster names show within this many metres, or when targeted

var zone: Node
var net: RoseNet
var entities := {}  # entity id -> NetEntity
var me: Node3D
var my_id := -1
var my_target := -1
var player_name := ""
var ranged := false
var hud: Label
var target_hud: Label
var xp_bar: ProgressBar
var xp_label: Label
var character_window: PanelContainer
var _level := 0
var _zone_requested := 0
var log_damage := false
var log_positions := false  # print every player's position once a second
var _next_log_ms := 0
var _killed := {}  # entity ids whose last hit killed them


func start(zone_node: Node, uri: String, token_path: String, name_text: String, use_bow: bool) -> bool:
	zone = zone_node
	player_name = name_text
	ranged = use_bow
	net = RoseNet.new()
	net.name = "Net"
	add_child(net)

	var layer := CanvasLayer.new()
	add_child(layer)
	hud = _hud_label(layer, Vector2(12, 8))
	target_hud = _hud_label(layer, Vector2(12, 30))
	target_hud.add_theme_font_size_override("font_size", 20)

	# Experience bar along the bottom of the screen.
	xp_bar = ProgressBar.new()
	xp_bar.show_percentage = false
	xp_bar.set_anchors_and_offsets_preset(Control.PRESET_BOTTOM_WIDE)
	xp_bar.offset_top = -14
	xp_bar.max_value = 1.0
	xp_bar.mouse_filter = Control.MOUSE_FILTER_IGNORE
	layer.add_child(xp_bar)
	xp_label = _hud_label(layer, Vector2.ZERO)
	xp_label.set_anchors_and_offsets_preset(Control.PRESET_CENTER_BOTTOM)
	xp_label.offset_top = -34
	xp_label.grow_horizontal = Control.GROW_DIRECTION_BOTH
	xp_label.horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER

	character_window = CharacterWindow.new()
	character_window.net = net
	character_window.visible = false
	character_window.position = Vector2(12, 70)
	layer.add_child(character_window)

	print("rose net: connecting to ", uri, " (identity in ", token_path, ")")
	hud.text = "Connecting to %s..." % uri
	return net.connect_to(uri, token_path)


func _hud_label(layer: CanvasLayer, at: Vector2) -> Label:
	var label := Label.new()
	label.position = at
	label.add_theme_color_override("font_shadow_color", Color.BLACK)
	label.add_theme_constant_override("shadow_offset_x", 1)
	label.add_theme_constant_override("shadow_offset_y", 1)
	layer.add_child(label)
	return label


## The main scene loaded the zone our character is in: rebuild everything on it.
func use_zone(zone_node: Node) -> void:
	zone = zone_node
	for entity in entities.values():
		entity.queue_free()
	entities.clear()
	me = null
	_zone_requested = 0


func toggle_character_window() -> void:
	character_window.visible = not character_window.visible


func move_to(target: Vector3) -> void:
	if me == null:
		return
	my_target = -1
	net.move_to(target.x, target.z)
	me.predict_move(target)


func attack(id: int) -> void:
	if me == null or not entities.has(id):
		return
	my_target = id
	net.attack(id)
	if log_damage:
		print("rose net: attack ", entities[id].label.text, " (entity ", id, ")")


func stop() -> void:
	my_target = -1
	net.stop()


## The monster under the mouse, or -1. Entities have no colliders yet, so this picks the
## one whose body centre is nearest the click on screen.
func pick_monster(camera: Camera3D, screen_position: Vector2) -> int:
	var best := -1
	var best_distance := PICK_RADIUS_PX
	for id in entities:
		var entity: Node3D = entities[id]
		if not entity.is_monster or entity.dead or entity.dying:
			continue
		var centre: Vector3 = entity.global_position + Vector3(0, entity.height * 0.5, 0)
		if camera.is_position_behind(centre):
			continue
		var d := camera.unproject_position(centre).distance_to(screen_position)
		if d < best_distance:
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
		if entity.is_monster and not entity.dead and not entity.dying:
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
		hud.text = "Not connected" + (": " + error if error != "" else "")
		return
	var notice := net.get_server_notice()
	if notice != "":
		hud.text = notice
		return
	if my_id < 0:
		if not net.is_ready():
			return
		my_id = net.my_entity_id()
		if player_name != "":
			net.set_player_name(player_name)
		net.set_loadout(ranged)
		print("rose net: signed in as entity ", my_id)

	var character: Dictionary = net.get_character()
	var my_zone: int = character.get("zone", 0)
	if my_zone > 0 and my_zone != zone.get_meta("zone_id", 0) and my_zone != _zone_requested:
		_zone_requested = my_zone
		print("rose net: our character is in zone ", my_zone)
		zone_needed.emit(my_zone)
		return
	if _zone_requested > 0:
		return

	var states: Array = net.get_entities()
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

	if me == null and entities.has(my_id):
		me = entities[my_id]
		joined.emit(me)
	# The server's attack target wins (a monster that hits us back, a kill that ends it).
	if by_id.has(my_id):
		var server_target: int = by_id[my_id].get("target", -1)
		if server_target >= 0:
			my_target = server_target
	if not entities.has(my_target):
		my_target = -1

	if me:
		for id in entities:
			var entity: Node3D = entities[id]
			if entity.is_monster and not entity.dying:
				entity.label.visible = id == my_target or entity.position.distance_to(me.position) < MONSTER_NAME_RANGE

	if log_positions and Time.get_ticks_msec() >= _next_log_ms:
		_next_log_ms = Time.get_ticks_msec() + 1000
		var line := []
		for entity in entities.values():
			if not entity.is_monster:
				line.append("%s (%.2f, %.2f) %s" % [entity.label.text, entity.position.x, entity.position.z, entity.anim.current_animation])
		print("rose net: t=%d ms  " % Time.get_ticks_msec(), ", ".join(line))

	var monsters := 0
	for entity in entities.values():
		monsters += 1 if entity.is_monster else 0
	var c := character
	var level: int = c.get("level", 0)
	hud.text = "%s   Lv %d   HP %d/%d   MP %d/%d   players %d   monsters %d" % [
		c.get("name", player_name), level, me.hp if me else 0, me.max_hp if me else 0,
		me.mp if me else 0, me.max_mp if me else 0, entities.size() - monsters, monsters]
	if not c.is_empty():
		var needed: int = max(int(c["xp_needed"]), 1)
		xp_bar.value = float(c["xp"]) / needed
		xp_label.text = "XP %d / %d  (%.1f%%)" % [c["xp"], needed, 100.0 * c["xp"] / needed]
		if c["stat_points"] > 0:
			xp_label.text += "   %d stat points (C)" % c["stat_points"]
	if me:
		if gained > 0:
			_float_text(me, "+%d XP" % gained, Color(0.6, 0.9, 1.0), 26)
		if _level > 0 and level > _level:
			_float_text(me, "Level up!", Color(1.0, 0.85, 0.2), 40, 1.6)
	if level > 0:
		_level = level
	if my_target >= 0:
		var target: Node3D = entities[my_target]
		target_hud.text = "%s   Lv %d   HP %d/%d" % [target.label.text, target.level, target.hp, target.max_hp]
	else:
		target_hud.text = ""


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
	var label := Label3D.new()
	label.text = text
	label.billboard = BaseMaterial3D.BILLBOARD_ENABLED
	label.no_depth_test = true
	label.fixed_size = true
	label.pixel_size = 0.0015
	label.font_size = size
	label.outline_size = 8
	label.modulate = colour
	add_child(label)
	label.global_position = over.global_position + Vector3(randf_range(-0.3, 0.3), over.height * 0.8, 0)
	var tween := label.create_tween()
	tween.set_parallel(true)
	tween.tween_property(label, "global_position:y", label.global_position.y + 1.2, seconds)
	tween.tween_property(label, "modulate:a", 0.0, seconds).set_delay(seconds * 0.4)
	tween.chain().tween_callback(label.queue_free)
