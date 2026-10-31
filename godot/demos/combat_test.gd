extends "res://demos/demo.gd"

## Combat balance test (dev server): says the GM lines of --say, equips --equip, prints the
## character values, then fights --kills monsters with /heal before each, every hit logged.
func run() -> void:
	online.log_damage = true
	await get_tree().create_timer(2.0).timeout
	for line in String(options.get("say", "")).split("|", false):
		print("rose net demo: say ", line)
		online.chat_window.say(line)
		await get_tree().create_timer(2.0).timeout
	var want := String(options.get("equip", ""))
	if want != "":
		var at := _find_bag_item(want)
		if not at.is_empty():
			online.net.equip_item(at[0], at[1])
			print("rose net demo: equip ", want)
		await get_tree().create_timer(2.0).timeout
	var c: Dictionary = online.net.get_character()
	print("rose net demo: stats level %s STR %s DEX %s CON %s SEN %s attack %s defence %s hit %s avoid %s critical %s (%.1f%%) crit damage %s%% speed %s (+%s%%)" % [
		c.get("level"), c.get("str"), c.get("dex"), c.get("con"), c.get("sen"), c.get("attack"), c.get("defence"),
		c.get("hit"), c.get("avoid"), c.get("critical"), c.get("critical_chance", -1.0), c.get("critical_damage", -1), c.get("attack_speed"), c.get("attack_speed_bonus", -1)])
	var kills := int(options.get("kills", "5"))
	for k in kills:
		online.chat_window.say("/heal")
		await get_tree().create_timer(1.0).timeout
		var target: int = online.nearest_monster()
		var waited := 0.0
		while target < 0 and waited < 10.0:
			await get_tree().create_timer(0.5).timeout
			waited += 0.5
			target = online.nearest_monster()
		if target < 0:
			break
		print("rose net demo: fight ", k + 1, " ", online.entities[target].label.text, " (entity ", target, ")")
		online.attack(target)
		var t := 0.0
		while online.entities.has(target) and not online.entities[target].dying and online.me and not online.me.dead and t < 120.0:
			await get_tree().create_timer(0.25).timeout
			t += 0.25
		print("rose net demo: fight ", k + 1, " over after %.1f s, dead %s" % [t, online.me.dead if online.me else true])
	print("rose net demo: combat test done")
