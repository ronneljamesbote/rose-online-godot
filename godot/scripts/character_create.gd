## Character creation, shown after signing in to an account that has no character yet:
## a name, male or female, and one of the iROSE faces and hair styles, with a turning
## preview of the character.
extends CanvasLayer

## Faces and hair styles of the iROSE creation screen (same lists as the server's).
const FACES := [1, 8, 15, 22, 29, 36, 43]
const HAIRS := [0, 5, 10, 15, 20]

var net: RoseNet
var name_edit: LineEdit
var male_button: Button
var female_button: Button
var face_label: Label
var hair_label: Label
var error_label: Label
var create_button: Button
var male := true
var face_index := 0
var hair_index := 0
var _preview_root: Node3D
var _model: Node3D
var _waiting := false
var auto_create_after := -1.0  # --create-after=SECONDS presses Create by itself (tests)


func _ready() -> void:
	# The whole screen behind the creation panel, in the theme's colours.
	var backdrop := TextureRect.new()
	backdrop.set_anchors_preset(Control.PRESET_FULL_RECT)
	backdrop.texture = RoseData.texture("3DDATA/CONTROL/RES/LOADING.DDS")
	backdrop.expand_mode = TextureRect.EXPAND_IGNORE_SIZE
	backdrop.stretch_mode = TextureRect.STRETCH_KEEP_ASPECT_COVERED
	backdrop.modulate = Color(0.45, 0.45, 0.55)
	add_child(backdrop)
	var root := HBoxContainer.new()
	root.set_anchors_and_offsets_preset(Control.PRESET_CENTER)
	root.grow_horizontal = Control.GROW_DIRECTION_BOTH
	root.grow_vertical = Control.GROW_DIRECTION_BOTH
	root.add_theme_constant_override("separation", 16)
	add_child(root)

	var panel := PanelContainer.new()
	panel.theme_type_variation = "QuestionPanel"
	panel.custom_minimum_size = Vector2(340, 460)
	root.add_child(panel)
	var margin := MarginContainer.new()
	for side in ["left", "right", "top", "bottom"]:
		margin.add_theme_constant_override("margin_" + side, 18)
	panel.add_child(margin)
	var box := VBoxContainer.new()
	box.add_theme_constant_override("separation", 10)
	margin.add_child(box)

	var title := Label.new()
	title.text = "Create your character"
	title.theme_type_variation = "HeaderLabel"
	title.add_theme_font_size_override("font_size", 22)
	box.add_child(title)

	box.add_child(_caption("Name"))
	name_edit = LineEdit.new()
	name_edit.max_length = 16
	name_edit.placeholder_text = "3-16 letters and digits"
	name_edit.text_submitted.connect(func(_t): _create())
	box.add_child(name_edit)

	box.add_child(_caption("Body"))
	var genders := HBoxContainer.new()
	genders.add_theme_constant_override("separation", 8)
	box.add_child(genders)
	var group := ButtonGroup.new()
	male_button = _toggle(genders, "Male", group, true)
	female_button = _toggle(genders, "Female", group, false)
	male_button.pressed.connect(func(): male = true; _rebuild())
	female_button.pressed.connect(func(): male = false; _rebuild())

	face_label = _chooser(box, "Face", func(step): face_index = posmod(face_index + step, FACES.size()); _rebuild())
	hair_label = _chooser(box, "Hair", func(step): hair_index = posmod(hair_index + step, HAIRS.size()); _rebuild())

	var spacer := Control.new()
	spacer.size_flags_vertical = Control.SIZE_EXPAND_FILL
	box.add_child(spacer)
	error_label = Label.new()
	error_label.theme_type_variation = "BadLabel"
	error_label.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
	error_label.custom_minimum_size.x = 300
	box.add_child(error_label)
	create_button = Button.new()
	create_button.text = "Create character"
	create_button.theme_type_variation = "ButtonPrimary"
	create_button.pressed.connect(_create)
	box.add_child(create_button)

	# The preview: the character on its own, slowly turning.
	var view_container := SubViewportContainer.new()
	view_container.custom_minimum_size = Vector2(380, 500)
	view_container.stretch = true
	root.add_child(view_container)
	var viewport := SubViewport.new()
	viewport.own_world_3d = true
	viewport.transparent_bg = false
	view_container.add_child(viewport)
	var environment := Environment.new()
	environment.background_mode = Environment.BG_COLOR
	environment.background_color = Color(UI.color("panel.bottom").darkened(0.45), 1.0)
	environment.ambient_light_source = Environment.AMBIENT_SOURCE_COLOR
	environment.ambient_light_color = Color(0.7, 0.7, 0.75)
	var world_environment := WorldEnvironment.new()
	world_environment.environment = environment
	viewport.add_child(world_environment)
	var light := DirectionalLight3D.new()
	light.rotation_degrees = Vector3(-35, 30, 0)
	viewport.add_child(light)
	var camera := Camera3D.new()
	camera.position = Vector3(0, 1.05, 2.6)
	camera.rotation_degrees = Vector3(-6, 0, 0)
	camera.fov = 40
	viewport.add_child(camera)
	_preview_root = Node3D.new()
	viewport.add_child(_preview_root)
	_rebuild()
	name_edit.grab_focus()


