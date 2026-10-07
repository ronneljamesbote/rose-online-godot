## A player, monster or town NPC shown from server state. It stands where the server's motion path
## puts it, faces where it runs or what it fights, and plays run, stop, attack, hit and die.
## A swing is timed so its hit frame lands when the server resolves the hit.
##
## Our own character also predicts a click-to-move: it starts running at once and hands
## over to the server's path when that arrives.
extends Node3D

const BLEND := 0.12
const RUN_SPEED := 4.505  # player move_speed 450.5 cm/s on the server
const PREDICTION_TIMEOUT_MS := 1000
const CORPSE_SECONDS := 3.0
# Collision, as in rose-offline-client's collision_system.rs.
const WALLS := 1  # physics layer bits from zone.rs
const FLOORS := 2
const BODY_RADIUS := 0.4
const BODY_HEIGHT := 1.2  # height of the wall probe
const STEP_HEIGHT := 1.35  # highest step up onto a floor object
const BLOCKED_TIMEOUT_MS := 1500

signal collided(at: Vector3)

var zone: Node
var model: Node3D
var anim: AnimationPlayer
var label: Label3D
var entity_id := -1
var is_monster := false
var is_npc := false  # a town NPC: stands still, facing its direction
var has_store := false
var npc_id := 0
var look := []  # male, face, hair, head, body, hands, feet, weapon, sub weapon (players)
var dead := false
var dying := false  # the server removed it after a killing blow; playing the death
var was_swinging := false
var attack_index := 0
var height := 2.0
var hp := 0
var max_hp := 0
var mp := 0
var max_mp := 0
var level := 0
var predicted := {}  # from, to (Vector3), started (msec)
var is_me := false
var blocked := {}  # at, to (Vector3), since (msec): we hit a wall and wait for the server to stop us
var placed := false
var is_moving := false
var _probe: SphereShape3D
var _idle := "stop1"
var _walk := "run"


func setup(zone_node: Node, state: Dictionary, is_me_: bool) -> void:
	zone = zone_node
	is_me = is_me_
	entity_id = state["id"]
	is_monster = state["kind"] == "monster"
	is_npc = state["kind"] == "npc"
	has_store = state.get("store", false)
	npc_id = state["npc_id"]
	look = state.get("look", [])
	_build()
	label = Label3D.new()
	label.billboard = BaseMaterial3D.BILLBOARD_ENABLED
	label.no_depth_test = true
	label.fixed_size = true
	label.pixel_size = 0.0015
	label.font_size = 22
	label.outline_size = 6
	if is_me_:
		label.modulate = Color(1.0, 0.95, 0.6)
	elif is_monster:
		label.modulate = Color(1.0, 0.75, 0.7)
	elif is_npc:
		label.modulate = Color(0.65, 1.0, 0.65)
	label.position.y = height + 0.3
	add_child(label)
	if is_npc:
		rotation.y = deg_to_rad(state.get("direction", 0.0))
	update_state(state, null)


func _build() -> void:
	if model:
		model.queue_free()
	if is_monster or is_npc:
		model = RoseNpc.new()
		model.build(npc_id)
		_idle = "stop"
		_walk = "move"
	else:
		model = RoseCharacter.new()
		var l: Array = look if look.size() == 9 else [true, 1, 0, 0, 1, 1, 1, 0, 0]
		model.build(l[0], l[1], l[2], l[3], l[4], l[5], l[6], l[7], l[8])
	model.name = "Model"
	add_child(model)
	anim = model.get_node_or_null("AnimationPlayer")
	if anim:
		anim.animation_finished.connect(_on_animation_finished)
	height = _model_height()
	_play(_idle)


## Height of the model's bounds in its rest pose, for the name tag.
func _model_height() -> float:
	var top := 0.0
	for node in model.find_children("*", "MeshInstance3D", true, false):
		var mesh_instance := node as MeshInstance3D
		var box := mesh_instance.get_aabb()
		var to_model := model.global_transform.affine_inverse() * mesh_instance.global_transform
		top = maxf(top, (to_model * box).end.y)
	return clampf(top * model.scale.y, 1.0, 12.0) if top > 0.0 else 2.0


## Starts running toward a Godot position before the server confirms the move.
func predict_move(target: Vector3) -> void:
	if dead:
		return
	predicted = {"from": Vector3(position.x, 0, position.z), "to": Vector3(target.x, 0, target.z), "started": Time.get_ticks_msec()}
	was_swinging = false
	_play(_walk, true)


