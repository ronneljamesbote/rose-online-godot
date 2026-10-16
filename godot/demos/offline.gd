extends "res://demos/demo.gd"

## Run, swing, cancel the swing by moving, swing again and let it finish.
func run() -> void:
	var tree := get_tree()
	await tree.create_timer(1.0).timeout
	for name in ["attack", "attack2", "attack3"]:
		_say("%s: %d ms, hit at %d ms" % [name, main.player.anim.get_animation(name).length * 1000.0, main.player.hit_time(name) * 1000.0])
	_say("run")
	main.player.move_to(main.START + Vector3(8.0, 0.0, -6.0))
	await tree.create_timer(2.5).timeout
	_say("attack (full swing)")
	main.player.attack()
	await tree.create_timer(1.6).timeout
	_say("attack, cancelled by moving")
	main.player.attack()
	await tree.create_timer(0.35).timeout
	main.player.move_to(main.START + Vector3(2.0, 0.0, -10.0))
	await tree.create_timer(2.0).timeout
	_say("attack combo")
	main.player.attack()
	await tree.create_timer(1.3).timeout
	main.player.attack()
	await tree.create_timer(1.3).timeout
	main.player.attack()
	await tree.create_timer(1.6).timeout
	_say("done")
	tree.quit()


func _say(text: String) -> void:
	print("rose demo: ", text)
