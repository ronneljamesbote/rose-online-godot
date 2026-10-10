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
	var mouse := viewport.get_mouse_position()
	var item_id: int = online.world_overlay.item_label_at(mouse) if online.world_overlay else -1
	if item_id < 0:
		item_id = online.pick_item(camera, mouse)
	if item_id >= 0 and online.ground.has(item_id):
		return _item(online.ground[item_id])
	var id: int = online.pick_monster(camera, mouse)
	if id >= 0:
		return _monster(online.entities[id])
	id = online.pick_npc(camera, mouse)
	if id >= 0:
		return _npc(online.entities[id])
	return {}


func _monster(e: Node3D) -> Dictionary:
	var out: Array[String] = []
	if e.level > 0:
		out.append("Level %d" % e.level)
	if e.max_hp > 0:
		out.append("HP %d / %d" % [e.hp, e.max_hp])
	return {"title": e.label.text if e.label else "Monster", "lines": "\n".join(out), "colour": UI.color("tags.monster")}


func _npc(e: Node3D) -> Dictionary:
	var out: Array[String] = ["Town NPC, has a store" if e.has_store else "Town NPC", "Click to talk"]
	return {"title": e.label.text if e.label else "NPC", "lines": "\n".join(out), "colour": UI.color("tags.npc")}


func _item(node: Node3D) -> Dictionary:
	var item: Dictionary = node.get_meta("item", {})
	var grade := WorldOverlay.item_grade(item)
	var colour: Color = UI.color("colors.unique").lightened(0.15) if grade == "unique" else WorldOverlay.ITEM_COLOURS[grade]
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
