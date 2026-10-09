## Square minimap, top right. The map always points north; only our arrow turns. Shows
## party members, town NPCs and monsters nearby, the zone name and our position.
## M opens the big zone map.
extends VBoxContainer

## The zone's minimap image has 64 pixels per 160 m map block, starting at the block
## LIST_ZONE.STB gives (as in the original client).
const METRES_PER_PIXEL := 2.5
const BLOCK := 160.0

var online: Node
var pvp := false  # shown next to the zone name
var view_size := 176.0
var zoom := 1.0  # map pixels per screen pixel
var frame: Panel
var view: Control
var ring: Control
var zone_label: Label
var place_label: Label
var _zone_id := -1
var _texture: Texture2D
var _start := Vector2.ZERO


func _ready() -> void:
	mouse_filter = Control.MOUSE_FILTER_IGNORE
	add_theme_constant_override("separation", 4)
	var top := Control.new()
	top.custom_minimum_size = Vector2(view_size, view_size)
	top.mouse_filter = Control.MOUSE_FILTER_IGNORE
	add_child(top)
	# The rounded frame clips the map drawn inside it.
	frame = Panel.new()
	frame.set_anchors_preset(Control.PRESET_FULL_RECT)
	frame.clip_children = CanvasItem.CLIP_CHILDREN_AND_DRAW
	frame.mouse_filter = Control.MOUSE_FILTER_STOP
	frame.gui_input.connect(_on_input)
	top.add_child(frame)
	view = _View.new()
	view.map = self
	view.set_anchors_preset(Control.PRESET_FULL_RECT)
	view.mouse_filter = Control.MOUSE_FILTER_IGNORE
	frame.add_child(view)
	ring = _Ring.new()
	ring.set_anchors_preset(Control.PRESET_FULL_RECT)
	ring.mouse_filter = Control.MOUSE_FILTER_IGNORE
	top.add_child(ring)
	for z in [["+", 0.7, Vector2(-26, -50)], ["–", 1.0 / 0.7, Vector2(-26, -26)]]:
		var b := Button.new()
		b.text = z[0]
		b.theme_type_variation = "FlatButton"
		b.focus_mode = Control.FOCUS_NONE
		b.custom_minimum_size = Vector2(22, 22)
		b.set_anchors_preset(Control.PRESET_BOTTOM_RIGHT)
		b.position = Vector2(view_size, view_size) + z[2]
		b.tooltip_text = "Zoom in" if z[0] == "+" else "Zoom out"
		var factor: float = z[1]
		b.pressed.connect(func(): zoom = clampf(zoom * factor, 0.35, 2.5); view.queue_redraw())
		top.add_child(b)
	# Zone name over the coordinates, both centred under the map.
	var labels := VBoxContainer.new()
	labels.add_theme_constant_override("separation", -4)
	labels.mouse_filter = Control.MOUSE_FILTER_IGNORE
	add_child(labels)
	zone_label = Label.new()
	zone_label.theme_type_variation = "AccentLabel"
	labels.add_child(zone_label)
	place_label = Label.new()
	place_label.theme_type_variation = "MutedLabel"
	labels.add_child(place_label)
	for l in [zone_label, place_label]:
		l.horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER
		l.text_overrun_behavior = TextServer.OVERRUN_TRIM_ELLIPSIS
		l.add_theme_color_override("font_outline_color", Color(0, 0, 0, 0.75))
		l.add_theme_constant_override("outline_size", 4)
	UI.theme_changed.connect(_restyle)
	_restyle()


func _restyle() -> void:
	var box := UiBox.make(Color(0.1, 0.12, 0.2), Color(0.1, 0.12, 0.2), UI.num("map.radius", 8))
	frame.add_theme_stylebox_override("panel", box)
	ring.queue_redraw()


func _on_input(event: InputEvent) -> void:
	if event is InputEventMouseButton and event.pressed:
		if event.button_index == MOUSE_BUTTON_WHEEL_UP:
			zoom = clampf(zoom * 0.85, 0.35, 2.5)
		elif event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
			zoom = clampf(zoom / 0.85, 0.35, 2.5)
		view.queue_redraw()
		frame.accept_event()


