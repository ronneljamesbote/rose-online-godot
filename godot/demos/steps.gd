extends "res://demos/demo.gd"

## Runs --steps, separated by ";", for testing: "wait 2", "skill ID TARGET" (TARGET is
## nearest, self, none, a player's name or of:NAME, what that player fights; the default
## keeps the current one), "attack
## nearest", "select NAME", "move DX DZ" (metres), "accept" (a party invitation),
## "waitdead NAME", "status" (prints our status effects and the target's), "revive save|here",
## "invite NAME" (to our party), "friend NAME", "friendaccept", "unfriend NAME",
## "friends [open]" (prints our friends list, opens the window),
## "shop TITLE|PAGE,INDEX,QTY,PRICE|...", "shopsetup", "wantsearch TEXT|QTY|PRICE", "shopopen TITLE", "closeshop", "browse NAME",
## "buy NAME N QTY", "money", "equip PAGE INDEX", "use PAGE INDEX", "unequippart N", "drive",
## "inventory [close] [TAB]", "vehicle" (prints our vehicle parts, fuel and stats), "savepoint" and "me" (prints our zone, level, experience and whether we lie fallen),
## "say TEXT" (a chat line, or a GM command on a dev client), "type TEXT" (leaves it typed
## in the chat box, for screenshots of the GM suggestions), "sheet" (prints job, level,
## points, stats, skill count, hotbar slots and worn weapon), "hotbarskill SLOT ID",
## "hotbaritem SLOT PAGE INDEX" (a bag item on hotbar key SLOT + 1), "key SLOT" (presses
## hotbar key SLOT + 1) and "cooldowns" (prints the cooldown left on each hotbar item).
func run() -> void:
	var steps := String(options.get("steps", ""))
	await get_tree().create_timer(3.0).timeout
	for step in steps.split(";", false):
		var words := step.strip_edges().split(" ", false)
		if words.is_empty() or online.me == null:
			continue
		var arg: String = words[1] if words.size() > 1 else ""
		match words[0]:
			"wait":
				await get_tree().create_timer(float(arg)).timeout
			"skill":
				var wanted := int(arg)
				var found := false
				var pages: Array = online.net.get_skills()
				for page in pages.size():
					for index in pages[page].size():
						var skill = pages[page][index]
						if skill != null and int(skill["id"]) == wanted and not found:
							found = true
							if words.size() > 2:
								_steps_target(words[2])
							print("rose net demo: use %s (%d) on %d" % [skill["name"], wanted, online.my_target])
							online.use_skill(page, index, skill)
				if not found:
					print("rose net demo: no skill ", wanted)
			"attack":
				_steps_target(arg)
				if online.my_target >= 0:
					online.attack(online.my_target)
			"select":
				_steps_target(" ".join(words.slice(1)))  # names may have spaces
			"leaveparty":
				online.net.party_leave()
			"move":
				var dz := float(words[2]) if words.size() > 2 else 0.0
				online.move_to(online.me.position + Vector3(float(arg), 0, dz))
			"accept":
				for i in 20:
					if not online.net.get_party_invites().is_empty():
						online.net.party_answer(online.net.get_party_invites()[0][0], true)
						print("rose net demo: joined the party")
						break
					await get_tree().create_timer(0.5).timeout
			"waitdead":
				for i in 40:
					_steps_target(arg)
					if online.my_target >= 0 and online.entities[online.my_target].dead:
						print("rose net demo: %s is down" % arg)
						break
					await get_tree().create_timer(0.25).timeout
			"friend":
				online.net.friend_ask(arg)
			"friendaccept":
				for i in 40:
					var requests: Array = online.net.get_friend_requests()
					if not requests.is_empty():
						online.net.friend_answer(int(requests[0][0]), true)
						print("rose net demo: accepted %s as a friend" % requests[0][1])
						break
					await get_tree().create_timer(0.25).timeout
			"unfriend":
				online.net.friend_remove(arg)
			"friends":
				if arg == "open":
					online.friends_window.visible = true
				var list: Array = online.net.get_friends()
				print("rose net demo: friends ", list.map(func(f): return "%s %s %s" % [f["name"], "online" if f["online"] else "offline", f["zone"]]))
			"invite":
				_steps_target(arg)
				if online.my_target >= 0:
					online.net.party_invite(online.my_target)
			"shop":
				# shop TITLE|PAGE,INDEX,QTY,PRICE|...: open our shop with these bag slots.
				# A part "want:TYPE,NUMBER,QTY,PRICE" asks to buy instead.
				var parts := step.strip_edges().substr(5).split("|")
				var listings := []
				var wanted := []
				for part in parts.slice(1):
					if part.begins_with("want:"):
						var w := part.trim_prefix("want:").split(",")
						wanted.append([w[0], int(w[1]), int(w[2]), int(w[3])])
						continue
					var n := part.split(",")
					listings.append([int(n[0]), int(n[1]), int(n[2]), int(n[3])])
				online.net.store_open(parts[0], listings, wanted)
			"wantsearch":
				# wantsearch TEXT: type into the buy list search of the setup window and add the first hit.
				# wantsearch TEXT|QTY|PRICE also sets the amount and price.
				var want := step.strip_edges().substr(11).split("|")
				online.shop_window.search_box.text = want[0]
				online.shop_window._search(want[0])
				var hits: Array = online.net.find_items(want[0])
				if not hits.is_empty():
					online.shop_window.add_want(hits[0])
					var row: Array = online.shop_window._want_rows[-1]
					if want.size() > 2:
						row[2].value = int(want[1])
						row[3].value = int(want[2])
			"shopopen":
				# shopopen TITLE: press Open shop in the setup window.
				online.shop_window.title_box.text = step.strip_edges().substr(9)
				online.shop_window._action()
			"sell":
				# sell NAME N QTY: sell QTY to the shop's Nth buy list entry (from 0).
				_steps_target(arg)
				var shop: Dictionary = online.net.get_personal_store(online.my_target)
				if shop.is_empty() or shop.get("wants", []).size() <= int(words[2]):
					print("rose net demo: %s has no such buy list entry" % arg)
				else:
					var w: Dictionary = shop["wants"][int(words[2])]
					online.net.store_sell(online.my_target, int(w["id"]), int(w["page"]), int(w["index"]), int(words[3]))
					print("rose net demo: selling %s x%s for %d each (have %d)" % [w["name"], words[3], w["price"], w["have"]])
			"ride":
				# ride NAME: offer that player a ride.
				_steps_target(arg)
				if online.my_target >= 0:
					online.net.ride_offer_to(online.my_target)
			"rideaccept":
				for i in 60:
					var offers: Array = online.net.get_ride_offers()
					if not offers.is_empty():
						print("rose net demo: ride offers ", offers)
						online.net.ride_answer(int(offers[0][0]), true)
						break
					await get_tree().create_timer(0.25).timeout
			"rideleave":
				online.net.ride_leave()
			"shopsetup":
				online.shop_window.open_setup()
			"closeshop":
				online.net.store_close()
			"browse":
				_steps_target(arg)
				if online.my_target >= 0:
					online.shop_window.browse(online.my_target)
			"buy":
				# buy NAME N QTY: buy QTY of the shop's Nth item (from 0).
				_steps_target(arg)
				var store: Dictionary = online.net.get_personal_store(online.my_target)
				if store.is_empty():
					print("rose net demo: %s has no shop" % arg)
				else:
					var it: Dictionary = store["items"][int(words[2])]
					online.net.store_buy(online.my_target, int(it["id"]), int(words[3]))
					print("rose net demo: buying %s x%s for %d each" % [it["name"], words[3], it["price"]])
			"bag":
				var pages: Array = online.net.get_inventory().get("pages", [])
				for page in pages.size():
					for index in pages[page].size():
						var it = pages[page][index]
						if it != null:
							print("rose net demo: bag %d,%d %s x%d %s" % [page, index, it["name"], it["quantity"], "" if it.get("tradeable", true) else "(no trade)"])
			"money":
				print("rose net demo: money ", online.net.get_inventory().get("money", 0))
			"equip":
				online.net.equip_item(int(arg), int(words[2]))
			"use":
				online.net.use_item(int(arg), int(words[2]))
			"unequippart":
				online.net.unequip_vehicle_part(int(arg))
			"drive":
				online.net.drive_toggle()
			"inventory":
				online.inventory_window.visible = arg != "close"
				if words.size() > 2:
					online.inventory_window.tabs.current_tab = int(words[2])
			"vehicle":
				var c: Dictionary = online.net.get_character()
				var parts: Array = online.net.get_inventory().get("vehicle", [])
				print("rose net demo: vehicle %s driving %s fuel %d speed %.0f attack %d range %.1f" % [
					parts.map(func(p): return p["name"] if p != null else "-"), c.get("driving", false),
					c.get("fuel", -1), c.get("move_speed", 0.0), c.get("attack", 0), c.get("range", 0.0)])
			"revive":
				online.net.revive(arg == "save")
			"savepoint":
				online.net.set_save_point()
			"me":
				var c: Dictionary = online.net.get_character()
				print("rose net demo: me zone %d level %d xp %d/%d debt %d fallen %s hp %d" % [c.get("zone", 0), c.get("level", 0),
					c.get("xp", 0), c.get("xp_needed", 0), c.get("xp_debt", 0), c.get("fallen", false), online.me.hp])
				print("rose net demo: me at %.1f, %.1f driving %s passenger %s stamina %d" % [online.me.position.x, online.me.position.z, c.get("driving", false), c.get("passenger", false), c.get("stamina", 0)])
			"say":
				online.chat_window.input.clear()
				online.chat_window.input.text_changed.emit("")
				online.chat_window.input.release_focus()
				online.chat_window.say(step.strip_edges().substr(4))
			"type":
				var input: LineEdit = online.chat_window.input
				input.grab_focus()
				input.text = step.strip_edges().substr(5)
				input.caret_column = input.text.length()
				input.text_changed.emit(input.text)
			"sheet":
				var c: Dictionary = online.net.get_character()
				var skills := 0
				for page in online.net.get_skills():
					for skill in page:
						if skill != null:
							skills += 1
				var hotbar: Array = online.net.get_hotbar().filter(func(h): return h != null).map(func(h): return "%s %d,%d" % [h["kind"], h["page"], h["index"]])
				var weapon = online.net.get_inventory()["equipped"][6]
				print("rose net demo: sheet job %d level %d stat points %d skill points %d STR %d DEX %d INT %d CON %d CHA %d SEN %d skills %d hotbar %s weapon %s" % [
					c.get("job", 0), c.get("level", 0), c.get("stat_points", 0), c.get("skill_points", 0), c.get("str", 0), c.get("dex", 0),
					c.get("int", 0), c.get("con", 0), c.get("cha", 0), c.get("sen", 0), skills, hotbar, weapon["name"] if weapon != null else "-"])
			"hotbarskill":
				var pages: Array = online.net.get_skills()
				for page in pages.size():
					for index in pages[page].size():
						var skill = pages[page][index]
						if skill != null and int(skill["id"]) == int(words[2]):
							online.net.set_hotbar(int(arg), "skill", page, index)
			"hotbaritem":
				online.net.set_hotbar(int(arg), "item", int(words[2]), int(words[3]))
			"key":
				online.hotbar.use_slot(int(arg))
			"cooldowns":
				var now: float = online.net.server_time_ms()
				var left := func(e): return "-" if e == null else "%s %.1f s" % [e["name"], maxf(0.0, (float(e.get("cooldown_end_ms", 0.0)) - now) / 1000.0)]
				print("rose net demo: hotbar cooldowns ", online.net.get_hotbar().map(left))
			"status":
				var mine: Array = online.net.get_status_effects(online.my_id)
				var theirs: Array = online.net.get_status_effects(online.my_target) if online.my_target >= 0 else []
				var names := func(e): return "%s %.0f s" % [e["name"], e["seconds"]]
				print("rose net demo: my effects ", mine.map(names), ", target's ", theirs.map(names))


func _steps_target(which: String) -> void:
	match which:
		"nearest":
			online.my_target = online.nearest_monster()
		"self":
			online.my_target = online.my_id
		"none":
			online.my_target = -1
		_ when which.begins_with("of:"):
			# What that player is fighting.
			for id in online.entities:
				if online.entities[id].label.text == which.substr(3):
					online.my_target = online.entities[id].target_id
					if online.my_target < 0:
						print("rose net demo: %s isn't fighting anything" % which.substr(3))
		_:
			for id in online.entities:
				if online.entities[id].label.text == which:
					online.my_target = id
