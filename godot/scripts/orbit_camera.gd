## ROSE-style orbit camera: right-drag to rotate, wheel to zoom. Zone objects in the way
## pull it closer, like the Bevy client's orbit_camera_system.rs. The terrain never lets it
## below the ground: a slope behind the character first pulls it in, then tilts it up to
## look down over the slope.
extends Camera3D

## How far the camera keeps above the terrain, so the near plane never dips below it.
const TERRAIN_CLEARANCE := 0.5
## Furthest the wheel zooms out, in metres (3/4 of the old 60 m).
const MAX_DISTANCE := 45.0
## A slope that pulls the camera closer than this tilts it up instead.
const MIN_FREE_DISTANCE := 4.0
const MIN_PITCH := deg_to_rad(-85.0)
const MAX_PITCH := deg_to_rad(10.0)
const TILT_STEP := deg_to_rad(2.5)

var target: Node3D
var zone: Node  # the RoseZone, for its terrain height
var yaw := deg_to_rad(-45.0)
var pitch := deg_to_rad(-25.0)  # what the player chose; the slope may tilt the view higher
var distance := float(OS.get_environment("ROSE_CAMERA_DISTANCE")) if OS.get_environment("ROSE_CAMERA_DISTANCE") != "" else 12.0
var look_height := 1.4
var _shown_pitch := pitch
var _tilted := false  # the slope held the view above the chosen angle last frame


func _unhandled_input(event: InputEvent) -> void:
	if event is InputEventMouseMotion and (event.button_mask & MOUSE_BUTTON_MASK_RIGHT):
		yaw -= event.relative.x * 0.01
		pitch = clampf(pitch - event.relative.y * 0.01, MIN_PITCH, MAX_PITCH)
	elif event is InputEventMouseButton and event.pressed:
		if event.button_index == MOUSE_BUTTON_WHEEL_UP:
			distance = maxf(2.0, distance * 0.9)
		elif event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
			distance = minf(MAX_DISTANCE, distance * 1.1)


func _process(delta: float) -> void:
	if target == null:
		return
	var focus := target.global_position + Vector3(0.0, look_height, 0.0)
	# The slope tilts the camera up at once; it settles back to the chosen angle smoothly,
	# never lower than the slope allows.
	var feet := target.global_position + Vector3(0.0, 0.3, 0.0)
	var wanted := minf(distance, MIN_FREE_DISTANCE)
	var tilt := pitch
	var reach := _reach(focus, tilt)
	while not _good_view(focus, feet, tilt, reach, wanted) and tilt > MIN_PITCH:
		tilt = maxf(tilt - TILT_STEP, MIN_PITCH)
		reach = _reach(focus, tilt)
	if not _good_view(focus, feet, tilt, reach, wanted):
		tilt = pitch  # nothing better (a ceiling): stay at the chosen angle, pulled in
		reach = _reach(focus, tilt)
	if _tilted and tilt > _shown_pitch:
		var settled := move_toward(_shown_pitch, tilt, delta * 2.0)
		var settled_reach := _reach(focus, settled)
		if _good_view(focus, feet, settled, settled_reach, minf(reach, wanted)):
			tilt = settled
			reach = settled_reach
	_shown_pitch = tilt
	_tilted = tilt < pitch
	global_position = focus + _direction(tilt) * reach
	look_at(focus)


## The camera is far enough out and the terrain does not hide the character's feet (in a
## ditch or behind a ridge).
func _good_view(focus: Vector3, feet: Vector3, at_pitch: float, reach: float, wanted: float) -> bool:
	if reach < wanted:
		return false
	if zone == null or not is_instance_valid(zone):
		return true
	var from := focus + _direction(at_pitch) * reach
	var length := from.distance_to(feet)
	var t := 0.5
	while t < length - 0.5:
		var p := from.lerp(feet, t / length)
		if p.y < zone.get_terrain_height(p.x, p.z):
			return false
		t += 0.5
	return true


func _direction(at_pitch: float) -> Vector3:
	return Basis.from_euler(Vector3(at_pitch, yaw, 0.0)) * Vector3(0.0, 0.0, 1.0)


## How far from the focus the camera can go at this pitch: up to the zoom distance, short
## of walls and floors (physics ray) and of the terrain.
func _reach(focus: Vector3, at_pitch: float) -> float:
	var direction := _direction(at_pitch)
	var length := distance
	var query := PhysicsRayQueryParameters3D.create(focus, focus + direction * length, 1 | 2)
	var hit := get_world_3d().direct_space_state.intersect_ray(query)
	if not hit.is_empty():
		length = maxf(focus.distance_to(hit["position"]) - 0.3, 0.5)
	return _terrain_free_distance(focus, direction, length)


## How far from the focus the camera can go along direction (at most length) before it
## would sink into the terrain. The terrain is a height field with no physics body in zones
## loaded from the ROSE map files, so the ray above misses it there; this walks out from
## the focus instead. Godot zone scenes fill the same terrain heights from Terrain3D.
func _terrain_free_distance(focus: Vector3, direction: Vector3, length: float) -> float:
	if zone == null or not is_instance_valid(zone):
		return length
	var t := 0.0
	while t < length:
		var step := minf(0.5, length - t)
		var p := focus + direction * (t + step)
		if p.y < zone.get_terrain_height(p.x, p.z) + TERRAIN_CLEARANCE:
			return t
		t += step
	return length