func _process(_delta: float) -> void:
	if online == null or online.zone == null:
		return
	var zone_id: int = online.zone.get_meta("zone_id", 0)
	if zone_id != _zone_id:
		_zone_id = zone_id
		_load(zone_id)
	var zone_text := RoseData.zone_name(_zone_id) + ("  ·  PvP" if pvp else "")
	if zone_label.text != zone_text:
		zone_label.text = zone_text
	if online.me:
		var p: Vector3 = online.me.global_position
		place_label.text = "%d, %d" % [int(p.x), int(-p.z)]
	view.queue_redraw()


func _load(zone_id: int) -> void:
	zone_label.text = RoseData.zone_name(zone_id)
	var info: Dictionary = RoseData.zone_minimap(zone_id)
	_texture = RoseData.texture(info["path"]) if info.has("path") else null
	_start = Vector2(info.get("start_x", 0), info.get("start_y", 0))


## Map image pixel of a world position.
func to_map(at: Vector3) -> Vector2:
	return Vector2((at.x - _start.x * BLOCK) / METRES_PER_PIXEL, (at.z + BLOCK * (65.0 - _start.y)) / METRES_PER_PIXEL)


class _View extends Control:
	var map: Node

	func _draw() -> void:
		var online: Node = map.online
		if online == null or online.me == null:
			return
		var centre := size * 0.5
		var me_px: Vector2 = map.to_map(online.me.global_position)
		var z: float = map.zoom
		if map._texture:
			var src := Rect2(me_px - centre * z, size * z)
			draw_texture_rect_region(map._texture, Rect2(Vector2.ZERO, size), src)
		else:
			draw_rect(Rect2(Vector2.ZERO, size), Color(0.16, 0.2, 0.28))
		var monster := Color(1.0, 0.42, 0.42)
		var npc := Color(0.55, 0.95, 0.5)
		var player := Color(1, 1, 1)
		var party := Color(0.45, 0.8, 1.0)
		var party_ids := {}
		for m in online.net.get_party().get("members", []):
			party_ids[int(m.get("entity", -1))] = true
		for id in online.entities:
			var e: Node3D = online.entities[id]
			if e == online.me or e.dead:
				continue
			var at: Vector2 = centre + (map.to_map(e.global_position) - me_px) / z
			if not Rect2(Vector2(4, 4), size - Vector2(8, 8)).has_point(at):
				continue
			if e.is_npc:
				draw_circle(at, 3.2, Color(0, 0, 0, 0.6), true, -1.0, true)
				draw_circle(at, 2.4, npc, true, -1.0, true)
			elif e.is_monster:
				draw_circle(at, 2.0, monster, true, -1.0, true)
			else:
				var c := party if party_ids.has(id) else player
				draw_circle(at, 3.0, Color(0, 0, 0, 0.6), true, -1.0, true)
				draw_circle(at, 2.2, c, true, -1.0, true)
		# Our arrow, turned the way we face.
		var forward: Vector3 = online.me.global_basis.z
		var angle := atan2(forward.x, -forward.z)
		var tip := Vector2(0, -9).rotated(angle)
		var left := Vector2(-6, 6).rotated(angle)
		var right := Vector2(6, 6).rotated(angle)
		var back := Vector2(0, 3).rotated(angle)
		var points := PackedVector2Array([centre + tip, centre + right, centre + back, centre + left])
		var shadow := PackedVector2Array()
		for p in points:
			shadow.append(p + Vector2(0, 1.5))
		draw_colored_polygon(shadow, Color(0, 0, 0, 0.45))
		draw_colored_polygon(points, UI.color("map.me"))
		draw_polyline(points + PackedVector2Array([points[0]]), Color.WHITE, 1.2, true)


class _Ring extends Control:
	func _draw() -> void:
		var box := UiBox.new()
		box.colors = PackedColorArray([Color(0, 0, 0, 0)])
		box.radius = UI.num("map.radius", 8)
		box.border_width = UI.num("map.ring_width", 2)
		box.border_color = UI.color("map.ring_color")
		box.shadow_color = Color(0, 0, 0, 0.3)
		box.shadow_size = 10
		box.shadow_offset = 4
		draw_style_box(box, Rect2(Vector2.ZERO, size))
