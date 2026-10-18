## Renders our own character's head for the player frame portrait. The camera follows the
## head bone, so the head stays in the middle of the circle. Live: the idle motion plays and
## the picture is redrawn every frame. Static: one still picture, taken again when the look
## (face, hair, gear) changes, and nothing is drawn in between.
extends SubViewport

const HEAD_ABOVE_JOINT := 0.21  # metres from the head bone up to the middle of the head
const DISTANCE := 1.9
const FOV := 22.0

var live := true
var _model: Node3D
var _anim: AnimationPlayer
var _skeleton: Skeleton3D
var _head := -1
var _camera: Camera3D
var _look := []
var _settle := 0  # frames to wait for the pose before the static picture is taken


func _init() -> void:
	size = Vector2i(160, 160)
	own_world_3d = true
	transparent_bg = true
	msaa_3d = Viewport.MSAA_4X
	render_target_update_mode = SubViewport.UPDATE_DISABLED


func _ready() -> void:
	var environment := Environment.new()
	environment.background_mode = Environment.BG_CLEAR_COLOR
	environment.ambient_light_source = Environment.AMBIENT_SOURCE_COLOR
	environment.ambient_light_color = Color(0.7, 0.7, 0.75)
	var world_environment := WorldEnvironment.new()
	world_environment.environment = environment
	add_child(world_environment)
	var light := DirectionalLight3D.new()
	light.rotation_degrees = Vector3(-35, 30, 0)
	add_child(light)
	_camera = Camera3D.new()
	_camera.fov = FOV
	add_child(_camera)


## look: male, face, hair, head, body, hands, feet, weapon, sub weapon, face item, back (as on
## the server).
## Returns true when there is a picture to show.
func show_look(look: Array) -> bool:
	if look.size() != 11:
		return _model != null
	if look == _look:
		return true
	_look = look.duplicate()
	if _model:
		_model.queue_free()
	var character := RoseCharacter.new()
	character.build(look[0], look[1], look[2], look[3], look[4], look[5], look[6], look[7], look[8], look[9], look[10])
	add_child(character)
	_model = character
	_anim = character.get_node_or_null("AnimationPlayer")
	_skeleton = character.get_node_or_null("Skeleton3D")
	# The head is the highest body bone of the rest pose (dummy bones like the hat point are
	# named d0, d1 ...).
	_head = -1
	var top := -INF
	if _skeleton:
		for i in _skeleton.get_bone_count():
			var y := _skeleton.get_bone_global_rest(i).origin.y
			if _skeleton.get_bone_name(i).begins_with("b") and y > top:
				top = y
				_head = i
	set_live(live)
	return true


func set_live(on: bool) -> void:
	live = on
	if _anim and _anim.has_animation("stop1"):
		_anim.play("stop1")
		if not live:
			_anim.seek(0.0, true)
			_anim.pause()
	_settle = 2
	render_target_update_mode = SubViewport.UPDATE_ALWAYS if live else SubViewport.UPDATE_DISABLED


func _process(_delta: float) -> void:
	if _model == null or (not live and _settle == 0):
		return
	_frame_head()
	if not live:
		_settle -= 1
		if _settle == 0:
			render_target_update_mode = SubViewport.UPDATE_ONCE


func _frame_head() -> void:
	var head := Vector3(0, 1.36, 0)
	if _skeleton and _head >= 0:
		head = _skeleton.global_transform * _skeleton.get_bone_global_pose(_head).origin
	var target := head + Vector3(0, HEAD_ABOVE_JOINT, 0)
	_camera.position = target + Vector3(0, 0.02, DISTANCE)
	_camera.look_at(target)