func _caption(text: String) -> Label:
	var label := Label.new()
	label.text = text
	label.theme_type_variation = "MutedLabel"
	return label


func _toggle(parent: Control, text: String, group: ButtonGroup, pressed: bool) -> Button:
	var b := Button.new()
	b.text = text
	b.toggle_mode = true
	b.button_group = group
	b.button_pressed = pressed
	b.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	parent.add_child(b)
	return b


## A "<  Face 1 of 7  >" row; step is called with -1 or 1.
func _chooser(box: VBoxContainer, caption: String, step: Callable) -> Label:
	var row := HBoxContainer.new()
	box.add_child(row)
	var left := Button.new()
	left.text = "<"
	left.pressed.connect(func(): step.call(-1))
	row.add_child(left)
	var label := Label.new()
	label.horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER
	label.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	label.set_meta("caption", caption)
	row.add_child(label)
	var right := Button.new()
	right.text = ">"
	right.pressed.connect(func(): step.call(1))
	row.add_child(right)
	return label


func _rebuild() -> void:
	face_label.text = "Face %d of %d" % [face_index + 1, FACES.size()]
	hair_label.text = "Hair %d of %d" % [hair_index + 1, HAIRS.size()]
	if _model:
		_model.queue_free()
	_model = RoseCharacter.new()
	# In the clothes and with the sword a new character starts with.
	var look: PackedInt32Array = RoseData.starting_look(not male)
	_model.build(male, FACES[face_index], HAIRS[hair_index], look[0], look[1], look[2], look[3], look[4], 0)
	_preview_root.add_child(_model)
	var anim: AnimationPlayer = _model.get_node_or_null("AnimationPlayer")
	if anim and anim.has_animation("stop1"):
		anim.play("stop1")


## Fill the form from the command line (--create=NAME,female,FACE,HAIR, 1-based).
func preset(text: String) -> void:
	var parts := text.split(",")
	name_edit.text = parts[0]
	if parts.size() > 1 and parts[1] == "female":
		male = false
		female_button.button_pressed = true
	if parts.size() > 2:
		face_index = clampi(int(parts[2]) - 1, 0, FACES.size() - 1)
	if parts.size() > 3:
		hair_index = clampi(int(parts[3]) - 1, 0, HAIRS.size() - 1)
	_rebuild()


func _create() -> void:
	if _waiting:
		return
	_waiting = true
	create_button.disabled = true
	error_label.text = ""
	net.create_character(name_edit.text.strip_edges(), not male, FACES[face_index], HAIRS[hair_index])


func _process(delta: float) -> void:
	if _preview_root:
		_preview_root.rotate_y(delta * 0.5)
	if auto_create_after >= 0.0:
		auto_create_after -= delta
		if auto_create_after < 0.0:
			_create()
	if not _waiting:
		return
	var answer: String = net.poll_create_result()
	if answer == "":
		return
	_waiting = false
	create_button.disabled = false
	if answer != "ok":
		error_label.text = answer.substr(0, 1).to_upper() + answer.substr(1)
