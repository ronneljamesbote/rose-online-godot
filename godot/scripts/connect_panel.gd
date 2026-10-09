## Start screen: sign in with the email and password of an account made on the website,
## or play offline. The server, website and email are kept in user://settings.cfg; the
## password is never saved.
extends CanvasLayer

## token is the website's game token for the account.
signal connect_requested(uri: String, name_text: String, use_bow: bool, token: String)
signal offline_requested

const SETTINGS := "user://settings.cfg"
const AccountLogin := preload("res://scripts/account_login.gd")

var server_edit: LineEdit
var website_edit: LineEdit
var email_edit: LineEdit
var password_edit: LineEdit
var error_label: Label
var sign_in_button: Button
var _login: Node


func _ready() -> void:
	var settings := ConfigFile.new()
	settings.load(SETTINGS)
	_login = AccountLogin.new()
	add_child(_login)

	var center := CenterContainer.new()
	center.set_anchors_preset(Control.PRESET_FULL_RECT)
	add_child(center)
	var panel := PanelContainer.new()
	panel.theme_type_variation = "QuestionPanel"
	panel.custom_minimum_size = Vector2(420, 0)
	center.add_child(panel)
	var margin := MarginContainer.new()
	for side in ["left", "right", "top", "bottom"]:
		margin.add_theme_constant_override("margin_" + side, 18)
	panel.add_child(margin)
	var box := VBoxContainer.new()
	box.add_theme_constant_override("separation", 8)
	margin.add_child(box)

	var title := Label.new()
	title.text = "ROSE"
	title.horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER
	title.theme_type_variation = "HeaderLabel"
	title.add_theme_font_size_override("font_size", 40)
	box.add_child(title)

	email_edit = _field(box, "Email", settings.get_value("net", "email", ""))
	email_edit.placeholder_text = "you@example.com"
	password_edit = _field(box, "Password", "")
	password_edit.secret = true
	server_edit = _field(box, "Server", settings.get_value("net", "server", "ws://127.0.0.1:3000"))
	website_edit = _field(box, "Website", settings.get_value("net", "website", "http://127.0.0.1:3001"))

	error_label = Label.new()
	error_label.theme_type_variation = "BadLabel"
	error_label.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
	error_label.custom_minimum_size.x = 380
	error_label.visible = false
	box.add_child(error_label)

	var buttons := HBoxContainer.new()
	buttons.alignment = BoxContainer.ALIGNMENT_CENTER
	buttons.add_theme_constant_override("separation", 12)
	box.add_child(buttons)
	sign_in_button = Button.new()
	sign_in_button.text = "Sign in"
	sign_in_button.theme_type_variation = "ButtonPrimary"
	sign_in_button.custom_minimum_size.x = 120
	sign_in_button.pressed.connect(_on_sign_in)
	buttons.add_child(sign_in_button)
	var offline_button := Button.new()
	offline_button.text = "Play offline"
	offline_button.pressed.connect(func(): offline_requested.emit(); queue_free())
	buttons.add_child(offline_button)

	# The website handles new accounts and forgotten passwords.
	var links := HBoxContainer.new()
	links.alignment = BoxContainer.ALIGNMENT_CENTER
	links.add_theme_constant_override("separation", 16)
	box.add_child(links)
	for link in [["Create an account", "/signup"], ["Forgot password?", "/forgot-password"]]:
		var b := LinkButton.new()
		b.text = link[0]
		b.pressed.connect(func(): OS.shell_open(website_edit.text.strip_edges().trim_suffix("/") + link[1]))
		links.add_child(b)

	password_edit.text_submitted.connect(func(_t): _on_sign_in())
	email_edit.text_submitted.connect(func(_t): password_edit.grab_focus())
	if email_edit.text == "":
		email_edit.grab_focus()
	else:
		password_edit.grab_focus()


func _field(box: VBoxContainer, caption: String, value: String) -> LineEdit:
	var row := HBoxContainer.new()
	box.add_child(row)
	var label := Label.new()
	label.text = caption
	label.custom_minimum_size.x = 80
	row.add_child(label)
	var edit := LineEdit.new()
	edit.text = value
	edit.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	row.add_child(edit)
	return edit


func show_error(text: String) -> void:
	error_label.text = text
	error_label.visible = text != ""


func _on_sign_in() -> void:
	var email := email_edit.text.strip_edges()
	var server := server_edit.text.strip_edges()
	var website := website_edit.text.strip_edges()
	var settings := ConfigFile.new()
	settings.load(SETTINGS)
	settings.set_value("net", "server", server)
	settings.set_value("net", "website", website)
	settings.set_value("net", "email", email)
	settings.save(SETTINGS)
	if email == "" or password_edit.text == "":
		show_error("Enter your email and password. No account yet? Create one on the website.")
		return
	sign_in_button.disabled = true
	show_error("")
	var answer: Dictionary = await _login.login(website, email, password_edit.text)
	sign_in_button.disabled = false
	if answer.has("error"):
		show_error(answer["error"])
		return
	password_edit.text = ""
	connect_requested.emit(server, "", false, answer["token"])
	queue_free()
