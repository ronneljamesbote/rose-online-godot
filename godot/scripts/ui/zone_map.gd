## The zone map (M): the whole minimap image with town NPCs named, party members and us.
## North is always up.
extends PanelContainer

var online: Node
var view: Control


func _ready() -> void:
	theme_type_variation = "Clear"
	view = _Map.new()
	view.owner_map = self
	view.custom_minimum_size = Vector2(520, 520)
	add_child(view)


func _process(_delta: float) -> void:
	if visible:
		view.queue_redraw()


class _Map extends Control:
	var owner_map: Node

	func _draw() -> void:
		var online: Node = owner_map.online
		if online == null or online.minimap == null:
			return
		var minimap: Node = online.minimap
		var tex: Texture2D = minimap._texture
		var rect := Rect2(Vector2.ZERO, size)
		var box := UiBox.make(Color(0.1, 0.12, 0.2), Color(0.1, 0.12, 0.2), 8)
		draw_style_box(box, rect)
		if tex == null:
			var font := get_theme_font("font", "Label")
			draw_string(font, Vector2(20, 40), "No map for this zone", HORIZONTAL_ALIGNMENT_LEFT, -1, 16, Color.WHITE)
			return
		var k := minf(size.x / tex.get_width(), size.y / tex.get_height())
		var img := Rect2((size - tex.get_size() * k) * 0.5, tex.get_size() * k)
		draw_texture_rect(tex, img, false)
		var font := get_theme_font("bold_font", "Fonts")
		# Short names (without the [job] part) where they fit; a name that would cover
		# another is left out, and the NPC under the mouse always shows its full name.
		var mouse := get_local_mouse_position()
		var placed: Array[Rect2] = []
		var hovered := ""
		var hovered_at := Vector2.ZERO
		for id in online.entities:
			var e: Node3D = online.entities[id]
			if not e.is_npc or e.label == null:
				continue
			var at: Vector2 = img.position + minimap.to_map(e.global_position) * k
			if not img.has_point(at):
				continue
			draw_circle(at, 4.0, Color(0, 0, 0, 0.6), true, -1.0, true)
			draw_circle(at, 3.0, Color(0.55, 0.95, 0.5), true, -1.0, true)
			var full: String = e.label.text
			if at.distance_to(mouse) < 7.0:
				hovered = full
				hovered_at = at
				continue
			var short := full.get_slice("]", 1).strip_edges() if full.begins_with("[") else full
			var w := font.get_string_size(short, HORIZONTAL_ALIGNMENT_LEFT, -1, 11).x
			var tag := Rect2(at + Vector2(6, -8), Vector2(w, 12)).grow(1)
			var free := true
			for r in placed:
				if r.intersects(tag):
					free = false
					break
			if free:
				placed.append(tag)
				_name(font, at, short)
		if hovered != "":
			_name(font, hovered_at, hovered)
		for m in online.net.get_party().get("members", []):
			var pid := int(m.get("entity", -1))
			if online.entities.has(pid) and pid != online.my_id:
				var at: Vector2 = img.position + minimap.to_map(online.entities[pid].global_position) * k
				draw_circle(at, 4.0, Color(0.45, 0.8, 1.0), true, -1.0, true)
		if online.me:
			var at: Vector2 = img.position + minimap.to_map(online.me.global_position) * k
			var forward: Vector3 = online.me.global_basis.z
			var angle := atan2(forward.x, -forward.z)
			var points := PackedVector2Array()
			for v in [Vector2(0, -11), Vector2(7, 7), Vector2(0, 3), Vector2(-7, 7)]:
				points.append(at + v.rotated(angle))
			draw_colored_polygon(points, UI.color("map.me"))
			draw_polyline(points + PackedVector2Array([points[0]]), Color.WHITE, 1.5, true)


	func _name(font: Font, at: Vector2, text: String) -> void:
		draw_string_outline(font, at + Vector2(6, 4), text, HORIZONTAL_ALIGNMENT_LEFT, -1, 11, 3, Color(0, 0, 0, 0.8))
		draw_string(font, at + Vector2(6, 4), text, HORIZONTAL_ALIGNMENT_LEFT, -1, 11, Color.WHITE)
