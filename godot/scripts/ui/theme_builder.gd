## Turns the values of a theme file into a Godot Theme: styleboxes, fonts and colours for
## every control, plus the type variations the windows and HUD use (WindowPanel,
## ButtonPrimary, Slot, HeaderLabel ...).
extends RefCounted

const UiBoxScript := preload("res://scripts/ui/ui_box.gd")


static func build(ui: Node) -> Theme:
	var t := Theme.new()
	var c := func(path: String) -> Color: return ui.color(path)
	var n := func(path: String, fallback := 0.0) -> float: return ui.num(path, fallback)
	var dark: bool = ui.is_dark()

	var size := int(n.call("font.size", 14))
	var regular: Font = ui.font()
	var bold: Font = ui.font(int(n.call("font.bold_weight", 800)))
	t.default_font = regular
	t.default_font_size = size
	t.set_font("bold_font", "Fonts", bold)

	var text: Color = c.call("colors.text")
	var muted: Color = c.call("colors.muted")
	var accent: Color = c.call("colors.accent")
	var outline: Color = c.call("colors.text_outline")
	var radius: float = n.call("panel.radius", 10)
	var small_radius := minf(radius * 0.6, 8.0)

	# Window and popup panels.
	var panel := _panel(ui)
	t.set_stylebox("panel", "PanelContainer", panel.duplicate().set_margin_all(12))
	t.set_stylebox("panel", "Panel", panel.duplicate())
	t.set_type_variation("WindowPanel", "PanelContainer")
	t.set_stylebox("panel", "WindowPanel", panel.duplicate().set_margin_all(0))
	t.set_type_variation("HudPanel", "PanelContainer")
	t.set_stylebox("panel", "HudPanel", panel.duplicate().set_margins(10, 8, 10, 8))
	# The chat box: a lighter panel so the world shows through.
	var chat: UiBox = panel.duplicate().set_margins(8, 8, 8, 8)
	chat.colors = _alpha(chat.colors, 0.55)
	chat.shadow_color.a *= 0.5
	t.set_type_variation("ChatPanel", "PanelContainer")
	t.set_stylebox("panel", "ChatPanel", chat)
	# Pop-up questions and the sign-in and creation cards sit on busy backgrounds
	# without the window blur, so they are nearly opaque.
	var question: UiBox = panel.duplicate().set_margins(18, 14, 18, 14)
	question.colors = _alpha(panel.colors, 1.0, 0.95)
	t.set_type_variation("QuestionPanel", "PanelContainer")
	t.set_stylebox("panel", "QuestionPanel", question)
	t.set_type_variation("Clear", "PanelContainer")
	t.set_stylebox("panel", "Clear", StyleBoxEmpty.new())

	var title := UiBoxScript.make(c.call("title.top"), c.call("title.bottom"), radius)
	title.set_margins(10, 0, 6, 0)
	t.set_type_variation("WindowTitle", "PanelContainer")
	t.set_stylebox("panel", "WindowTitle", title)
	t.set_constant("title_height", "WindowTitle", int(n.call("title.height", 32)))
	t.set_color("line", "WindowTitle", c.call("title.line"))

	# Text.
	t.set_color("font_color", "Label", text)
	t.set_color("font_outline_color", "Label", outline)
	t.set_constant("outline_size", "Label", 3 if outline.a > 0.0 else 0)
	t.set_color("font_shadow_color", "Label", Color(0, 0, 0, 0))
	t.set_type_variation("HeaderLabel", "Label")
	t.set_font("font", "HeaderLabel", bold)
	t.set_font_size("font_size", "HeaderLabel", size + 2)
	t.set_color("font_color", "HeaderLabel", c.call("colors.title"))
	t.set_type_variation("MutedLabel", "Label")
	t.set_color("font_color", "MutedLabel", muted)
	t.set_font_size("font_size", "MutedLabel", size - 1)
	t.set_type_variation("SmallLabel", "Label")
	t.set_font_size("font_size", "SmallLabel", size - 2)
	t.set_type_variation("AccentLabel", "Label")
	t.set_color("font_color", "AccentLabel", accent)
	t.set_font("font", "AccentLabel", bold)
	t.set_type_variation("WindowTitleLabel", "Label")
	t.set_font("font", "WindowTitleLabel", bold)
	t.set_font_size("font_size", "WindowTitleLabel", int(n.call("font.title_size", 14)))
	t.set_color("font_color", "WindowTitleLabel", c.call("colors.title"))
	t.set_type_variation("BarLabel", "Label")
	t.set_font("font", "BarLabel", bold)
	t.set_font_size("font_size", "BarLabel", int(round(n.call("bars.font_size", 10))))
	t.set_color("font_color", "BarLabel", c.call("bars.text"))
	t.set_color("font_outline_color", "BarLabel", c.call("bars.text_outline"))
	t.set_constant("outline_size", "BarLabel", 3)
	t.set_type_variation("GoodLabel", "Label")
	t.set_color("font_color", "GoodLabel", c.call("colors.good"))
	t.set_type_variation("BadLabel", "Label")
	t.set_color("font_color", "BadLabel", c.call("colors.bad"))
	t.set_color("default_color", "RichTextLabel", text)
	t.set_color("font_outline_color", "RichTextLabel", outline)
	t.set_constant("outline_size", "RichTextLabel", 3 if outline.a > 0.0 else 0)
	t.set_font("bold_font", "RichTextLabel", bold)
	t.set_stylebox("normal", "RichTextLabel", StyleBoxEmpty.new())
	t.set_stylebox("focus", "RichTextLabel", StyleBoxEmpty.new())

	# Buttons: the default button is the quiet one; ButtonPrimary is the main action.
	var btn_radius := minf(n.call("button.radius", 8), 16.0)
	var b2 := UiBoxScript.make(c.call("button2.top"), c.call("button2.bottom"), btn_radius)
	b2.border_width = 1.0
	b2.border_color = c.call("button2.border_color")
	b2.shadow_color = Color(0, 0, 0, 0.18 if dark else 0.08)
	b2.shadow_size = 3
	b2.shadow_offset = 1
	b2.set_margins(12, 4, 12, 4)
	_button(t, "Button", b2, c.call("button2.text"), muted, c.call("colors.hover"))
	# A toggle button that is on (Male / Female) looks like the selected tab.
	var on := UiBoxScript.make(c.call("tab.top"), c.call("tab.bottom"), btn_radius)
	if str(ui.value_at("tab.style")) != "pill":
		on = UiBoxScript.make(Color(accent, 0.28), Color(accent, 0.12), btn_radius)
		on.border_width = 1.0
		on.border_color = accent
	on.set_margins(12, 4, 12, 4)
	t.set_stylebox("pressed", "Button", on)
	t.set_stylebox("hover_pressed", "Button", on)
	t.set_color("font_pressed_color", "Button", c.call("tab.text_on"))
	t.set_color("font_hover_pressed_color", "Button", c.call("tab.text_on"))
	var b1 := UiBoxScript.make(c.call("button.top"), c.call("button.bottom"), btn_radius)
	b1.colors = PackedColorArray([c.call("button.top"), c.call("button.middle"), c.call("button.bottom")])
	b1.stops = PackedFloat32Array([0.0, 0.5, 1.0])
	b1.border_width = 1.0
	b1.border_color = c.call("button.border_color")
	b1.shadow_color = c.call("button.shadow_color")
	b1.shadow_size = 5
	b1.shadow_offset = 2
	b1.highlight = Color(1, 1, 1, 0.55)
	b1.set_margins(16, 5, 16, 5)
	t.set_type_variation("ButtonPrimary", "Button")
	_button(t, "ButtonPrimary", b1, c.call("button.text"), c.call("button.text"), Color(1, 1, 1, 0.15))
	t.set_font("font", "ButtonPrimary", bold)
	t.set_color("font_outline_color", "ButtonPrimary", c.call("button.text_outline"))
	t.set_constant("outline_size", "ButtonPrimary", 2 if c.call("button.text_outline").a > 0.0 else 0)
	for state in ["normal", "hover", "pressed", "disabled", "focus"]:
		t.set_stylebox(state, "OptionButton", t.get_stylebox(state, "Button"))
	_button_colors(t, "OptionButton", c.call("button2.text"), muted)
	# Flat buttons on window title bars and the menu bar.
	var hover := UiBoxScript.make(c.call("colors.hover"), c.call("colors.hover"), 6)
	hover.set_margins(4, 2, 4, 2)
	var flat := StyleBoxEmpty.new()
	flat.set_content_margin_all(4)
	for flat_type in ["WindowButton", "WindowClose", "FlatButton"]:
		t.set_type_variation(flat_type, "Button")
		t.set_stylebox("normal", flat_type, flat)
		t.set_stylebox("hover", flat_type, hover)
		t.set_stylebox("pressed", flat_type, hover)
		t.set_stylebox("disabled", flat_type, flat)
		t.set_stylebox("focus", flat_type, StyleBoxEmpty.new())
	_button_colors(t, "WindowButton", c.call("colors.window_button"), muted)
	_button_colors(t, "WindowClose", c.call("colors.close"), muted)
	_button_colors(t, "FlatButton", text, muted)
	t.set_font("font", "WindowButton", bold)
	t.set_font("font", "WindowClose", bold)
	t.set_font_size("font_size", "WindowButton", size + 2)
	t.set_font_size("font_size", "WindowClose", size + 2)
	for check in ["CheckBox", "CheckButton"]:
		_button_colors(t, check, text, muted)
		t.set_stylebox("normal", check, flat)
		t.set_stylebox("hover", check, hover)
		t.set_stylebox("pressed", check, flat)
		t.set_stylebox("hover_pressed", check, hover)
		t.set_stylebox("focus", check, StyleBoxEmpty.new())

	# Item and skill slots.
	var slot := UiBoxScript.make(c.call("slot.top"), c.call("slot.bottom"), n.call("slot.radius", 8))
	slot.border_width = 1.0
	slot.border_color = c.call("slot.border_color")
	slot.inner_shadow = c.call("slot.inner_shadow")
	slot.inner_shadow_size = 5.0
	t.set_stylebox("panel", "Panel", slot)
	t.set_type_variation("Slot", "Panel")
	t.set_stylebox("panel", "Slot", slot)
	t.set_color("label", "Slot", c.call("slot.label"))
	t.set_color("key", "Slot", c.call("colors.key"))
	# Text drawn over slot icons: the stack count and the hotbar key. Both get an outline
	# (and the key a chip once the slot is filled) so they read on top of any icon. Themes
	# edited before these values existed get the fallbacks here.
	t.set_type_variation("SlotCount", "Label")
	t.set_font("font", "SlotCount", bold)
	t.set_font_size("font_size", "SlotCount", int(n.call("slot.count_size", 12)))
	t.set_color("font_color", "SlotCount", ui.color("slot.count", Color.WHITE))
	t.set_color("font_outline_color", "SlotCount", ui.color("slot.count_outline", Color(0, 0, 0, 0.9)))
	t.set_constant("outline_size", "SlotCount", int(n.call("slot.count_outline_size", 4)))
	t.set_type_variation("SlotKey", "Label")
	t.set_font("font", "SlotKey", bold)
	t.set_font_size("font_size", "SlotKey", int(n.call("slot.key_size", 10)))
	t.set_color("font_color", "SlotKey", c.call("slot.label"))
	t.set_constant("outline_size", "SlotKey", 0)
	t.set_stylebox("normal", "SlotKey", StyleBoxEmpty.new())
	t.set_type_variation("SlotKeyFilled", "SlotKey")
	t.set_color("font_color", "SlotKeyFilled", c.call("colors.key"))
	t.set_color("font_outline_color", "SlotKeyFilled", ui.color("slot.key_outline", Color(0, 0, 0, 0)))
	t.set_constant("outline_size", "SlotKeyFilled", int(n.call("slot.key_outline_size", 0)))
	var chip := StyleBoxFlat.new()
	chip.bg_color = ui.color("slot.key_background", Color(0, 0, 0, 0.6) if dark else Color(1, 1, 1, 0.88))
	chip.set_corner_radius_all(4)
	chip.set_content_margin_all(0)
	chip.content_margin_left = 3
	chip.content_margin_right = 3
	t.set_stylebox("normal", "SlotKeyFilled", chip)

	# Text fields.
	var field := UiBoxScript.make(c.call("field.background"), c.call("field.background"), small_radius)
	field.border_width = 1.0
	field.border_color = c.call("field.border_color")
	field.inner_shadow = Color(0, 0, 0, 0.25 if dark else 0.06)
	field.set_margins(10, 5, 10, 5)
	var field_focus: UiBox = field.duplicate()
	field_focus.border_color = accent
	for edit in ["LineEdit", "TextEdit"]:
		t.set_stylebox("normal", edit, field)
		t.set_stylebox("focus", edit, field_focus)
		t.set_stylebox("read_only", edit, field)
		t.set_color("font_color", edit, c.call("field.text"))
		t.set_color("font_placeholder_color", edit, muted)
		t.set_color("caret_color", edit, accent)
		t.set_color("selection_color", edit, Color(accent, 0.35))
		t.set_color("font_uneditable_color", edit, muted)
	t.set_stylebox("panel", "ItemList", field.duplicate().set_margin_all(4))
	t.set_stylebox("focus", "ItemList", StyleBoxEmpty.new())
	var selected := UiBoxScript.make(c.call("colors.selected"), c.call("colors.selected"), small_radius)
	var hovered := UiBoxScript.make(c.call("colors.hover"), c.call("colors.hover"), small_radius)
	t.set_stylebox("selected", "ItemList", selected)
	t.set_stylebox("selected_focus", "ItemList", selected)
	t.set_stylebox("hovered", "ItemList", hovered)
	t.set_stylebox("hovered_selected", "ItemList", selected)
	t.set_stylebox("hovered_selected_focus", "ItemList", selected)
	t.set_color("font_color", "ItemList", text)
	t.set_color("font_selected_color", "ItemList", text)
	t.set_color("font_hovered_color", "ItemList", text)
	t.set_color("guide_color", "ItemList", Color(0, 0, 0, 0))

	# Tabs: pills or an underline.
	var pill: bool = str(ui.value_at("tab.style")) == "pill"
	var tab_on := UiBoxScript.make(c.call("tab.top"), c.call("tab.bottom"), 14.0 if pill else 0.0)
	if pill:
		tab_on.shadow_color = Color(0, 0, 0, 0.18 if dark else 0.12)
		tab_on.shadow_size = 3
		tab_on.shadow_offset = 1
		tab_on.highlight = Color(1, 1, 1, 0.45)
		tab_on.set_margins(12, 3, 12, 3)
	else:
		tab_on.border_width = 2.0
		tab_on.border_color = c.call("tab.underline")
		tab_on.border_sides = [false, false, false, true]
		tab_on.set_margins(4, 3, 4, 5)
	var tab_off := StyleBoxEmpty.new()
	tab_off.content_margin_left = tab_on.content_margin_left
	tab_off.content_margin_right = tab_on.content_margin_right
	tab_off.content_margin_top = tab_on.content_margin_top
	tab_off.content_margin_bottom = tab_on.content_margin_bottom
	var tab_hover: UiBox = hovered.duplicate()
	tab_hover.radius = 14.0 if pill else 4.0
	tab_hover.set_margins(tab_on.content_margin_left, tab_on.content_margin_top, tab_on.content_margin_right, tab_on.content_margin_bottom)
	for tabs in ["TabBar", "TabContainer"]:
		t.set_stylebox("tab_selected", tabs, tab_on)
		t.set_stylebox("tab_unselected", tabs, tab_off)
		t.set_stylebox("tab_hovered", tabs, tab_hover)
		t.set_stylebox("tab_disabled", tabs, tab_off)
		t.set_stylebox("tab_focus", tabs, StyleBoxEmpty.new())
		t.set_color("font_selected_color", tabs, c.call("tab.text_on"))
		t.set_color("font_unselected_color", tabs, muted)
		t.set_color("font_hovered_color", tabs, text)
		t.set_color("font_disabled_color", tabs, Color(muted, 0.5))
		t.set_font("font", tabs, bold)
		t.set_constant("h_separation", tabs, 4 if pill else 14)
	t.set_stylebox("panel", "TabContainer", StyleBoxEmpty.new())
	t.set_stylebox("tabbar_background", "TabContainer", StyleBoxEmpty.new())

	# Bars.
	var bar_radius: float = n.call("bars.radius", 5)
	var back := UiBoxScript.make(c.call("bars.back_top"), c.call("bars.back_bottom"), bar_radius)
	back.inner_shadow = Color(0, 0, 0, 0.45 if dark else 0.18)
	back.inner_shadow_size = 3.0
	var fill := UiBoxScript.make(c.call("bars.xp"), c.call("bars.xp"), bar_radius)
	fill.sheen = n.call("bars.sheen", 0.45)
	t.set_stylebox("background", "ProgressBar", back)
	t.set_stylebox("fill", "ProgressBar", fill)
	t.set_color("font_color", "ProgressBar", c.call("bars.text"))
	t.set_color("font_outline_color", "ProgressBar", c.call("bars.text_outline"))
	t.set_constant("outline_size", "ProgressBar", 3)
	t.set_font_size("font_size", "ProgressBar", int(round(n.call("bars.font_size", 10))))
	t.set_font("font", "ProgressBar", bold)
	var track: UiBox = back.duplicate()
	track.content_margin_top = 3
	track.content_margin_bottom = 3
	t.set_stylebox("slider", "HSlider", track)
	var track_fill := UiBoxScript.make(accent.lightened(0.2), accent, bar_radius)
	track_fill.content_margin_top = 3
	track_fill.content_margin_bottom = 3
	t.set_stylebox("grabber_area", "HSlider", track_fill)
	t.set_stylebox("grabber_area_highlight", "HSlider", track_fill)

	# Scroll bars: thin and quiet.
	for bar in ["VScrollBar", "HScrollBar"]:
		var gutter := StyleBoxEmpty.new()
		gutter.set_content_margin_all(3)
		var grab := UiBoxScript.make(Color(muted, 0.45), Color(muted, 0.45), 4)
		grab.set_margin_all(3)
		var grab_on := UiBoxScript.make(Color(muted, 0.7), Color(muted, 0.7), 4)
		t.set_stylebox("scroll", bar, gutter)
		t.set_stylebox("scroll_focus", bar, gutter)
		t.set_stylebox("grabber", bar, grab)
		t.set_stylebox("grabber_highlight", bar, grab_on)
		t.set_stylebox("grabber_pressed", bar, grab_on)

	var line := StyleBoxLine.new()
	line.color = c.call("colors.separator")
	line.thickness = 1
	t.set_stylebox("separator", "HSeparator", line)
	t.set_constant("separation", "HSeparator", 9)
	var vline := StyleBoxLine.new()
	vline.color = line.color
	vline.vertical = true
	t.set_stylebox("separator", "VSeparator", vline)

	# Tooltips and menus.
	# Tooltips stay readable over anything: their background alpha is the theme's
	# tooltip.opacity (1 = solid), whatever the window opacity setting or the colours' alpha.
	var tip_opacity := clampf(n.call("tooltip.opacity", 1.0), 0.0, 1.0)
	var tip := UiBoxScript.make(Color(c.call("tooltip.top"), tip_opacity), Color(c.call("tooltip.bottom"), tip_opacity), small_radius)
	tip.border_width = 1.0
	tip.border_color = c.call("tooltip.border_color")
	tip.shadow_color = Color(0, 0, 0, 0.35 if dark else 0.18)
	tip.shadow_size = 8
	tip.shadow_offset = 3
	tip.set_margins(10, 6, 10, 6)
	t.set_stylebox("panel", "TooltipPanel", tip)
	t.set_color("font_color", "TooltipLabel", c.call("tooltip.text"))
	t.set_color("font_outline_color", "TooltipLabel", Color(0, 0, 0, 0))
	var menu_panel: UiBox = tip.duplicate()
	menu_panel.set_margins(6, 6, 6, 6)
	t.set_stylebox("panel", "PopupMenu", menu_panel)
	t.set_stylebox("hover", "PopupMenu", selected)
	t.set_color("font_color", "PopupMenu", c.call("tooltip.text"))
	t.set_color("font_hover_color", "PopupMenu", c.call("tooltip.text"))
	t.set_color("font_disabled_color", "PopupMenu", muted)
	t.set_constant("v_separation", "PopupMenu", 8)
	t.set_stylebox("panel", "PopupPanel", menu_panel)
	t.set_type_variation("Tooltip", "PanelContainer")
	t.set_stylebox("panel", "Tooltip", tip)

	# Colours the HUD and windows read from the theme.
	for key in ["text", "muted", "accent", "good", "bad", "rare", "unique", "zuly", "enemy", "friendly", "separator", "hover", "grip"]:
		t.set_color(key, "Ui", c.call("colors." + key))
	for key in ["hp", "hp_lag", "mp", "stamina", "xp", "cast"]:
		t.set_color(key, "Bars", c.call("bars." + key))
	return t


