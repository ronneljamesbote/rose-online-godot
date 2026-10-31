extends "res://demos/sticky.gd"

## Click or double-click to attack: shows Options > Gameplay with --attack=single|double
## (default double), then clicks the nearest monster once with the mouse, then double-clicks
## it, then walks away with a ground click and a hold that steers (hold-to-move). Prints
## twice a second the selected target, whether the server has us attacking, and whether
## we move, so both modes can be compared.
func run() -> void:
	var mode: String = "single" if options.get("attack", "double") == "single" else "double"
	await get_tree().create_timer(2.0).timeout
	_show_attack_option(mode)
	await get_tree().create_timer(3.0).timeout
	online.toggle_options_window()
	if online.net.has_method("gm_command"):
		# On a dev server, spawn a few Mini-Jelly Beans beside us to click on.
		online.net.gm_command("/spawn 1 3", -1)
		await get_tree().create_timer(3.0).timeout
	var target: int = online.nearest_monster()
	if not online.entities.has(target):
		print("rose net demo: no monster nearby")
		return
	print("rose net demo: %s-click to attack, monster %s (entity %d)" % [mode, _name(target), target])
	_watching = true
	await get_tree().create_timer(1.0).timeout
	print("rose net demo: one click on the monster")
	await _click_entity(target, false)
	await get_tree().create_timer(3.0).timeout
	if online.entities.has(target):
		print("rose net demo: double-click the monster")
		await _click_entity(target, false)
		await _click_entity(target, true)
		await get_tree().create_timer(4.0).timeout
	var centre := get_viewport().get_visible_rect().size / 2.0
	print("rose net demo: click the ground and hold to steer")
	await _press(centre + Vector2(-220, 90))
	for i in 16:
		var a := float(i) / 16.0 * PI
		_move_mouse(centre + Vector2(-cos(a) * 220.0, 90.0 - sin(a) * 140.0))
		await get_tree().create_timer(0.15).timeout
	await _release()
	print("rose net demo: let go")
	await get_tree().create_timer(3.0).timeout
	print("rose net demo: done, target %s" % _name(online.my_target))
	_watching = false


## Opens Options on its Gameplay tab with the attack choice set.
func _show_attack_option(mode: String) -> void:
	online.toggle_options_window()
	for bar in online.options_window.find_children("*", "TabBar", true, false):
		for i in bar.tab_count:
			if bar.get_tab_title(i) == "Gameplay":
				bar.current_tab = i
	for button in online.options_window.find_children("*", "OptionButton", true, false):
		if button.get_item_count() > 0 and button.get_item_text(0) == "Click to attack":
			button.selected = 1 if mode == "double" else 0
	UI.set_setting("attack_click", mode)


## A left click (press and release) on the entity's body, as a double click if asked.
func _click_entity(id: int, double: bool) -> void:
	var at: Vector2 = main.camera.unproject_position(online.entities[id].global_position + Vector3(0, 0.8, 0))
	_move_mouse(at)
	await get_tree().process_frame
	var click := InputEventMouseButton.new()
	click.button_index = MOUSE_BUTTON_LEFT
	click.button_mask = MOUSE_BUTTON_MASK_LEFT
	click.pressed = true
	click.double_click = double
	click.position = at
	click.global_position = at
	Input.parse_input_event(click)
	await get_tree().process_frame
	await _release()


func _process(_delta: float) -> void:
	if not _watching or Time.get_ticks_msec() < _next_ms:
		return
	_next_ms = Time.get_ticks_msec() + 500
	var attacking := -1
	for state in online.net.get_entities():
		if state["id"] == online.my_id:
			attacking = state.get("target", -1)
	print("rose net demo: target %s, attacking %s, %s" % [_name(online.my_target), _name(attacking),
		"moving" if online.me.is_moving else "standing"])
