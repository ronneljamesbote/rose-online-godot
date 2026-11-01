extends "res://demos/demo.gd"

## Walks to the NPC named --npc (part of the name), prints what it says, and answers with
## --answers (1-based, comma separated; "q" prints the quest list, "shot=PATH" saves a
## picture, "say=LINE" says a chat line, "~TEXT" picks the
## answer with that text, "cam=DIST:YAW:PITCH" moves the camera; see demos/README.md for the rest), then prints our quests.
func run() -> void:
	await get_tree().create_timer(3.0).timeout
	if options.has("goto"):
		# Dev server: stand next to that NPC first (the /goto GM command).
		online.chat_window.say("/goto " + String(options["goto"]))
		await get_tree().create_timer(4.0).timeout
	var wanted := String(options.get("npc", "")).to_lower()
	var npc := -1
	var nearest := INF
	for id in online.entities:
		var entity: Node3D = online.entities[id]
		var d: float = entity.position.distance_to(online.me.position)
		if entity.is_npc and wanted in String(entity.label.text).to_lower() and d < nearest:
			npc = id
			nearest = d
	if npc < 0:
		var names := []
		for id in online.entities:
			if online.entities[id].is_npc:
				names.append(online.entities[id].label.text)
		print("rose net demo: no NPC called ", wanted, " here, only ", names)
		return
	print("rose net demo: walk to %s, %.0f m away" % [online.entities[npc].label.text, online.entities[npc].position.distance_to(online.me.position)])
	online.talk_to(npc)
	var waited := 0.0
	while not online.conversation_window.visible and not online.store_window.visible and waited < 60.0:
		await get_tree().create_timer(0.5).timeout
		waited += 0.5
	_print_conversation()
	var answers := String(options.get("answers", ""))
	for answer in answers.split(",", false):
		await get_tree().create_timer(1.5).timeout
		if answer == "q":
			_print_quests()
			continue
		if answer == "deposit":
			# The first item in the bag's consumables page, whole stack.
			var bag: Array = online.net.get_inventory()["pages"][1]
			for i in bag.size():
				if bag[i] != null:
					print("rose net demo: deposit %s x%d" % [bag[i]["name"], bag[i].get("quantity", 1)])
					online.bank_window.deposit(1, i, bag[i].get("quantity", 1))
					break
			await get_tree().create_timer(1.0).timeout
			_print_bank()
			continue
		if answer == "withdraw":
			var bank: Array = online.net.get_bank()
			for i in bank.size():
				if bank[i] != null:
					print("rose net demo: withdraw %s x%d" % [bank[i]["name"], bank[i].get("quantity", 1)])
					online.bank_window.withdraw(i, bank[i].get("quantity", 1))
					break
			await get_tree().create_timer(1.0).timeout
			_print_bank()
			continue
		if answer.begins_with("pick="):
			var found := _find_bag_item(answer.trim_prefix("pick="))
			if found.is_empty():
				print("rose net demo: no ", answer.trim_prefix("pick="), " in the bag")
			else:
				online.work_window.set_target(found[0], found[1])
				await get_tree().create_timer(1.5).timeout
				print("rose net demo: %s: %s | %s" % [online.work_window.title.text, online.work_window.info.text.replace("\n", " / "), online.work_window.cost.text])
			continue
		if answer == "work":
			online.work_window.work()
			await get_tree().create_timer(2.5).timeout
			print("rose net demo: now: %s" % online.work_window.info.text.replace("\n", " / "))
			continue
		if answer.begins_with("skill="):
			# Use our skill with this name (e.g. Item Refining opens the refine window).
			var pages: Array = online.net.get_skills()
			for page in pages.size():
				for index in pages[page].size():
					var skill = pages[page][index]
					if skill != null and answer.trim_prefix("skill=").to_lower() in String(skill["name"]).to_lower():
						online.use_skill(page, index, skill)
			await get_tree().create_timer(1.0).timeout
			continue
		if answer.begins_with("shot="):
			await get_tree().create_timer(0.5).timeout
			main.get_viewport().get_texture().get_image().save_png(answer.trim_prefix("shot="))
			print("rose net demo: saved ", answer.trim_prefix("shot="))
			continue
		if answer.begins_with("cam="):
			# cam=DISTANCE:YAW:PITCH (metres, degrees) for a wider picture; closes the bag.
			var parts := answer.trim_prefix("cam=").split(":")
			main.camera.distance = float(parts[0])
			if parts.size() > 2:
				main.camera.yaw = deg_to_rad(float(parts[1]))
				main.camera.pitch = deg_to_rad(float(parts[2]))
			online.inventory_window.visible = false
			await get_tree().create_timer(1.0).timeout
			continue
		if answer.begins_with("say="):
			# A chat line, e.g. a GM command on a dev server.
			online.chat_window.say(answer.trim_prefix("say="))
			await get_tree().create_timer(2.0).timeout
			continue
		if answer == "revive":
			online.net.revive(false)
			await get_tree().create_timer(2.0).timeout
			continue
		if answer == "wait":
			await get_tree().create_timer(6.0).timeout
			continue
		if answer.begins_with("equip="):
			var found := _find_bag_item(answer.trim_prefix("equip="))
			if not found.is_empty():
				online.net.equip_item(found[0], found[1])
				await get_tree().create_timer(1.0).timeout
			continue
		if answer.begins_with("use="):
			var found := _find_bag_item(answer.trim_prefix("use="))
			if not found.is_empty():
				online.net.use_item(found[0], found[1])
				await get_tree().create_timer(1.5).timeout
			continue
		if answer == "unequip":
			online.net.unequip_item(6)
			await get_tree().create_timer(1.0).timeout
			continue
		if answer.begins_with("gem="):
			var found := _find_bag_item(answer.trim_prefix("gem="))
			if not found.is_empty():
				online.net.insert_gem(found[0], found[1])
				await get_tree().create_timer(1.5).timeout
				var weapon = online.net.get_inventory()["equipped"][6]
				print("rose net demo: weapon now: ", weapon["tooltip"].replace("\n", " / ") if weapon else "none")
			continue
		if answer.begins_with("act=") or answer.begins_with("show="):
			# Right-click (act) or print (show) an inventory slot: e6 = equipped weapon,
			# v3 = vehicle arms, anything else = the first bag item with that name.
			var slot = _inventory_slot(answer.get_slice("=", 1))
			if slot == null:
				print("rose net demo: no slot ", answer)
			elif answer.begins_with("act="):
				online.inventory_window.activate(slot)
				await get_tree().create_timer(1.5).timeout
				print("rose net demo: repair hint: ", online.inventory_window.repair_hint.text if online.inventory_window.repair_hint.visible else "(none)")
			else:
				# Hover it too, so the tooltip shows on screen.
				get_viewport().warp_mouse(slot.get_global_rect().get_center())
				print("rose net demo: %s: %s" % [slot.item.get("name", "?"), String(slot.item.get("tooltip", "")).replace("\n", " / ")] if slot.item else "rose net demo: empty slot")
				await get_tree().create_timer(2.5).timeout
			continue
		if answer == "stats":
			var c: Dictionary = online.net.get_character()
			print("rose net demo: attack %d defence %d" % [c.get("attack", 0), c.get("defence", 0)])
			continue
		if answer == "talk":
			online.talk_to(npc)
			await get_tree().create_timer(1.0).timeout
			_print_conversation()
			continue
		var choice := int(answer) - 1
		if answer.begins_with("~"):
			# The first answer whose text has these words.
			var responses: Array = online.net.get_conversation().get("responses", [])
			choice = responses.find_custom(func(r): return answer.trim_prefix("~").to_lower() in String(r).to_lower())
			if choice < 0:
				print("rose net demo: no answer like ", answer)
				continue
		print("rose net demo: answer ", answer)
		online.conversation_window.choose(choice)
		await get_tree().create_timer(0.5).timeout
		_print_conversation()
	await get_tree().create_timer(2.0).timeout
	_print_quests()