## Applies one entry from RoseNet.get_entities(). target_position is where the entity it
## attacks stands, or null.
func update_state(state: Dictionary, target_position) -> void:
	var name_text: String = state["name"]
	hp = state.get("hp", 0)
	max_hp = state.get("max_hp", 0)
	mp = state.get("mp", 0)
	max_mp = state.get("max_mp", 0)
	level = state.get("level", 0)
	if label.text != name_text:
		label.text = name_text
	if not is_monster and state.has("look") and state["look"] != look:
		look = state["look"]
		_build()
		label.position.y = height + 0.3

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

	if is_me:
		var stop = null
		if not blocked.is_empty():
			var waited := Time.get_ticks_msec() - int(blocked["since"])
			if moving and server_to.distance_to(blocked["to"]) < 0.05 and waited < BLOCKED_TIMEOUT_MS:
				stop = blocked["at"]  # the server still has us walking on; hold at the wall
			else:
				blocked = {}
		elif moving and placed:
			stop = _wall_hit(Vector3(position.x, 0, position.z), flat)
			if stop != null:
				blocked = {"at": stop, "to": server_to, "since": Time.get_ticks_msec()}
				predicted = {}
				collided.emit(stop)
		if stop != null:
			flat = stop
			moving = false
			server_to = stop

	var heading := server_to - flat if moving else Vector3.ZERO
	if not moving and target_position != null:
		heading = target_position - flat
	if Vector2(heading.x, heading.z).length() > 0.01 and not is_npc:
		rotation.y = atan2(heading.x, heading.z)
	position = Vector3(flat.x, _ground_height(flat), flat.z)
	placed = true

	var is_dead: bool = state.get("dead", false)
	var swinging: bool = state.get("swinging", false) and predicted.is_empty()
	if is_dead:
		if not dead:
			_play("die", true)
	elif moving:
		# Monsters walk when they wander or head home and run when they chase.
		var walk := _walk
		if is_monster and state.get("chasing", false) and anim and anim.has_animation("run"):
			walk = "run"
		_play(walk)  # moving cancels a swing at once
	elif swinging and not was_swinging:
		_swing(state.get("hit_in", 0.0))
	elif not _is_busy():
		_play(_idle)
	dead = is_dead
	was_swinging = swinging
	is_moving = moving


## Where a body walking from one flat position to the next first touches a wall, or null.
## Like the Bevy client, the probe starts one radius ahead, so walking away from a wall
## you stand against is never blocked.
func _wall_hit(from: Vector3, to: Vector3):
	var motion := to - from
	if motion.length() < 0.0001:
		return null
	if _probe == null:
		_probe = SphereShape3D.new()
		_probe.radius = BODY_RADIUS
	var direction := motion.normalized()
	var params := PhysicsShapeQueryParameters3D.new()
	params.shape = _probe
	params.collision_mask = WALLS
	params.transform = Transform3D(Basis(), from + direction * BODY_RADIUS + Vector3(0, position.y + BODY_HEIGHT, 0))
	params.motion = motion
	var result := get_world_3d().direct_space_state.cast_motion(params)
	if result.size() < 2 or result[0] >= 1.0:
		return null
	return from + direction * maxf(motion.length() * result[0] - 0.1, 0.0)


## Terrain height, or the floor object under the feet if that is higher (bridges, stairs).
func _ground_height(flat: Vector3) -> float:
	var terrain: float = zone.get_terrain_height(flat.x, flat.z)
	var top := (position.y if placed else terrain + 50.0) + STEP_HEIGHT
	if top <= terrain:
		return terrain
	var query := PhysicsRayQueryParameters3D.create(Vector3(flat.x, top, flat.z), Vector3(flat.x, terrain, flat.z), FLOORS)
	var hit := get_world_3d().direct_space_state.intersect_ray(query)
	return maxf(terrain, hit["position"].y) if not hit.is_empty() else terrain


## The server removed this entity after a killing blow: play the death, then go.
func die_and_free() -> void:
	dying = true
	label.visible = false
	_play("die", true)
	await get_tree().create_timer(CORPSE_SECONDS).timeout
	queue_free()


## A hit landed on this entity: flinch if it is standing idle.
func on_hit() -> void:
	if not dead and not dying and not _is_busy() and anim and anim.current_animation == _idle:
		_play("hit", true)


func _swing(hit_in: float) -> void:
	var name := "attack"
	if not is_monster:
		var names := ["attack", "attack2", "attack3"]
		name = names[attack_index % names.size()]
		attack_index += 1
	if anim == null or not anim.has_animation(name):
		name = "attack"
	_play(name, true)
	# Play at the speed that puts the hit frame on the server's hit time (attack speed and
	# network delay both change it a little).
	if anim and anim.has_animation(name) and hit_in > 0.05:
		anim.speed_scale = clampf(hit_time(name) / hit_in, 0.7, 1.6)


## Time of the first damage frame in an attack motion, using the same frame event ids
## (10, 20-28, 56-57, 66-67) as the server's attack_hit_ms import.
func hit_time(name: String) -> float:
	var animation := anim.get_animation(name)
	var events: PackedInt32Array = animation.get_meta("frame_events", PackedInt32Array())
	var fps: float = animation.get_meta("fps", 30.0)
	for frame in events.size():
		var e := events[frame]
		if e == 10 or (e >= 20 and e <= 28) or e == 56 or e == 57 or e == 66 or e == 67:
			return frame / fps
	return animation.length / 2.0


func _is_busy() -> bool:
	if anim == null or not anim.is_playing():
		return false
	var current := String(anim.current_animation)
	return current.begins_with("attack") or current == "hit"


func _play(name: String, restart := false) -> void:
	if anim == null or not anim.has_animation(name):
		return
	if anim.current_animation == name and not restart:
		return
	if restart:
		anim.stop()
	anim.speed_scale = 1.0
	anim.play(name, BLEND)


func _on_animation_finished(name: StringName) -> void:
	var finished := String(name)
	if (finished.begins_with("attack") or finished == "hit") and not dead and not dying:
		_play(_idle)
