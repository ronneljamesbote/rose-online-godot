## Player character: click-to-move on the terrain and a melee swing with animation cancelling.
## Moving or stopping during a swing ends it at once, like the server's combat model.
extends Node3D

const RUN_SPEED := 4.505  # metres per second (run speed 450.5 in ROSE units)
const BLEND := 0.12

var zone: Node
var character: Node3D
var anim: AnimationPlayer
var move_target := Vector3.ZERO
var moving := false
var attacking := false
var attack_index := 0


func setup(zone_node: Node, male: bool, equipment: Dictionary) -> void:
	zone = zone_node
	character = RoseCharacter.new()
	character.name = "Character"
	add_child(character)
	character.build(male, equipment.get("face", 1), equipment.get("hair", 1), equipment.get("head", 0),
		equipment.get("body", 1), equipment.get("hands", 1), equipment.get("feet", 1),
		equipment.get("weapon", 0), equipment.get("sub_weapon", 0))
	anim = character.get_node("AnimationPlayer")
	anim.animation_finished.connect(_on_animation_finished)
	_play("stop1")


func place(pos: Vector3) -> void:
	position = Vector3(pos.x, zone.get_terrain_height(pos.x, pos.z), pos.z)


func face(direction: Vector3) -> void:
	if Vector2(direction.x, direction.z).length() > 0.001:
		rotation.y = atan2(direction.x, direction.z)


func move_to(target: Vector3) -> void:
	move_target = Vector3(target.x, 0.0, target.z)
	moving = true
	attacking = false  # animation cancel: the swing ends now
	_play("run")


func stop() -> void:
	moving = false
	attacking = false
	_play("stop1")


func attack() -> void:
	moving = false
	attacking = true
	var names := ["attack", "attack2", "attack3"]
	var name: String = names[attack_index % names.size()]
	attack_index += 1
	if not anim.has_animation(name):
		name = "attack"
	anim.stop()
	anim.play(name, BLEND)


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


func _play(name: String) -> void:
	if anim and anim.has_animation(name) and anim.current_animation != name:
		anim.play(name, BLEND)


func _on_animation_finished(name: StringName) -> void:
	if String(name).begins_with("attack"):
		attacking = false
		_play("run" if moving else "stop1")


func _process(delta: float) -> void:
	if not moving:
		return
	var flat := Vector3(position.x, 0.0, position.z)
	var to_target := move_target - flat
	var distance := to_target.length()
	if distance < 0.05:
		stop()
		return
	var step := minf(distance, RUN_SPEED * delta)
	var direction := to_target / distance
	face(direction)
	flat += direction * step
	position = Vector3(flat.x, zone.get_terrain_height(flat.x, flat.z), flat.z)
