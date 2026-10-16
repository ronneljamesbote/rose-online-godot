extends "res://demos/demo.gd"

## Walks to the nearest store, buys the first item of each tab, then sells one of them back.
func run() -> void:
	await get_tree().create_timer(3.0).timeout
	var npc: int = online.nearest_store()
	if npc < 0:
		print("rose net demo: no store in this zone")
		return
	print("rose net demo: walk to %s, %.0f m away" % [online.entities[npc].label.text, online.entities[npc].position.distance_to(online.me.position)])
	online.talk_to(npc)
	var waited := 0.0
	while not online.store_window.visible and waited < 60.0:
		await get_tree().create_timer(0.5).timeout
		waited += 0.5
		# The NPC talks first: pick the answer that opens its store.
		if online.conversation_window.visible:
			var responses: Array = online.net.get_conversation().get("responses", [])
			for i in responses.size():
				var text := String(responses[i]).to_lower()
				if "trade" in text or "store" in text or "shop" in text or "buy" in text:
					print("rose net demo: answer ", responses[i])
					online.conversation_window.choose(i)
					break
	if not online.store_window.visible:
		print("rose net demo: the store did not open")
		return
	var store: Dictionary = online.store_window.store
	for t in store["tabs"]:
		var names := []
		for entry in t["items"]:
			names.append("%s %d" % [entry["item"]["name"], entry["price"]])
		print("rose net demo: %s, tab %s: %s" % [store["name"], t["name"], ", ".join(names)])
	var money_before: int = online.net.get_inventory()["money"]
	var wanted: String = options.get("buy", "")
	for t in store["tabs"].size():
		online.store_window.tabs.current_tab = t
		await get_tree().create_timer(1.0).timeout
		for entry in store["tabs"][t]["items"]:
			if wanted != "" and not wanted.to_lower() in String(entry["item"]["name"]).to_lower():
				continue
			print("rose net demo: buy %s for %d Zuly" % [entry["item"]["name"], entry["price"]])
			online.store_window.buy(entry["index"], 1)
			await get_tree().create_timer(1.0).timeout
			break
	print("rose net demo: Zuly %d -> %d" % [money_before, online.net.get_inventory()["money"]])
	# Read what we bought: skill books teach their skill, scrolls cast theirs.
	var use_page: Array = online.net.get_inventory()["pages"][1]
	for i in use_page.size():
		var item = use_page[i]
		if item != null and (item.get("class", "") == "Skill Book" or item.get("class", "") == "Magic Item"):
			print("rose net demo: use ", item["name"])
			online.net.use_item(1, i)
			await get_tree().create_timer(4.0).timeout
	await get_tree().create_timer(2.0).timeout
	var pages: Array = online.net.get_inventory()["pages"]
	for i in pages[0].size():
		if pages[0][i] != null and online.net.sell_price(0, i) >= 0:
			print("rose net demo: sell %s for %d Zuly" % [pages[0][i]["name"], online.net.sell_price(0, i)])
			online.store_window.sell(0, i, 1)
			break
	await get_tree().create_timer(2.0).timeout
	print("rose net demo: Zuly now %d" % online.net.get_inventory()["money"])
