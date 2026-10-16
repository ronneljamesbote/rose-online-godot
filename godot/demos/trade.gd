extends "res://demos/demo.gd"

## Trade test with two clients: with --trade-with=NAME ask that player, without accept the
## first request; then offer --offer=ITEM and --zuly=N, lock, and trade.
func run() -> void:
	var wanted := String(options.get("trade-with", ""))
	if not online.net.get_trade().is_empty():
		online.net.trade_cancel()
		await get_tree().create_timer(2.0).timeout
	var waited := 0.0
	while waited < 90.0 and online.net.get_trade().is_empty():
		await get_tree().create_timer(1.0).timeout
		waited += 1.0
		if wanted != "":
			for id in online.entities:
				if online.entities[id].label.text == wanted:
					print("rose net demo: ask ", wanted, " to trade")
					online.net.trade_ask(id)
					await get_tree().create_timer(4.0).timeout
					break
		elif not online.net.get_trade_requests().is_empty():
			print("rose net demo: accept trade from ", online.net.get_trade_requests()[0][1])
			online.trade_window.answer_request(true)
	if online.net.get_trade().is_empty():
		print("rose net demo: no trade")
		return
	await get_tree().create_timer(2.0).timeout
	var offer := String(options.get("offer", ""))
	if offer != "":
		var found := _find_bag_item(offer)
		if not found.is_empty():
			var item = online.net.get_inventory()["pages"][found[0]][found[1]]
			online.trade_window.add(found[0], found[1], item.get("quantity", 1))
	await get_tree().create_timer(1.0).timeout
	if options.has("zuly"):
		online.trade_window.my_money.value = int(options["zuly"])
	await get_tree().create_timer(3.0).timeout
	_print_trade()
	online.net.trade_lock(true)
	waited = 0.0
	while waited < 30.0 and not online.net.get_trade().is_empty() and not online.net.get_trade()["theirs"]["locked"]:
		await get_tree().create_timer(1.0).timeout
		waited += 1.0
	_print_trade()
	await get_tree().create_timer(float(options.get("hold", "2"))).timeout
	var before: int = online.net.get_inventory()["money"]
	online.net.trade_accept()
	await get_tree().create_timer(4.0).timeout
	print("rose net demo: Zuly %d -> %d" % [before, online.net.get_inventory()["money"]])
	var names := []
	for page in online.net.get_inventory()["pages"]:
		for item in page:
			if item != null:
				names.append("%s x%d" % [item["name"], item.get("quantity", 1)])
	print("rose net demo: bag now ", names)


func _print_trade() -> void:
	var t: Dictionary = online.net.get_trade()
	if t.is_empty():
		print("rose net demo: not trading")
		return
	var side := func(s: Dictionary) -> String:
		return "%s + %d Zuly%s" % [s["items"].map(func(i): return "%s x%d" % [i["name"], i.get("quantity", 1)]), s["money"], " (locked)" if s["locked"] else ""]
	print("rose net demo: trade with %s: mine %s, theirs %s" % [t["with"], side.call(t["mine"]), side.call(t["theirs"])])
