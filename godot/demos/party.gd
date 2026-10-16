extends "res://demos/fight.gd"

## Party test with two clients: with --invite=NAME, invite that player once they are near;
## without, accept the first invitation. Then both fight, printing the party now and then.
func run() -> void:
	var wanted := String(options.get("invite", ""))
	if not online.net.get_party().is_empty():
		# Start from scratch: leave the party of an earlier run.
		print("rose net demo: leave the old party")
		online.net.party_leave()
		await get_tree().create_timer(3.0).timeout
	var waited := 0.0
	while waited < 90.0 and not _party_has(wanted):
		await get_tree().create_timer(1.0).timeout
		waited += 1.0
		if wanted != "":
			for id in online.entities:
				if online.entities[id].label.text == wanted:
					print("rose net demo: invite ", wanted)
					online.net.party_invite(id)
					await get_tree().create_timer(4.0).timeout
					break
		elif not online.net.get_party_invites().is_empty():
			# --accept-after=SECONDS leaves the invitation up for a screenshot.
			await get_tree().create_timer(float(options.get("accept-after", "0"))).timeout
			print("rose net demo: accept invitation from ", online.net.get_party_invites()[0][1])
			online.party_window.answer_invite(true)
	_print_party()
	if options.has("rules"):
		var r := String(options["rules"]).split(",")
		online.net.party_set_rules(int(r[0]), int(r[1]))
	get_tree().create_timer(20.0).timeout.connect(_print_party)
	get_tree().create_timer(50.0).timeout.connect(_print_party)
	super()


## In a party, with this member when a name is given.
func _party_has(member: String) -> bool:
	var party: Dictionary = online.net.get_party()
	if party.is_empty():
		return false
	if member == "":
		return true
	return party["members"].any(func(m): return m["name"] == member)


func _print_party() -> void:
	var party: Dictionary = online.net.get_party()
	if party.is_empty():
		print("rose net demo: no party")
		return
	var members := []
	for m in party["members"]:
		members.append("%s%s Lv %d HP %d/%d" % ["*" if m["leader"] else "", m["name"], m["level"], m["hp"], m["max_hp"]])
	print("rose net demo: party xp %d items %d: %s" % [party["xp_sharing"], party["item_sharing"], members])
