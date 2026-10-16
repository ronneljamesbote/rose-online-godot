extends "res://demos/demo.gd"

func run() -> void:
	await get_tree().create_timer(float(options.get("chat-wait", "4"))).timeout
	var sit_for := float(options.get("sit-for", "0"))
	if sit_for > 0:
		print("rose net demo: sit down, HP %d/%d MP %d/%d" % [online.me.hp, online.me.max_hp, online.me.mp, online.me.max_mp])
		online.sit()
	for line in String(options.get("say", "")).split("|", false):
		print("rose net demo: say ", line)
		online.chat_window.say(line)
		await get_tree().create_timer(float(options.get("say-every", "3"))).timeout
	if sit_for > 0:
		var waited := 0.0
		while waited < sit_for:
			await get_tree().create_timer(4.0).timeout
			waited += 4.0
			print("rose net demo: sitting %s, HP %d/%d MP %d/%d" % [online.me.sitting, online.me.hp, online.me.max_hp, online.me.mp, online.me.max_mp])
		print("rose net demo: stand up")
		online.sit()
