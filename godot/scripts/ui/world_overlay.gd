## Everything drawn over the 3D world, under the interface: name tags with life bars,
## ground item labels in the style of Path of Exile (text in the item's grade colour on a
## dark box, a beam over the best drops, pushed apart so they never overlap), chat
## bubbles and damage numbers. Positions come from the camera each frame.
extends CanvasLayer

const NPC_NAME_RANGE := 30.0
const MONSTER_NAME_RANGE := 15.0
const PLAYER_NAME_RANGE := 45.0
const ITEM_LABEL_RANGE := 30.0
const BUBBLE_SECONDS := 6.0
const ITEM_COLOURS := {
	"normal": Color(0.93, 0.93, 0.93),
	"rare": Color(0.56, 0.69, 1.0),
	"quest": Color(0.49, 0.88, 0.54),
	"zuly": Color(0.91, 0.77, 0.42),
	"gem": Color(0.84, 0.64, 1.0),
}

var online: Node
var pvp := false
var _canvas: Control
var _floats: Array = []  # {at: Vector3, text, colour, size, born, seconds, dx}
var _bubbles := {}  # entity -> {text, until}
var _labels: Array = []  # this frame's item labels: {rect: Rect2, id}


func _ready() -> void:
	layer = 0
	add_to_group("world_overlay")
	_canvas = Control.new()
	_canvas.set_anchors_preset(Control.PRESET_FULL_RECT)
	_canvas.mouse_filter = Control.MOUSE_FILTER_IGNORE
	_canvas.draw.connect(_draw_all)
	add_child(_canvas)


func _process(_delta: float) -> void:
	_canvas.queue_redraw()


## A chat bubble over someone's head (players nearby see what was said).
func bubble(entity: Node3D, text: String) -> void:
	if not UI.settings.get("bubbles", true):
		return
	_bubbles[entity] = {"text": text, "until": Time.get_ticks_msec() + int(BUBBLE_SECONDS * 1000)}


## Text that rises and fades above an entity (damage, experience, level up).
func float_text(over: Node3D, text: String, colour: Color, size: int, seconds := 1.0) -> void:
	if not UI.settings.get("damage_numbers", true) and text.is_valid_int():
		return
	_floats.append({
		"at": over.global_position + Vector3(0, over.height * 0.8, 0),
		"text": text, "colour": colour, "size": size,
		"born": Time.get_ticks_msec(), "seconds": seconds, "dx": randf_range(-14.0, 14.0),
	})


func _input(event: InputEvent) -> void:
	# Clicking an item label picks the item up, unless a window is under the mouse.
	if event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_LEFT:
		if get_viewport().gui_get_hovered_control() != null:
			return
		for label in _labels:
			if label["rect"].has_point(event.position):
				online.pickup(label["id"])
				get_viewport().set_input_as_handled()
				return


func _draw_all() -> void:
	_labels.clear()
	var camera := get_viewport().get_camera_3d()
	if camera == null or online == null:
		return
	var s := clampf(float(UI.settings.get("scale", 1.0)), 0.75, 1.5)
	var font := _canvas.get_theme_font("bold_font", "Fonts")
	_draw_items(camera, font, s)
	_draw_tags(camera, font, s)
	_draw_bubbles(camera, font, s)
	_draw_floats(camera, font, s)


func _screen(camera: Camera3D, at: Vector3):
	if camera.is_position_behind(at):
		return null
	return camera.unproject_position(at)


