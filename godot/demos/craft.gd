extends "res://demos/demo.gd"

## Opens the first craft skill, picks the item matching --craft=NAME and crafts it
## --times=N times (default 3), a few seconds apart.
func run() -> void:
	await get_tree().create_timer(4.0).timeout
	var pages: Array = online.net.get_skills()
	for page in pages.size():
		for index in pages[page].size():
			var skill = pages[page][index]
			if skill == null or skill["type"] != "Create Window":
				continue
			online.use_skill(page, index, skill)
			await get_tree().create_timer(1.0).timeout
			if not online.craft_window.visible:
				continue
			var wanted := String(options.get("craft", "")).to_lower()
			var w = online.craft_window
			print("rose net demo: %s makes %d items" % [skill["name"], w.entries.size()])
			for i in w.entries.size():
				if wanted in String(w.entries[i]["item"]["name"]).to_lower():
					w.list.select(i)
					w._select(i)
					break
			await get_tree().create_timer(1.5).timeout
			var e: Dictionary = w.entries[w.selected]
			print("rose net demo: craft %s from %s" % [e["item"]["name"], e["materials"].map(func(m): return "%s x%d" % [m["name"], m["quantity"]])])
			for n in int(options.get("times", "3")):
				print("rose net demo: materials ", w._slots)
				w.craft()
				await get_tree().create_timer(4.0).timeout
			return
	print("rose net demo: no craft skill")