## The inventory window slot for e<N> (equipped), v<N> (vehicle part) or a bag item name.
func _inventory_slot(text: String):
	var inv = online.inventory_window
	inv.visible = true
	inv._refresh(true)
	if text.length() >= 2 and text[0] == "e" and text.substr(1).is_valid_int():
		return inv.equipped_slots[int(text.substr(1))]
	if text.length() >= 2 and text[0] == "v" and text.substr(1).is_valid_int():
		return inv.vehicle_slots[int(text.substr(1))]
	var found := _find_bag_item(text)
	if found.is_empty():
		return null
	inv.tabs.current_tab = found[0]
	inv.page = found[0]
	inv._refresh(true)
	return inv.page_slots[found[1]]


func _print_conversation() -> void:
	var d: Dictionary = online.net.get_conversation()
	if not d.get("open", false):
		var open := " store open" if online.store_window.visible else (" bank open" if online.bank_window.visible else (" %s open" % online.work_window.mode if online.work_window.visible else ""))
		print("rose net demo: (no conversation)", open)
		return
	print("rose net demo: %s says: %s" % [d["title"], d["message"]])
	var responses: Array = d["responses"]
	for i in responses.size():
		print("rose net demo:   %d. %s" % [i + 1, responses[i]])


func _print_bank() -> void:
	var items := []
	for item in online.net.get_bank():
		if item != null:
			items.append("%s x%d" % [item["name"], item.get("quantity", 1)])
	print("rose net demo: bank ", items)


func _print_quests() -> void:
	var quests: Array = online.net.get_quests()
	print("rose net demo: %d quests" % quests.size())
	for q in quests:
		var items: Array = q["items"].map(func(i): return "%s x%d" % [i["name"], i["quantity"]])
		print("rose net demo:   %s (slot %d, id %d) items %s" % [q["name"], q["slot"], q["id"], items])
