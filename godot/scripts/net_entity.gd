## A character shown from server state. It stands where the server's motion path puts it,
## faces where it runs or what it fights, and plays run, stop, attack and die.
##
## Our own character also predicts a click-to-move: it starts running at once and hands
## over to the server's path when that arrives.
extends Node3D

const BLEND := 0.12
const RUN_SPEED := 4.505  # player move_speed 450.5 cm/s on the server
const SWORD := 2  # Short Sword
const BOW := 202  # Short Bow
const PREDICTION_TIMEOUT_MS := 1000

var zone: Node
var character: Node3D
var anim: AnimationPlayer
var label: Label3D
var entity_id := -1
var ranged := false
var dead := false
var was_swinging := false
var attack_index := 0
var predicted := {}  # from, to (Vector3), started (msec)


func setup(zone_node: Node, state: Dictionary, is_me: bool) -> void:
	zone = zone_node
	entity_id = state["id"]
	ranged = state.get("ranged", false)
	_build()
	label = Label3D.new()
	label.billboard = BaseMaterial3D.BILLBOARD_ENABLED
	label.no_depth_test = true
	label.fixed_size = true
	label.pixel_size = 0.0015
	label.font_size = 24
	label.outline_size = 6
	label.modulate = Color(1.0, 0.95, 0.6) if is_me else Color(1, 1, 1)
	label.position.y = 2.1
	add_child(label)
	update_state(state, null)


func _build() -> void:
	if character:
		character.queue_free()
	character = RoseCharacter.new()
	character.name = "Character"
	add_child(character)
	character.build(true, 1, 0, 0, 1, 1, 1, BOW if ranged else SWORD, 0)
	anim = character.get_node("AnimationPlayer")
	anim.animation_finished.connect(_on_animation_finished)
	_play("stop1")


## Starts running toward a Godot position before the server confirms the move.
func predict_move(target: Vector3) -> void:
	if dead:
		return
	predicted = {"from": Vector3(position.x, 0, position.z), "to": Vector3(target.x, 0, target.z), "started": Time.get_ticks_msec()}
	was_swinging = false
	_play("run", true)


## Applies one entry from RoseNet.get_entities(). target_position is where the entity we
## attack stands, or null.
func update_state(state: Dictionary, target_position) -> void:
	var name_text: String = state["name"]
	if label.text != name_text:
		label.text = name_text
	if state.get("ranged", false) != ranged:
		ranged = state["ranged"]
		_build()

	var server_pos := Vector3(state["x"], 0, state["z"])
	var server_to := Vector3(state["to_x"], 0, state["to_z"])
	var moving: bool = state["moving"]
	var flat := server_pos
	if not predicted.is_empty():
		var elapsed := (Time.get_ticks_msec() - int(predicted["started"])) / 1000.0
		if server_to.distance_to(predicted["to"]) < 0.05 or elapsed * 1000.0 > PREDICTION_TIMEOUT_MS:
			predicted = {}  # the server has the move now
		else:
			var from: Vector3 = predicted["from"]
			var to: Vector3 = predicted["to"]
			var travel := minf(elapsed * RUN_SPEED, from.distance_to(to))
			flat = from + (to - from).normalized() * travel if from.distance_to(to) > 0.01 else to
			moving = travel < from.distance_to(to)
			server_to = to

	var heading := server_to - flat if moving else Vector3.ZERO
	if not moving and target_position != null:
		heading = target_position - flat
	if Vector2(heading.x, heading.z).length() > 0.01:
		rotation.y = atan2(heading.x, heading.z)
	position = Vector3(flat.x, zone.get_terrain_height(flat.x, flat.z), flat.z)

	var is_dead: bool = state.get("dead", false)
	var swinging: bool = state.get("swinging", false) and predicted.is_empty()
	if is_dead:
		if not dead:
			_play("die", true)
	elif moving:
		_play("run")  # moving cancels a swing at once
	elif swinging and not was_swinging:
		_swing()
	elif not _is_attacking():
		_play("stop1")
	dead = is_dead
	was_swinging = swinging


func _swing() -> void:
	var names := ["attack", "attack2", "attack3"]
	var name: String = names[attack_index % names.size()]
	attack_index += 1
	if not anim.has_animation(name):
		name = "attack"
	_play(name, true)


func _is_attacking() -> bool:
	return anim.is_playing() and String(anim.current_animation).begins_with("attack")


func _play(name: String, restart := false) -> void:
	if anim == null or not anim.has_animation(name):
		return
	if anim.current_animation == name and not restart:
		return
	if restart:
		anim.stop()
	anim.play(name, BLEND)


func _on_animation_finished(name: StringName) -> void:
	if String(name).begins_with("attack") and not dead:
		_play("stop1")
