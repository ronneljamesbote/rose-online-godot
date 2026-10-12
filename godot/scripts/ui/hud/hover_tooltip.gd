## What the mouse is over in the world (a monster, a town NPC or an item on the ground),
## shown as a tooltip in the bottom right corner of the screen, above the menu bar.
extends PanelContainer

const WorldOverlay := preload("res://scripts/ui/world_overlay.gd")

var online: Node
var title: Label
var lines: Label
var _shown := ""  # what is shown now, so the labels only change when it does


func _ready() -> void:
	theme_type_variation = "Tooltip"
	mouse_filter = Control.MOUSE_FILTER_IGNORE
	visible = false
	var column := VBoxContainer.new()
	column.add_theme_constant_override("separation", 2)
	column.mouse_filter = Control.MOUSE_FILTER_IGNORE
	add_child(column)
	title = Label.new()
	title.theme_type_variation = "HeaderLabel"
	column.add_child(title)
	lines = Label.new()
	lines.theme_type_variation = "TooltipLabel"
	column.add_child(lines)


func _process(_delta: float) -> void:
	var info := _under_mouse()
	if info.is_empty():
		visible = false
		_shown = ""
		return
	var key := "%s|%s|%s" % [info["title"], info["lines"], info["colour"]]
	if key != _shown:
		_shown = key
		title.text = info["title"]
		title.add_theme_color_override("font_color", info["colour"])
		lines.text = info["lines"]
		lines.visible = info["lines"] != ""
		lines.add_theme_color_override("font_color", UI.color("tooltip.text"))
		title.add_theme_color_override("font_outline_color", Color(0, 0, 0, 0))
		lines.add_theme_color_override("font_outline_color", Color(0, 0, 0, 0))
		size = Vector2.ZERO
	visible = true
	_place()


## Bottom right of the screen, just above the menu bar when that is in the bottom right.
func _place() -> void:
	var screen := get_viewport_rect().size
	var bottom := screen.y - 12.0
	var menu: Control = online.menu_bar if online else null
	if menu and menu.is_visible_in_tree():
		var r := menu.get_global_rect()
		if r.position.x > screen.x * 0.5 and r.end.y > screen.y * 0.75:
			bottom = r.position.y - 10.0
	# The interface size scales the HUD, so the box on screen is bigger than its own size.
	var s := get_combined_minimum_size() * get_global_transform().get_scale()
	global_position = Vector2(screen.x - 12.0 - s.x, bottom - s.y)


## title, lines and colour of what the mouse is over, or {} for nothing (or a window).
func _under_mouse() -> Dictionary:
	if online == null or online.me == null:
		return {}
	var viewport := get_viewport()
	if viewport.gui_get_hovered_control() != null:
		return {}
	var camera := viewport.get_camera_3d()
	if camera == null:
		return {}
	var under: Dictionary = online.under_mouse(camera, viewport.get_mouse_position())
	# The item/entity can be picked up or removed between under_mouse() and here, so
	# look it up safely (a missing key returns null, and null must not reach the helpers).
	match under.get("kind", ""):
		"item":
			var item: Node3D = online.ground.get(under.get("id", -1))
			return _item(item) if item != null else {}
		"monster":
			var monster: Node3D = online.entities.get(under.get("id", -1))
			return _monster(monster) if monster != null else {}
		"npc":
			var npc: Node3D = online.entities.get(under.get("id", -1))
			return _npc(npc) if npc != null else {}
	return {}


func _monster(e: Node3D) -> Dictionary:
	var out: Array[String] = []
	if e.level > 0:
		out.append("Level %d" % e.level)
	if e.max_hp > 0:
		out.append("HP %d / %d" % [e.hp, e.max_hp])
	return {"title": e.label.text if e.label else "Monster", "lines": "\n".join(out), "colour": UI.color("colors.enemy")}


func _npc(e: Node3D) -> Dictionary:
	var out: Array[String] = ["Town NPC, has a store" if e.has_store else "Town NPC", "Click to talk"]
	return {"title": e.label.text if e.label else "NPC", "lines": "\n".join(out), "colour": UI.color("colors.friendly")}


func _item(node: Node3D) -> Dictionary:
	var item: Dictionary = node.get_meta("item", {})
	var grade := WorldOverlay.item_grade(item)
	# The theme's own colours, so the name reads on light and dark tooltips alike.
	var colour: Color = {
		"normal": UI.color("tooltip.text"), "rare": UI.color("colors.rare"), "unique": UI.color("colors.unique"),
		"zuly": UI.color("colors.zuly"), "quest": UI.color("colors.good"), "gem": UI.color("colors.accent"),
	}[grade]
	var name: String = item.get("name", "?")
	var quantity: int = item.get("quantity", 1)
	var out: Array[String] = []
	if item.get("type", "") != "Money":
		if quantity > 1:
			name = "%s (%d)" % [name, quantity]
		var tip: String = item.get("tooltip", "")
		if tip != "":
			out.append(tip)
	out.append("Click to pick up" if node.get_meta("mine", true) else "Another player's drop for now")
	return {"title": name, "lines": "\n".join(out), "colour": colour}
