extends "res://demos/demo.gd"

## Offline, --demo=interface: shows what the interface settings (`res://data/interface/`)
## link. The loading screen over the zone for two seconds, the character creation backdrop,
## then every mouse cursor drawn 96 pixels wide (our own, and the Classic ones below) with a
## red dot on the pixel it clicks at. --shots=DIR saves a picture of each
## (loading.png, character-create.png, cursors.png). Then quits.


func run() -> void:
	var tree := get_tree()
	var shots: String = options.get("shots", "")
	var loading: CanvasLayer = preload("res://scripts/loading_screen.gd").new()
	main.add_child(loading)
	loading.show_zone(RoseData.zone_name(int(options.get("zone", "1"))))
	await tree.create_timer(2.0).timeout
	_shot(shots, "loading")
	loading.queue_free()

	var layer := CanvasLayer.new()
	layer.layer = 50
	main.add_child(layer)
	var backdrop := TextureRect.new()
	backdrop.set_anchors_preset(Control.PRESET_FULL_RECT)
	backdrop.texture = RoseData.interface_picture("character_create")
	backdrop.expand_mode = TextureRect.EXPAND_IGNORE_SIZE
	backdrop.stretch_mode = TextureRect.STRETCH_KEEP_ASPECT_COVERED
	backdrop.modulate = Color(0.45, 0.45, 0.55)
	layer.add_child(backdrop)
	_say("character creation backdrop %s" % ("shown" if backdrop.texture else "missing"))
	await tree.create_timer(0.5).timeout
	_shot(shots, "character-create")

	var panel := PanelContainer.new()
	panel.set_anchors_and_offsets_preset(Control.PRESET_CENTER)
	panel.grow_horizontal = Control.GROW_DIRECTION_BOTH
	panel.grow_vertical = Control.GROW_DIRECTION_BOTH
	var style := StyleBoxFlat.new()
	style.bg_color = Color(0.08, 0.07, 0.1, 0.85)
	style.set_content_margin_all(24)
	panel.add_theme_stylebox_override("panel", style)
	layer.add_child(panel)
	var rows := VBoxContainer.new()
	rows.add_theme_constant_override("separation", 18)
	panel.add_child(rows)
	var cursors := 0
	for classic in [false, true]:
		var row := HBoxContainer.new()
		row.add_theme_constant_override("separation", 28)
		rows.add_child(row)
		for state in preload("res://scripts/ui/game_cursor.gd").CURSORS:
			var cursor: Dictionary = RoseData.classic_cursor(state) if classic else RoseData.cursor(state)
			var column := VBoxContainer.new()
			var picture := TextureRect.new()
			picture.custom_minimum_size = Vector2(96, 96)
			picture.texture_filter = CanvasItem.TEXTURE_FILTER_NEAREST if classic else CanvasItem.TEXTURE_FILTER_LINEAR
			picture.expand_mode = TextureRect.EXPAND_IGNORE_SIZE
			if not cursor.is_empty():
				cursors += 1
				var image: Image = cursor["image"]
				picture.texture = ImageTexture.create_from_image(image)
				var dot := ColorRect.new()
				dot.color = Color.RED
				dot.size = Vector2(5, 5)
				dot.position = (cursor["click"] + Vector2(0.5, 0.5)) * 96.0 / image.get_width() - Vector2(2.5, 2.5)
				picture.add_child(dot)
			column.add_child(picture)
			var label := Label.new()
			label.text = state + (" (classic)" if classic else "")
			label.horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER
			column.add_child(label)
			row.add_child(column)
			_say("%scursor %s: %s" % ["classic " if classic else "", state, ("click at %s" % cursor["click"]) if not cursor.is_empty() else "none"])
	_say("%d cursors" % cursors)
	await tree.create_timer(0.5).timeout
	_shot(shots, "cursors")
	tree.quit()


func _shot(shots: String, name: String) -> void:
	if shots != "":
		main.get_viewport().get_texture().get_image().save_png("%s/%s.png" % [shots, name])


func _say(text: String) -> void:
	print("rose demo: ", text)