## The window panel: gradient, border, drop shadow and the top highlight.
static func _panel(ui: Node) -> UiBox:
	var box := UiBoxScript.make(ui.color("panel.top"), ui.color("panel.bottom"), ui.num("panel.radius", 10))
	box.colors = PackedColorArray([ui.color("panel.top"), ui.color("panel.middle"), ui.color("panel.bottom")])
	box.stops = PackedFloat32Array([0.0, 0.5, 1.0])
	box.colors = _alpha(box.colors, clampf(float(ui.settings.get("opacity", 1.0)), 0.3, 1.0))
	box.border_width = ui.num("panel.border_width", 1)
	box.border_color = ui.color("panel.border_color")
	box.shadow_color = ui.color("panel.shadow_color")
	box.shadow_size = ui.num("panel.shadow_size", 12)
	box.shadow_offset = ui.num("panel.shadow_offset", 5)
	box.highlight = ui.color("panel.highlight")
	return box


## A copy of the colours with their alpha multiplied by `factor`, then raised to at least
## `at_least`. Always a new array: a duplicated UiBox shares its colours with the original,
## so changing them in place (colors[i].a = ...) changed every panel at once.
static func _alpha(cols: PackedColorArray, factor: float, at_least := 0.0) -> PackedColorArray:
	var out := PackedColorArray()
	for col in cols:
		out.append(Color(col, maxf(col.a * factor, at_least)))
	return out


