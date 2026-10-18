extends "res://demos/demo.gd"

## --net-demo=glow: refine glows and gem effects. Needs a dev server (GM commands). Says
## each GM line of --say=LINE|LINE (for example /job, /level, /item), wears every bag item
## whose name has a text of --wear=TEXT,TEXT, then for each grade of --grades=0,3,6,9 sets it
## on every worn item with /grade all (the weapon also gets the gem --gem=NUMBER, when given)
## and turns the camera round the character. --shots=DIR saves a picture at each grade and
## turn (grade-G-T.png).


func run() -> void:
	var tree := get_tree()
	await tree.create_timer(float(options.get("chat-wait", "4"))).timeout
	for line in String(options.get("say", "")).split("|", false):
		await _gm(line)
	for text in String(options.get("wear", "")).split(",", false):
		for i in 8:  # every bag item with that text
			var found := _find_bag_item(text)
			if found.is_empty():
				break
			print("rose net demo: wear ", online.net.get_inventory()["pages"][found[0]][found[1]]["name"])
			online.net.equip_item(found[0], found[1])
			await tree.create_timer(1.0).timeout
	var camera = main.camera
	camera.distance = float(options.get("camera-distance", "4.5"))
	camera.look_height = 1.0
	camera.pitch = deg_to_rad(-12.0)
	var shots: String = options.get("shots", "")
	var turns := int(options.get("turns", "3"))
	for grade_text in String(options.get("grades", "0,3,6,9")).split(",", false):
		var grade := int(grade_text)
		await _gm("/grade all %d" % grade, 0.5)
		if options.has("gem"):
			await _gm("/grade weapon %d %s" % [grade, str(options["gem"]) if grade > 0 else "0"], 0.5)
		await tree.create_timer(2.0).timeout
		for turn in turns:
			camera.yaw = deg_to_rad(-45.0) + TAU * turn / turns
			await tree.create_timer(1.2).timeout
			if shots != "":
				var path := "%s/grade-%d-%d.png" % [shots, grade, turn]
				main.get_viewport().get_texture().get_image().save_png(path)
				print("rose net demo: saved ", path)
	print("rose net demo: glow done")


func _gm(line: String, wait := 1.5) -> void:
	print("rose net demo: say ", line)
	online.chat_window.say(line)
	await get_tree().create_timer(wait).timeout
