extends "res://demos/demo.gd"

## Sticky target: shows Options > Gameplay with Sticky target set to --sticky=on|off
## (default on), targets the nearest monster, then walks the way a player does with the
## mouse: a click on the ground, then a hold that steers in an arc, then lets go. Prints
## twice a second whether the monster is still the target and whether the click marker is
## up, so both modes can be compared.
func run() -> void:
	var sticky: bool = options.get("sticky", "on") != "off"
	await get_tree().create_timer(2.0).timeout
	_show_options(sticky)
	await get_tree().create_timer(3.0).timeout
	online.toggle_options_window()
	var target: int = online.nearest_monster()
	if not online.entities.has(target):
		print("rose net demo: no monster nearby")
		return
	online.select(target)
	print("rose net demo: sticky %s, target %s (entity %d)" % ["on" if sticky else "off", _name(target), target])
	_watching = true
	await get_tree().create_timer(2.0).timeout
	var centre := get_viewport().get_visible_rect().size / 2.0
	print("rose net demo: click the ground")
	await _press(centre + Vector2(-220, 90))
	await _release()
	await get_tree().create_timer(3.0).timeout
	print("rose net demo: hold the left button and steer")
	await _press(centre + Vector2(220, 60))
	for i in 24:
		var a := float(i) / 24.0 * PI
		_move_mouse(centre + Vector2(cos(a) * 220.0, 60.0 - sin(a) * 140.0))
		await get_tree().create_timer(0.15).timeout
	await _release()
	print("rose net demo: let go")
	await get_tree().create_timer(4.0).timeout
	print("rose net demo: done, target %s" % _name(online.my_target))
	_watching = false


## Opens Options on its Gameplay tab and ticks Sticky target, or clears it.
func _show_options(sticky: bool) -> void:
	online.toggle_options_window()
	for bar in online.options_window.find_children("*", "TabBar", true, false):
		for i in bar.tab_count:
			if bar.get_tab_title(i) == "Gameplay":
				bar.current_tab = i
	for check in online.options_window.find_children("*", "CheckBox", true, false):
		if check.text == "Sticky target":
			check.button_pressed = sticky
	UI.set_setting("sticky_target", sticky)


func _press(at: Vector2) -> void:
	_move_mouse(at)
	await get_tree().process_frame
	_button(at, true)


func _release() -> void:
	_button(get_viewport().get_mouse_position(), false)
	await get_tree().process_frame


func _button(at: Vector2, pressed: bool) -> void:
	var click := InputEventMouseButton.new()
	click.button_index = MOUSE_BUTTON_LEFT
	click.button_mask = MOUSE_BUTTON_MASK_LEFT if pressed else 0
	click.pressed = pressed
	click.position = at
	click.global_position = at
	Input.parse_input_event(click)


func _move_mouse(at: Vector2) -> void:
	get_viewport().warp_mouse(at)
	var motion := InputEventMouseMotion.new()
	motion.position = at
	motion.global_position = at
	motion.button_mask = Input.get_mouse_button_mask()
	Input.parse_input_event(motion)


func _name(id: int) -> String:
	return online.entities[id].label.text if online.entities.has(id) else "none"


var _watching := false
var _next_ms := 0


func _process(_delta: float) -> void:
	if not _watching or Time.get_ticks_msec() < _next_ms:
		return
	_next_ms = Time.get_ticks_msec() + 500
	var marker: bool = online._click_marker != null and is_instance_valid(online._click_marker)
	print("rose net demo: target %s, marker %s, %s" % [_name(online.my_target), "up" if marker else "gone",
		"moving" if online.me.is_moving else "standing"])
