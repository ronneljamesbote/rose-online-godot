extends "res://demos/demo.gd"

## Puts Mana Bolt on hotbar slot 1, equips a rod or staff, walks up to the nearest monster
## and presses key 1 --presses times (default 6) in fast succession, then prints every cast
## that starts. One cast should start, however often the key is pressed.
func run() -> void:
	await get_tree().create_timer(3.0).timeout
	var staff := _find_bag_item("rod")
	if staff.is_empty():
		staff = _find_bag_item("staff")
	if not staff.is_empty():
		online.net.equip_item(staff[0], staff[1])
	var active: Array = online.net.get_skills()[1]
	for i in active.size():
		if active[i] != null and active[i]["name"] == "Mana Bolt":
			online.net.set_hotbar(0, "skill", 1, i)
	await get_tree().create_timer(1.0).timeout
	var target: int = online.nearest_monster()
	if online.entities.has(target):
		var there: Vector3 = online.entities[target].position
		online.move_to(there + (online.me.position - there).normalized() * 6.0)
		await get_tree().create_timer(4.0).timeout
	online.my_target = online.nearest_monster()
	print("rose net demo: target ", online.my_target)
	_last = online.me._cast_started
	_watching = true
	for i in int(options.get("presses", "6")):
		for pressed in [true, false]:
			var key := InputEventKey.new()
			key.keycode = KEY_1
			key.pressed = pressed
			Input.parse_input_event(key)
		await get_tree().create_timer(0.12).timeout
	await get_tree().create_timer(6.0).timeout
	print("rose net demo: %d key presses, %d casts started" % [int(options.get("presses", "6")), _casts])


var _watching := false
var _casts := 0
var _last := 0


func _process(_delta: float) -> void:
	if not _watching:
		return
	var started: int = online.me._cast_started
	if started != _last and started != 0:
		_last = started
		_casts += 1
		print("rose net demo: cast started, MP %d" % online.me.mp)
