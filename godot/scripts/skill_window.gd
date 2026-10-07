## Skill window (K): the skill pages with icons, levels and tooltips. Right-click (or
## double-click) a skill to use it; select one and press Level up to spend skill points on
## its next level. Drag skills onto the hotbar.
extends PanelContainer

const InventoryWindow := preload("res://scripts/inventory_window.gd")
const PAGES := ["Basic", "Active", "Passive", "Clan"]

var net: RoseNet
var online: Node  # online.gd, which casts skills on the current target
var tabs: TabBar
var page := 1
var slots: Array = []
var selected = null
var points_label: Label
var level_button: Button
var _last := []
var _last_points := -1


func _ready() -> void:
	var margin := MarginContainer.new()
	for side in ["left", "right", "top", "bottom"]:
		margin.add_theme_constant_override("margin_" + side, 10)
	add_child(margin)
	var box := VBoxContainer.new()
	box.add_theme_constant_override("separation", 6)
	margin.add_child(box)

	var title := Label.new()
	title.text = "Skills"
	title.add_theme_font_size_override("font_size", 18)
	box.add_child(title)

	tabs = TabBar.new()
	for name in PAGES:
		tabs.add_tab(name)
	tabs.current_tab = page
	tabs.focus_mode = Control.FOCUS_NONE
	tabs.tab_changed.connect(func(t): page = t; _clear_selection(); _refresh(true))
	box.add_child(tabs)
	var grid := GridContainer.new()
	grid.columns = 6
	box.add_child(grid)
	for i in 30:
		var slot = InventoryWindow.Slot.new(self, "skill", i)
		grid.add_child(slot)
		slots.append(slot)

	var bottom := HBoxContainer.new()
	box.add_child(bottom)
	points_label = Label.new()
	points_label.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	bottom.add_child(points_label)
	level_button = Button.new()
	level_button.text = "Level up"
	level_button.focus_mode = Control.FOCUS_NONE
	level_button.disabled = true
	level_button.pressed.connect(_level_up_selected)
	bottom.add_child(level_button)


func _process(_delta: float) -> void:
	if visible and net != null:
		_refresh(false)


func _refresh(force: bool) -> void:
	var pages: Array = net.get_skills()
	if pages.is_empty():
		return
	var points: int = net.get_character().get("skill_points", 0)
	var skills: Array = pages[page]
	if not force and skills == _last and points == _last_points:
		return
	_last = skills
	_last_points = points
	points_label.text = "%d skill points" % points
	for i in slots.size():
		var skill = skills[i] if i < skills.size() else null
		if skill != null:
			skill = skill.duplicate()
			skill["quantity"] = skill["level"]  # the slot's count label shows the level
		slots[i].show_item(skill)
	_update_level_button()


func _update_level_button() -> void:
	var cost: int = selected.item.get("next_cost", -1) if selected and selected.item else -1
	level_button.disabled = cost < 0 or cost > _last_points
	level_button.text = "Level up (%d)" % cost if cost >= 0 else "Level up"


func select(slot) -> void:
	_clear_selection()
	if slot.item == null:
		return
	selected = slot
	slot.selected = true
	slot.queue_redraw()
	_update_level_button()


func _clear_selection() -> void:
	if selected:
		selected.selected = false
		selected.queue_redraw()
	selected = null
	if level_button:
		_update_level_button()


func activate(slot) -> void:
	if slot.item != null and online:
		online.use_skill(page, slot.index, slot.item)


func dropped(_from, _to) -> void:
	pass


func _level_up_selected() -> void:
	if selected and selected.item:
		net.level_up_skill(page, selected.index)