func _draw_tags(camera: Camera3D, font: Font, s: float) -> void:
	var me: Node3D = online.me
	if me == null:
		return
	var party := {}
	for m in online.net.get_party().get("members", []):
		party[int(m.get("entity", -1))] = true
	var my_level: int = me.level
	var tag_outline := UI.color("tags.outline")
	var fs := int(round(13 * s))
	for id in online.entities:
		var e: Node3D = online.entities[id]
		if e.label:
			e.label.visible = false
		if e.dying or (e.dead and e.is_monster):
			continue
		var d: float = e.position.distance_to(me.position)
		var show := false
		var bar := false
		var colour := UI.color("tags.player")
		var text: String = e.label.text if e.label else ""
		var level_text := ""
		var level_colour := Color(0, 0, 0, 0)
		if e == me:
			show = UI.settings.get("own_tag", true)
			bar = true
			colour = UI.color("colors.accent")
		elif e.is_npc:
			show = d < NPC_NAME_RANGE
			colour = UI.color("tags.npc")
		elif e.is_monster:
			var hurt: bool = e.hp < e.max_hp
			show = id == online.my_target or (d < MONSTER_NAME_RANGE)
			bar = (hurt or id == online.my_target) and not e.is_summon
			colour = UI.color("tags.monster")
			if e.level > 0 and not e.is_summon:
				level_text = "Lv %d " % e.level
				var gap: int = e.level - my_level
				level_colour = Color(1.0, 0.45, 0.4) if gap >= 5 else (Color(0.7, 0.7, 0.7) if gap <= -10 else Color(1.0, 0.92, 0.6))
		else:
			show = UI.settings.get("name_tags", true) and d < PLAYER_NAME_RANGE
			bar = true
			if party.has(id):
				colour = UI.color("tags.party")
			elif pvp and online.net.is_enemy_player(id):
				colour = UI.color("colors.enemy").lightened(0.2)
		if not show or text == "":
			continue
		var head: float = e.height + 0.3
		if e.get("shop_title") != null and e.shop_title != "":
			head += 0.45  # above the shop sign
		var p = _screen(camera, e.global_position + Vector3(0, head, 0))
		if p == null:
			continue
		var alpha := 1.0 if e != me else 0.95
		if e.hidden:
			alpha *= 0.5
		var name_w := font.get_string_size(text, HORIZONTAL_ALIGNMENT_LEFT, -1, fs).x
		var level_w := font.get_string_size(level_text, HORIZONTAL_ALIGNMENT_LEFT, -1, fs - 2).x if level_text != "" else 0.0
		var total := name_w + level_w
		var y: float = p.y - (10.0 * s if bar else 0.0)
		var x: float = p.x - total * 0.5
		if level_text != "":
			_text(font, Vector2(x, y), level_text, fs - 2, Color(level_colour, alpha), Color(tag_outline, alpha), 4)
		_text(font, Vector2(x + level_w, y), text, fs, Color(colour, alpha), Color(tag_outline, alpha), 4)
		if bar and e.max_hp > 0:
			var w := 64.0 * s
			var h := maxf(4.0, 5.0 * s)
			var rect := Rect2(Vector2(p.x - w * 0.5, y + 5.0 * s), Vector2(w, h))
			_canvas.draw_rect(rect.grow(1.0), Color(0, 0, 0, 0.6 * alpha))
			var part := clampf(float(e.hp) / float(e.max_hp), 0.0, 1.0)
			var hp_colour := UI.color("bars.hp")
			if e == me or party.has(id):
				hp_colour = Color(0.36, 0.86, 0.48) if part > 0.3 else hp_colour
			_canvas.draw_rect(Rect2(rect.position, Vector2(w * part, h)), Color(hp_colour, alpha))
			_canvas.draw_rect(Rect2(rect.position, Vector2(w * part, h * 0.45)), Color(1, 1, 1, 0.25 * alpha))


func _text(font: Font, at: Vector2, text: String, size: int, colour: Color, outline: Color, outline_size: int) -> void:
	if outline.a > 0.0:
		_canvas.draw_string_outline(font, at, text, HORIZONTAL_ALIGNMENT_LEFT, -1, size, outline_size, outline)
	_canvas.draw_string(font, at, text, HORIZONTAL_ALIGNMENT_LEFT, -1, size, colour)


## Grade of a ground item: normal, rare (a bonus option or a gem socket), unique (both),
## quest, zuly or gem.
static func item_grade(item: Dictionary) -> String:
	var type: String = item.get("type", "")
	if type == "Money":
		return "zuly"
	if type == "Quest":
		return "quest"
	if type == "Gem":
		return "gem"
	var bonus: bool = item.get("bonus", false)
	var socket: bool = item.get("socket", false)
	if bonus and socket:
		return "unique"
	if bonus or socket or int(item.get("grade", 0)) > 0:
		return "rare"
	return "normal"


