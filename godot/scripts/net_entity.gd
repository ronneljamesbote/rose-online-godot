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
const SPEECH_SECONDS := 6.0

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
var _cast_started := 0  # server start time of the skill cast we last played
var _cast_next := ""  # action motion to play after the casting motion
var _probe: SphereShape3D
var sitting := false
var _speech: Label3D
var _speech_tween: Tween
var _idle := "stop1"
var _walk := "run"
var target_id := -1  # entity it attacks or casts at
var cast_skill := 0  # skill of the last cast, for skill sounds and effects
var cast_target := -1  # what that cast was aimed at (-1: itself or the ground)
var cast_target_node: Node3D  # that target, kept while it plays its death after the hit
var last_target_node: Node3D  # what it last attacked, kept while it plays its death
var _last_target_id := -1
var _cast_anims: Array[String] = []  # the casting and action motions of that cast
var _event_anim := ""  # animation whose frame events we last sent to fx
var _event_frame := -1  # last frame of it whose events were sent
var _died_sound := false


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
	sitting = false
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
	target_id = state.get("target", -1)
	if target_id >= 0 and (target_id != _last_target_id or last_target_node == null):
		var fx := preload("res://scripts/fx.gd").find(self)
		last_target_node = fx.entity_by_id(target_id) if fx else null
		_last_target_id = target_id
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
	var sit: bool = state.get("sitting", false) and predicted.is_empty()
	var swinging: bool = state.get("swinging", false) and predicted.is_empty()
	var cast_started: int = state.get("cast_started", 0)
	if is_dead:
		if not dead:
			_play("die", true)
			_death_sound()
	elif moving:
		# Monsters walk when they wander or head home and run when they chase.
		var walk := _walk
		if is_monster and state.get("chasing", false) and anim and anim.has_animation("run"):
			walk = "run"
		_play(walk)  # moving cancels a swing at once
		_cast_next = ""
		sitting = false
	elif sit:
		if not sitting:
			sitting = true
			_play("sitting" if anim and anim.has_animation("sitting") else "sit", true)
	elif sitting:
		sitting = false
		_play("standup", true)
	elif cast_started != 0 and cast_started != _cast_started:
		_start_cast(state)
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


## Show what this player said over their head for a few seconds.
func say(text: String) -> void:
	if _speech == null:
		_speech = Label3D.new()
		_speech.billboard = BaseMaterial3D.BILLBOARD_ENABLED
		_speech.no_depth_test = true
		_speech.fixed_size = true
		_speech.pixel_size = 0.0015
		_speech.font_size = 20
		_speech.outline_size = 8
		_speech.outline_modulate = Color(0, 0, 0, 0.85)
		_speech.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
		_speech.width = 360
		_speech.vertical_alignment = VERTICAL_ALIGNMENT_BOTTOM
		add_child(_speech)
	_speech.text = text
	_speech.position.y = label.position.y + 0.7
	_speech.modulate = Color(1, 1, 1, 1)
	_speech.visible = true
	if _speech_tween:
		_speech_tween.kill()
	_speech_tween = _speech.create_tween()
	_speech_tween.tween_property(_speech, "modulate:a", 0.0, 0.6).set_delay(SPEECH_SECONDS)
	_speech_tween.tween_callback(func(): _speech.visible = false)


## The server removed this entity after a killing blow: play the death, then go.
func die_and_free() -> void:
	dying = true
	label.visible = false
	_play("die", true)
	_death_sound()
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


## A skill cast started on the server: play its casting motion so it ends when the skill
## takes effect, then its action motion.
func _start_cast(state: Dictionary) -> void:
	_cast_started = state["cast_started"]
	_cast_next = ""
	# The server forgets the cast once its action is over, but the action motion's frame
	# events (projectiles, hits) still need it, so it is kept until the next cast.
	cast_skill = state.get("cast_skill", 0)
	cast_target = state.get("cast_target", -1)
	var fx := preload("res://scripts/fx.gd").find(self)
	cast_target_node = fx.entity_by_id(cast_target) if fx else null
	_cast_anims.clear()
	if anim == null or not model.has_method("add_motion"):
		return
	var cast_motion: int = state.get("cast_motion", -1)
	var action_motion: int = state.get("action_motion", -1)
	var cast_anim: String = model.add_motion(cast_motion) if cast_motion >= 0 else ""
	var action_anim: String = model.add_motion(action_motion) if action_motion >= 0 else ""
	var effect_in: float = state.get("cast_effect_in", 0.0)
	_cast_anims.assign([cast_anim, action_anim])
	if cast_anim != "" and effect_in > 0.05:
		_play(cast_anim, true)
		anim.speed_scale = clampf(anim.get_animation(cast_anim).length / effect_in, 0.5, 2.0)
		_cast_next = action_anim
	elif action_anim != "":
		_play(action_anim, true)


## The skill whose motion is playing now, or 0.
func active_skill() -> int:
	if anim != null and String(anim.current_animation) in _cast_anims:
		return cast_skill
	return 0


func _death_sound() -> void:
	if (is_monster or is_npc) and not _died_sound:
		_died_sound = true
		var fx := preload("res://scripts/fx.gd").find(self)
		if fx:
			fx.npc_died(self)


## Sends the frame events (footsteps, swings, hits) the animation passed since last frame.
func _process(_delta: float) -> void:
	if anim == null or not anim.is_playing():
		return
	var name := String(anim.current_animation)
	var animation := anim.get_animation(name)
	if animation == null:
		return
	var events: PackedInt32Array = animation.get_meta("frame_events", PackedInt32Array())
	var fps: float = animation.get_meta("fps", 30.0)
	var frame := mini(int(anim.current_animation_position * fps), events.size() - 1)
	var looped := false
	if name != _event_anim:
		_event_anim = name
		_event_frame = -1
	elif frame < _event_frame:
		looped = true
	if frame == _event_frame or events.is_empty():
		return
	var fx := preload("res://scripts/fx.gd").find(self)
	if fx == null:
		_event_frame = frame
		return
	if looped:
		for f in range(_event_frame + 1, events.size()):
			if events[f] != 0:
				fx.frame_event(self, events[f])
		_event_frame = -1
		# Monsters and NPCs sometimes make their idle noise, once per idle loop at most.
		if (is_monster or is_npc) and name == _idle and randf() < 0.2:
			fx.npc_sound(self, "idle")
	for f in range(_event_frame + 1, frame + 1):
		if events[f] != 0:
			fx.frame_event(self, events[f])
	_event_frame = frame


func _is_busy() -> bool:
	if anim == null or not anim.is_playing():
		return false
	var current := String(anim.current_animation)
	return current.begins_with("attack") or current == "hit" or current.begins_with("motion_") or current == "standup"


func _play(name: String, restart := false) -> void:
	if anim == null or not anim.has_animation(name):
		return
	if anim.current_animation == name and not restart:
		return
	if restart:
		anim.stop()
		_event_anim = name
		_event_frame = -1
	anim.speed_scale = 1.0
	anim.play(name, BLEND)


func _on_animation_finished(name: StringName) -> void:
	var finished := String(name)
	if finished == "sitting" and sitting and not dead:
		_play("sit")
		return
	if finished == "standup" and not dead and not dying:
		_play(_idle)
		return
	if finished.begins_with("motion_") and _cast_next != "" and not dead and not dying:
		var next := _cast_next
		_cast_next = ""
		_play(next, true)
		return
	if finished.begins_with("motion_") and not dead and not dying:
		_play(_idle)
		return
	if (finished.begins_with("attack") or finished == "hit") and not dead and not dying:
		_play(_idle)
