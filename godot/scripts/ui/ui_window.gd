## A window frame: title bar (drag to move), minimise and close buttons, a corner grip to
## scale it, and the blurred, themed panel. The window's own content goes inside; the frame
## shows and hides with it, so content scripts keep using `visible` as before.
class_name UiWindow
extends UiMovable

const BlurShader := preload("res://shaders/ui_blur.gdshader")

var content: Control
var title_text := ""
var closable := true
## Whether the window opens again next session when it was open at exit.
var remember_open := false
## Method on the content that closes it properly (close_store ...); "" hides it.
var close_method := ""
## Take over the content's own header: a static one is hidden, a changing one (a store's
## name) becomes the title, and its Close button goes (the frame has one).
var adopt_header := true
var _title_source: Label
var blur: ColorRect
var panel: PanelContainer
var title_bar: PanelContainer
var title_label: Label
var title_line: ColorRect
var body: MarginContainer
var min_button: Button
var close_button: Button
var grip: Control
var extra: HBoxContainer  # title bar room for window-specific buttons
var _minimised := false
var _scaling := false
var _scale_from := 1.0
var _scale_mouse := Vector2.ZERO


static func wrap(what: Control, window_id: String, window_title: String, anchor := Vector2(0.5, 0.5), offset := Vector2.ZERO) -> UiWindow:
	var w := UiWindow.new()
	w.id = window_id
	w.title_text = window_title
	w.default_anchor = anchor
	w.default_offset = offset
	w.content = what
	w.name = window_id.to_pascal_case() + "Frame"
	return w


func _init() -> void:
	super._init()
	mouse_filter = Control.MOUSE_FILTER_STOP


func _ready() -> void:
	blur = ColorRect.new()
	blur.mouse_filter = Control.MOUSE_FILTER_IGNORE
	blur.material = ShaderMaterial.new()
	blur.material.shader = BlurShader
	blur.set_meta("ui_fill", true)
	add_child(blur)

	panel = PanelContainer.new()
	panel.theme_type_variation = "WindowPanel"
	panel.set_meta("ui_fill", true)
	panel.mouse_filter = Control.MOUSE_FILTER_STOP
	panel.gui_input.connect(_on_panel_input)
	add_child(panel)
	var column := VBoxContainer.new()
	column.add_theme_constant_override("separation", 0)
	panel.add_child(column)

	title_bar = PanelContainer.new()
	title_bar.theme_type_variation = "WindowTitle"
	title_bar.mouse_filter = Control.MOUSE_FILTER_STOP
	title_bar.mouse_default_cursor_shape = Control.CURSOR_MOVE
	title_bar.gui_input.connect(_on_title_input)
	column.add_child(title_bar)
	var row := HBoxContainer.new()
	row.add_theme_constant_override("separation", 6)
	title_bar.add_child(row)
	var dots := _Grip.new()
	dots.mouse_filter = Control.MOUSE_FILTER_IGNORE
	row.add_child(dots)
	title_label = Label.new()
	title_label.theme_type_variation = "WindowTitleLabel"
	title_label.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	title_label.vertical_alignment = VERTICAL_ALIGNMENT_CENTER
	title_label.mouse_filter = Control.MOUSE_FILTER_IGNORE
	row.add_child(title_label)
	extra = HBoxContainer.new()
	row.add_child(extra)
	min_button = _title_button("–", "WindowButton", "Minimise")
	min_button.pressed.connect(toggle_minimised)
	row.add_child(min_button)
	close_button = _title_button("×", "WindowClose", "Close (Esc)")
	close_button.pressed.connect(close)
	close_button.visible = closable
	row.add_child(close_button)

	title_line = ColorRect.new()
	title_line.custom_minimum_size.y = 1
	title_line.mouse_filter = Control.MOUSE_FILTER_IGNORE
	column.add_child(title_line)

	body = MarginContainer.new()
	for side in ["left", "right", "bottom"]:
		body.add_theme_constant_override("margin_" + side, 12)
	body.add_theme_constant_override("margin_top", 10)
	column.add_child(body)
	if content:
		if content.get_parent():
			content.get_parent().remove_child(content)
		# The frame draws the panel now; the content keeps only its layout.
		if content is PanelContainer:
			content.theme_type_variation = "Clear"
		content.set_anchors_preset(Control.PRESET_TOP_LEFT)
		content.position = Vector2.ZERO
		body.add_child(content)
		if adopt_header:
			_adopt(content, 0)
		visible = content.visible
		content.visibility_changed.connect(_on_content_visibility)

	grip = _Corner.new()
	grip.mouse_default_cursor_shape = Control.CURSOR_FDIAGSIZE
	grip.gui_input.connect(_on_grip_input)
	add_child(grip)

	UI.theme_changed.connect(_restyle)
	UI.settings_changed.connect(_restyle)
	_restyle()
	super._ready()


func _adopt(node: Node, depth: int) -> void:
	if depth == 0 and node.get_child_count() > 0 and node.get_child(0) is MarginContainer:
		for side in ["left", "right", "top", "bottom"]:
			node.get_child(0).add_theme_constant_override("margin_" + side, 0)
	for child in node.get_children():
		if child is Label and child.theme_type_variation == "HeaderLabel" and _title_source == null:
			if child.text == "":
				_title_source = child
				child.visible = false
			elif child.text.to_lower() == title_text.to_lower():
				child.visible = false
		elif child is Button and child.text == "Close" and depth <= 3:
			child.visible = false
		elif depth < 3 and (child is Container):
			_adopt(child, depth + 1)


