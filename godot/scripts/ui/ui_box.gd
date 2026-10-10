## A rounded box with a vertical gradient, border, drop shadow, a thin highlight along the
## top and an inner shade (the "inset" shadow of the mockups). StyleBoxFlat has no
## gradients, so the fill is drawn here as horizontal bands.
class_name UiBox
extends StyleBox

## Fill colours from top to bottom, at the matching stops (0 to 1).
@export var colors := PackedColorArray([Color.WHITE])
@export var stops := PackedFloat32Array([0.0])
@export var radius := 0.0
@export var border_width := 0.0
@export var border_color := Color(0, 0, 0, 0)
## Draw the border on these sides only (left, top, right, bottom).
@export var border_sides := [true, true, true, true]
@export var shadow_color := Color(0, 0, 0, 0)
@export var shadow_size := 0.0
@export var shadow_offset := 0.0
@export var highlight := Color(0, 0, 0, 0)
@export var inner_shadow := Color(0, 0, 0, 0)
@export var inner_shadow_size := 4.0
## Glossy sheen over the top half (bars), 0 to 1.
@export var sheen := 0.0
## Draw the box this far inside its rect (left, top, right, bottom), so its drop shadow fits
## in controls that clip what is drawn outside them, such as tooltip pop-ups.
@export var inset := Vector4.ZERO

var _shadow_box: StyleBoxFlat
var _border_box: StyleBoxFlat


static func make(top: Color, bottom: Color, corner := 0.0) -> UiBox:
	var box := UiBox.new()
	box.colors = PackedColorArray([top, bottom])
	box.stops = PackedFloat32Array([0.0, 1.0])
	box.radius = corner
	return box


func set_margins(left: float, top: float, right: float, bottom: float) -> UiBox:
	content_margin_left = left
	content_margin_top = top
	content_margin_right = right
	content_margin_bottom = bottom
	return self


func set_margin_all(m: float) -> UiBox:
	return set_margins(m, m, m, m)


func _draw(to_canvas_item: RID, rect: Rect2) -> void:
	rect = rect.grow_individual(-inset.x, -inset.y, -inset.z, -inset.w)
	var r := minf(radius, minf(rect.size.x, rect.size.y) * 0.5)
	if shadow_size > 0.0 and shadow_color.a > 0.0:
		if _shadow_box == null:
			_shadow_box = StyleBoxFlat.new()
			_shadow_box.draw_center = false
			_shadow_box.bg_color = Color(0, 0, 0, 0)
			_shadow_box.anti_aliasing = true
		_shadow_box.shadow_color = shadow_color
		_shadow_box.shadow_size = int(shadow_size)
		_shadow_box.set_corner_radius_all(int(r))
		# Moving the top edge down instead of the whole shadow keeps the shadow under
		# the bottom edge without a gap, like a CSS shadow with a y offset.
		var shadow_rect := Rect2(rect.position + Vector2(0, shadow_offset), rect.size - Vector2(0, shadow_offset))
		_shadow_box.draw(to_canvas_item, shadow_rect)
	_fill(to_canvas_item, rect, r, colors, stops, 0.0, rect.size.y)
	if inner_shadow.a > 0.0:
		var clear := Color(inner_shadow, 0.0)
		_fill(to_canvas_item, rect, r, PackedColorArray([inner_shadow, clear]), PackedFloat32Array([0.0, 1.0]), 0.0, minf(inner_shadow_size, rect.size.y))
	if sheen > 0.0:
		var top := Color(1, 1, 1, 0.5 * sheen)
		var mid := Color(1, 1, 1, 0.12 * sheen)
		var low := Color(0, 0, 0, 0.05 * sheen)
		var bottom := Color(0, 0, 0, 0.2 * sheen)
		_fill(to_canvas_item, rect, r, PackedColorArray([top, mid, low, bottom]), PackedFloat32Array([0.0, 0.48, 0.52, 1.0]), 0.0, rect.size.y)
	if highlight.a > 0.0 and rect.size.x > r * 2.0 + 2.0:
		var y := rect.position.y + border_width + 0.5
		RenderingServer.canvas_item_add_line(to_canvas_item, Vector2(rect.position.x + r, y), Vector2(rect.end.x - r, y), highlight, 1.0, true)
	if border_width > 0.0 and border_color.a > 0.0:
		if _border_box == null:
			_border_box = StyleBoxFlat.new()
			_border_box.draw_center = false
			_border_box.anti_aliasing = true
		_border_box.border_color = border_color
		var w := int(ceilf(border_width))
		_border_box.border_width_left = w if border_sides[0] else 0
		_border_box.border_width_top = w if border_sides[1] else 0
		_border_box.border_width_right = w if border_sides[2] else 0
		_border_box.border_width_bottom = w if border_sides[3] else 0
		_border_box.set_corner_radius_all(int(r))
		_border_box.draw(to_canvas_item, rect)


## Fills the rounded rectangle between local heights from_y and to_y with a gradient that
## runs over that range.
static func _fill(ci: RID, rect: Rect2, r: float, cols: PackedColorArray, at: PackedFloat32Array, from_y: float, to_y: float) -> void:
	var h := rect.size.y
	var w := rect.size.x
	if w <= 0.0 or to_y <= from_y or cols.is_empty():
		return
	var ys := PackedFloat32Array([from_y, to_y])
	if r > 0.5:
		var n := clampi(int(r / 2.0), 3, 10)
		for i in n + 1:
			var a := PI * 0.5 * i / n
			ys.append(r - r * sin(a))
			ys.append(h - r + r * sin(a))
	for s in at:
		ys.append(from_y + s * (to_y - from_y))
	ys.sort()
	var points := PackedVector2Array()
	var colours := PackedColorArray()
	var last := -1.0
	for y in ys:
		if y < from_y or y > to_y or (last >= 0.0 and y - last < 0.01):
			continue
		last = y
		var dx := 0.0
		if y < r:
			dx = r - sqrt(maxf(0.0, r * r - (r - y) * (r - y)))
		elif y > h - r:
			var t := y - (h - r)
			dx = r - sqrt(maxf(0.0, r * r - t * t))
		var c := _color_at((y - from_y) / (to_y - from_y), cols, at)
		points.append(rect.position + Vector2(dx, y))
		points.append(rect.position + Vector2(w - dx, y))
		colours.append(c)
		colours.append(c)
	var indices := PackedInt32Array()
	for i in range(0, points.size() - 2, 2):
		indices.append_array([i, i + 1, i + 3, i, i + 3, i + 2])
	if indices.is_empty():
		return
	RenderingServer.canvas_item_add_triangle_array(ci, indices, points, colours)


static func _color_at(t: float, cols: PackedColorArray, at: PackedFloat32Array) -> Color:
	if cols.size() == 1 or t <= at[0]:
		return cols[0]
	for i in range(1, cols.size()):
		if t <= at[i]:
			var span := at[i] - at[i - 1]
			return cols[i - 1].lerp(cols[i], (t - at[i - 1]) / span if span > 0.0 else 1.0)
	return cols[cols.size() - 1]
