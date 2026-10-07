## ROSE-style orbit camera: right-drag to rotate, wheel to zoom.
extends Camera3D

var target: Node3D
var yaw := deg_to_rad(-45.0)
var pitch := deg_to_rad(-25.0)
var distance := float(OS.get_environment("ROSE_CAMERA_DISTANCE")) if OS.get_environment("ROSE_CAMERA_DISTANCE") != "" else 12.0
var look_height := 1.4


func _unhandled_input(event: InputEvent) -> void:
	if event is InputEventMouseMotion and (event.button_mask & MOUSE_BUTTON_MASK_RIGHT):
		yaw -= event.relative.x * 0.01
		pitch = clampf(pitch - event.relative.y * 0.01, deg_to_rad(-85.0), deg_to_rad(10.0))
	elif event is InputEventMouseButton and event.pressed:
		if event.button_index == MOUSE_BUTTON_WHEEL_UP:
			distance = maxf(2.0, distance * 0.9)
		elif event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
			distance = minf(60.0, distance * 1.1)


func _process(_delta: float) -> void:
	if target == null:
		return
	var focus := target.global_position + Vector3(0.0, look_height, 0.0)
	var offset := Basis.from_euler(Vector3(pitch, yaw, 0.0)) * Vector3(0.0, 0.0, distance)
	global_position = focus + offset
	look_at(focus)