func _draw_items(camera: Camera3D, font: Font, s: float) -> void:
	var mode: String = UI.settings.get("item_labels", "always")
	var alt := Input.is_key_pressed(KEY_ALT)
	if (mode == "always" and alt) or (mode == "alt" and not alt):
		return
	var me: Node3D = online.me
	var fs := int(round(13 * s))
	var pad := Vector2(7, 3) * s
	var wanted: Array = []
	for id in online.ground:
		var node: Node3D = online.ground[id]
		if me and node.position.distance_to(me.position) > ITEM_LABEL_RANGE:
			continue
		var p = _screen(camera, node.global_position + Vector3(0, 0.35, 0))
		if p == null:
			continue
		var item: Dictionary = node.get_meta("item", {})
		var text: String = item.get("name", "?")
		var quantity: int = item.get("quantity", 1)
		if item.get("type", "") == "Money":
			text = "%d Zuly" % quantity
		elif quantity > 1:
			text = "%s (%d)" % [text, quantity]
		var w := font.get_string_size(text, HORIZONTAL_ALIGNMENT_LEFT, -1, fs).x
		var size := Vector2(w, fs) + pad * 2.0
		wanted.append({"id": id, "text": text, "grade": item_grade(item), "mine": node.get_meta("mine", true),
			"rect": Rect2(p - Vector2(size.x * 0.5, size.y), size), "ground": p})
	# Nearest the camera first; each label moves up until it overlaps no label placed before it.
	wanted.sort_custom(func(a, b): return a["ground"].y > b["ground"].y)
	var placed: Array[Rect2] = []
	for label in wanted:
		var rect: Rect2 = label["rect"]
		var moved := true
		var tries := 0
		while moved and tries < 30:
			moved = false
			tries += 1
			for other in placed:
				if rect.grow(1.5 * s).intersects(other):
					rect.position.y = other.position.y - rect.size.y - 2.0 * s
					moved = true
		placed.append(rect)
		label["rect"] = rect
	var unique := UI.color("colors.unique")
	for label in wanted:
		var rect: Rect2 = label["rect"]
		var grade: String = label["grade"]
		var colour: Color = unique.lightened(0.15) if grade == "unique" else ITEM_COLOURS[grade]
		var alpha := 1.0 if label["mine"] else 0.5
		if grade == "unique":
			# A beam of light over the best drops.
			var g: Vector2 = label["ground"]
			var beam := Rect2(Vector2(g.x - 5.0 * s, g.y - 180.0 * s), Vector2(10.0 * s, 180.0 * s))
			var box := UiBox.make(Color(unique, 0.0), Color(unique, 0.55 * alpha), 4)
			_canvas.draw_style_box(box, beam)
			_canvas.draw_style_box(UiBox.make(Color(1, 1, 1, 0.0), Color(1, 1, 1, 0.5 * alpha), 2), Rect2(Vector2(g.x - 1.5 * s, g.y - 160.0 * s), Vector2(3.0 * s, 160.0 * s)))
		if rect.position.y + rect.size.y < label["ground"].y - 2.0:
			_canvas.draw_line(Vector2(label["ground"].x, rect.end.y), label["ground"], Color(colour, 0.35 * alpha), 1.0, true)
		_canvas.draw_rect(rect, Color(0.04, 0.04, 0.06, 0.82 * alpha))
		_canvas.draw_rect(rect, Color(colour, 0.85 * alpha), false, 1.0)
		_canvas.draw_string(font, rect.position + Vector2(pad.x, pad.y + fs * 0.82), label["text"], HORIZONTAL_ALIGNMENT_LEFT, -1, fs, Color(colour, alpha))
		if label["mine"]:
			_labels.append({"rect": rect, "id": label["id"]})


func _draw_bubbles(camera: Camera3D, font: Font, s: float) -> void:
	var now := Time.get_ticks_msec()
	var fs := int(round(13 * s))
	var top := UI.color("tooltip.top")
	var bottom := UI.color("tooltip.bottom")
	var text_colour := UI.color("tooltip.text")
	for entity in _bubbles.keys():
		var b: Dictionary = _bubbles[entity]
		if not is_instance_valid(entity) or now > b["until"]:
			_bubbles.erase(entity)
			continue
		var p = _screen(camera, entity.global_position + Vector3(0, entity.height + 0.3, 0))
		if p == null:
			continue
		var width := 220.0 * s
		var text_size := font.get_multiline_string_size(b["text"], HORIZONTAL_ALIGNMENT_LEFT, width, fs)
		var box_size := text_size + Vector2(20, 12) * s
		var rect := Rect2(Vector2(p.x - box_size.x * 0.5, p.y - 30.0 * s - box_size.y), box_size)
		var fade := clampf((b["until"] - now) / 400.0, 0.0, 1.0)
		var box := UiBox.make(Color(top, top.a * fade), Color(bottom, bottom.a * fade), 10.0 * s)
		box.shadow_color = Color(0, 0, 0, 0.25 * fade)
		box.shadow_size = 6
		box.shadow_offset = 2
		box.border_width = 1.0
		box.border_color = Color(UI.color("tooltip.border_color"), UI.color("tooltip.border_color").a * fade)
		_canvas.draw_style_box(box, rect)
		var tail := PackedVector2Array([Vector2(p.x - 7 * s, rect.end.y - 1), Vector2(p.x + 7 * s, rect.end.y - 1), Vector2(p.x, rect.end.y + 8 * s)])
		_canvas.draw_colored_polygon(tail, Color(bottom, bottom.a * fade))
		_canvas.draw_multiline_string(font, rect.position + Vector2(10, 6 + fs * 0.8) * s, b["text"], HORIZONTAL_ALIGNMENT_CENTER, text_size.x, fs, -1, Color(text_colour, fade))


func _draw_floats(camera: Camera3D, font: Font, s: float) -> void:
	var now := Time.get_ticks_msec()
	var keep: Array = []
	for f in _floats:
		var t: float = (now - f["born"]) / 1000.0 / f["seconds"]
		if t >= 1.0:
			continue
		keep.append(f)
		var p = _screen(camera, f["at"])
		if p == null:
			continue
		var fs := int(round(f["size"] * 0.75 * s * (1.0 + 0.25 * maxf(0.0, 1.0 - t * 6.0))))
		var text: String = f["text"]
		var w := font.get_string_size(text, HORIZONTAL_ALIGNMENT_LEFT, -1, fs).x
		var at: Vector2 = p + Vector2(f["dx"] * s - w * 0.5, -t * 70.0 * s)
		var alpha := 1.0 if t < 0.5 else 1.0 - (t - 0.5) * 2.0
		var colour: Color = f["colour"]
		_text(font, at, text, fs, Color(colour, alpha), Color(0, 0, 0, 0.85 * alpha), maxi(4, int(fs * 0.25)))
	_floats = keep