func _process(_delta: float) -> void:
	if _title_source and visible:
		var text := _title_source.text.split("\n")[0].strip_edges()
		if text != "" and text != title_text:
			set_title(text)


func _title_button(text: String, type: String, tip: String) -> Button:
	var b := Button.new()
	b.text = text
	b.theme_type_variation = type
	b.focus_mode = Control.FOCUS_NONE
	b.tooltip_text = tip
	b.custom_minimum_size = Vector2(24, 22)
	return b


func set_title(text: String) -> void:
	title_text = text
	_restyle()


func _restyle() -> void:
	if title_label == null:
		return
	var upper: bool = UI.value_at("font.title_upper") == true
	title_label.text = title_text.to_upper() if upper else title_text
	var spacing := UI.num("font.title_spacing", 0)
	if spacing != 0.0:
		var f := FontVariation.new()
		f.base_font = UI.font(int(UI.num("font.bold_weight", 800)))
		f.spacing_glyph = int(spacing)
		title_label.add_theme_font_override("font", f)
	else:
		title_label.remove_theme_font_override("font")
	title_bar.custom_minimum_size.y = UI.num("title.height", 32)
	title_line.color = UI.color("title.line")
	var blur_px := UI.num("panel.blur", 0)
	blur.visible = UI.settings.get("blur", true) and blur_px > 0.0
	var mat: ShaderMaterial = blur.material
	mat.set_shader_parameter("lod", clampf(log(maxf(blur_px, 1.0)) / log(2.0) - 0.5, 0.0, 5.0))
	mat.set_shader_parameter("radius", UI.num("panel.radius", 10))
	_sort_extra()


func _sort_extra() -> void:
	if blur and blur.material:
		(blur.material as ShaderMaterial).set_shader_parameter("size", size)
	if grip:
		grip.size = Vector2(14, 14)
		grip.position = size - grip.size - Vector2(2, 2)
		grip.visible = not _minimised


func _on_content_visibility() -> void:
	if visible != content.visible:
		visible = content.visible
		if visible:
			move_to_front()
			_place_from_anchor()
	if remember_open and id != "":
		var place := UI.layout_get(id)
		place["open"] = content.visible
		UI.layout_set(id, place)


## Opens it again next session if it was open at exit.
func restore_open() -> void:
	if remember_open and bool(UI.layout_get(id).get("open", false)):
		content.visible = true


func close() -> void:
	if content and close_method != "" and content.has_method(close_method):
		content.call(close_method)
	elif content:
		content.visible = false
	else:
		visible = false


func toggle_minimised() -> void:
	_minimised = not _minimised
	body.visible = not _minimised
	title_line.visible = not _minimised
	min_button.text = "+" if _minimised else "–"
	save_place()


func _after_load(place: Dictionary) -> void:
	_minimised = bool(place.get("minimised", false))
	if body:
		body.visible = not _minimised
		title_line.visible = not _minimised
		min_button.text = "+" if _minimised else "–"


func _before_save(place: Dictionary) -> void:
	place["minimised"] = _minimised


func _on_title_input(event: InputEvent) -> void:
	if event is InputEventMouseButton and event.button_index == MOUSE_BUTTON_LEFT:
		if event.pressed and event.double_click:
			toggle_minimised()
		elif event.pressed:
			move_to_front()
			start_drag()
		accept_event()


func _on_panel_input(event: InputEvent) -> void:
	if event is InputEventMouseButton and event.pressed:
		move_to_front()


func _on_grip_input(event: InputEvent) -> void:
	if UI.settings.get("locked", false):
		return
	if event is InputEventMouseButton and event.button_index == MOUSE_BUTTON_LEFT:
		_scaling = event.pressed
		_scale_from = scale.x
		_scale_mouse = get_viewport().get_mouse_position()
		if not event.pressed:
			save_place()
		grip.accept_event()
	elif event is InputEventMouseMotion and _scaling:
		var moved := get_viewport().get_mouse_position() - _scale_mouse
		var grow := (moved.x + moved.y) * 0.5 / maxf(size.x, 100.0)
		scale = Vector2.ONE * clampf(snappedf(_scale_from + grow, 0.05), min_scale, max_scale)
		_keep_on_screen()
		grip.accept_event()


## Six dots on the title bar that show it can be dragged.
class _Grip extends Control:
	func _init() -> void:
		custom_minimum_size = Vector2(8, 14)

	func _draw() -> void:
		var c := get_theme_color("grip", "Ui")
		for row in 3:
			for col in 2:
				draw_circle(Vector2(1.5 + col * 4.0, size.y * 0.5 - 4.0 + row * 4.0), 1.1, c, true, -1.0, true)


## The corner grip that scales the window.
class _Corner extends Control:
	func _draw() -> void:
		var c := get_theme_color("grip", "Ui")
		for i in 3:
			var d := 4.0 + i * 4.0
			draw_line(Vector2(size.x - d, size.y - 1), Vector2(size.x - 1, size.y - d), c, 1.2, true)
