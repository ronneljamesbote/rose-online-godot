extends "res://demos/demo.gd"

## PvP test: attack the player --fight=NAME (in a PvP zone), printing both HPs until one
## falls. --leave-party leaves our party a few seconds in (party members can't fight in
## "all except party" zones).
func run() -> void:
	var wanted := String(options.get("fight", ""))
	await get_tree().create_timer(4.0).timeout
	print("rose net demo: zone PvP state ", online.net.zone_pvp())
	var target := -1
	var waited := 0.0
	while waited < 60.0:
		for id in online.entities:
			if online.entities[id].label.text == wanted:
				target = id
		if target >= 0:
			break
		await get_tree().create_timer(1.0).timeout
		waited += 1.0
	if target < 0:
		print("rose net demo: no ", wanted)
		return
	print("rose net demo: %s is an enemy: %s" % [wanted, online.net.is_enemy_player(target)])
	online.attack(target)
	await get_tree().create_timer(2.0).timeout
	if options.has("leave-party") and not online.net.get_party().is_empty():
		print("rose net demo: leave the party")
		online.net.party_leave()
		await get_tree().create_timer(2.0).timeout
		print("rose net demo: %s is an enemy: %s" % [wanted, online.net.is_enemy_player(target)])
		online.attack(target)
	for i in 20:
		await get_tree().create_timer(1.5).timeout
		if online.my_target != target and online.net.is_enemy_player(target):
			online.attack(target)
		var them: Node3D = online.entities.get(target)
		print("rose net demo: me HP %d/%d, %s HP %s" % [online.me.hp, online.me.max_hp, wanted, "%d/%d" % [them.hp, them.max_hp] if them else "?"])
		if them == null or them.dead or online.me.dead:
			break
