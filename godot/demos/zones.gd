extends "res://demos/demo.gd"

## Loads the zones of --zones=ID,ID,... one after another the way a warp gate does: the
## loading screen shows and the zone loads in steps beside the old one. Prints each load's
## times (as --perf-log) and the longest frame while it loaded. With --preload=SECONDS it
## first reads each zone ahead for that long (as standing near its gate does); --focus=X,Z
## builds the effects around that spot first; --shots=DIR saves a picture once each zone
## shows. Then quits.
func run() -> void:
	var tree := get_tree()
	if main.player:
		main.player.visible = false
	if main.loading == null:
		main.loading = preload("res://scripts/loading_screen.gd").new()
		main.add_child(main.loading)
	if main.zone_preload == null:
		main.zone_preload = preload("res://scripts/zone_preload.gd").new()
		main.add_child(main.zone_preload)
	var focus := {}
	if options.has("focus"):
		var at: PackedFloat64Array = str(options["focus"]).split_floats(",")
		focus = {"x": at[0], "z": at[1]}
	await tree.create_timer(1.0).timeout
	for id in str(options.get("zones", "20")).split(","):
		var zone_id := int(id)
		if options.has("preload"):
			main.zone_preload.want(zone_id)
			await tree.create_timer(float(options["preload"])).timeout
			_say("zone %d read ahead: %s" % [zone_id, main.zone_preload.zone_id == zone_id and not main.zone_preload.is_processing()])
		main.loading.show_zone(RoseData.zone_name(zone_id))
		await tree.process_frame
		var started := Time.get_ticks_msec()
		var loaded: bool = await main._load_zone_in_steps(zone_id, focus)
		_say("zone %d %s after %d ms" % [zone_id, "loaded" if loaded else "failed", Time.get_ticks_msec() - started])
		if main.player:
			main.player.zone = main.zone
		main.loading.hide_screen()
		await tree.create_timer(1.0).timeout
		if options.has("shots"):
			main.get_viewport().get_texture().get_image().save_png("%s/zone-%d.png" % [options["shots"], zone_id])
	_say("done")
	tree.quit()


func _say(text: String) -> void:
	print("rose demo: ", text)