static func _button(t: Theme, type: String, normal: UiBox, font: Color, disabled_font: Color, hover_tint: Color) -> void:
	var hover: UiBox = normal.duplicate()
	hover.colors = PackedColorArray()
	for col in normal.colors:
		hover.colors.append(col.blend(Color(hover_tint, hover_tint.a + 0.06)) if col.a > 0.05 else hover_tint)
	var pressed: UiBox = normal.duplicate()
	pressed.colors = PackedColorArray()
	for i in normal.colors.size():
		pressed.colors.append(normal.colors[normal.colors.size() - 1 - i].darkened(0.08))
	pressed.shadow_size = 1
	pressed.highlight = Color(0, 0, 0, 0)
	var disabled: UiBox = normal.duplicate()
	disabled.colors = PackedColorArray()
	for col in normal.colors:
		disabled.colors.append(Color(col, col.a * 0.45))
	disabled.shadow_size = 0
	t.set_stylebox("normal", type, normal)
	t.set_stylebox("hover", type, hover)
	t.set_stylebox("pressed", type, pressed)
	t.set_stylebox("hover_pressed", type, pressed)
	t.set_stylebox("disabled", type, disabled)
	t.set_stylebox("focus", type, StyleBoxEmpty.new())
	_button_colors(t, type, font, disabled_font)


static func _button_colors(t: Theme, type: String, font: Color, disabled_font: Color) -> void:
	for key in ["font_color", "font_hover_color", "font_pressed_color", "font_hover_pressed_color", "font_focus_color"]:
		t.set_color(key, type, font)
	t.set_color("font_disabled_color", type, Color(disabled_font, 0.6))
