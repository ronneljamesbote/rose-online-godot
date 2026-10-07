## Online play: connects to the SpacetimeDB server and shows every player in the zone,
## each moving along the server's motion paths.
extends Node3D

signal joined(me: Node3D)

const NetEntity := preload("res://scripts/net_entity.gd")

var zone: Node
var net: RoseNet
var entities := {}  # entity id -> NetEntity
var me: Node3D
var my_id := -1
var player_name := ""
var ranged := false
var hud: Label
var log_damage := false
var log_positions := false  # print every player's position once a second
var _next_log_ms := 0


func start(zone_node: Node, uri: String, token_path: String, name_text: String, use_bow: bool) -> bool:
	zone = zone_node
	player_name = name_text
	ranged = use_bow
	net = RoseNet.new()
	net.name = "Net"
	add_child(net)

	var layer := CanvasLayer.new()
	add_child(layer)
	hud = Label.new()
	hud.position = Vector2(12, 8)
	hud.add_theme_color_override("font_shadow_color", Color.BLACK)
	hud.add_theme_constant_override("shadow_offset_x", 1)
	hud.add_theme_constant_override("shadow_offset_y", 1)
	layer.add_child(hud)

	print("rose net: connecting to ", uri, " (identity in ", token_path, ")")
	hud.text = "Connecting to %s..." % uri
	return net.connect_to(uri, token_path)


func move_to(target: Vector3) -> void:
	if me == null:
		return
	net.move_to(target.x, target.z)
	me.predict_move(target)


func stop() -> void:
	net.stop()


func players() -> Array:
	return entities.values()


func _process(_delta: float) -> void:
	if net == null:
		return
	if not net.is_online():
		var error := net.get_error()
		hud.text = "Not connected" + (": " + error if error != "" else "")
		return
	if my_id < 0:
		if not net.is_ready():
			return
		my_id = net.my_entity_id()
		if player_name != "":
			net.set_player_name(player_name)
		net.set_loadout(ranged)
		print("rose net: signed in as entity ", my_id)

	var states: Array = net.get_entities()
	var by_id := {}
	for state in states:
		by_id[state["id"]] = state

	for id in entities.keys():
		var state = by_id.get(id)
		if state == null or state["kind"] != "player":
			print("rose net: ", entities[id].label.text, " left")
			entities[id].queue_free()
			entities.erase(id)

	for state in states:
		if state["kind"] != "player":
			continue  # monsters come in the next step
		var id: int = state["id"]
		if not entities.has(id):
			var entity := NetEntity.new()
			entity.name = "Player%d" % id
			add_child(entity)
			entity.setup(zone, state, id == my_id)
			entities[id] = entity
			print("rose net: ", state["name"], " is here (entity ", id, ")")
		var target_position = null
		var target_state = by_id.get(state.get("target", -1))
		if target_state != null:
			target_position = Vector3(target_state["x"], 0, target_state["z"])
		entities[id].update_state(state, target_position)

	if me == null and entities.has(my_id):
		me = entities[my_id]
		joined.emit(me)

	for hit in net.poll_damage_events():
		if log_damage:
			print("rose net: hit ", hit)

	if log_positions and Time.get_ticks_msec() >= _next_log_ms:
		_next_log_ms = Time.get_ticks_msec() + 1000
		var line := []
		for entity in entities.values():
			line.append("%s (%.2f, %.2f) %s" % [entity.label.text, entity.position.x, entity.position.z, entity.anim.current_animation])
		print("rose net: t=%d ms  " % Time.get_ticks_msec(), ", ".join(line))

	hud.text = "Online as %s   players here: %d   clock offset %.0f ms" % [
		me.label.text if me else player_name, entities.size(), net.clock_offset_ms()]
