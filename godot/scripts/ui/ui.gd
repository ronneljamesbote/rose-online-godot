## The interface: themes, settings and window layout. Autoloaded as UI.
##
## Themes are TOML files in the "themes" folder next to the game (next to the project
## folder while developing). The four built-in themes are copied there on first start so
## players can edit them or make their own; see ui/themes/arumic.toml.
extends Node

signal theme_changed
signal settings_changed

const BUILTIN_DIR := "res://ui/themes"
const BUILTIN := ["arumic", "twilight", "junon-air", "clear"]
const DEFAULT_THEME := "arumic"
const SETTINGS := "user://settings.cfg"
const LAYOUT := "user://ui_layout.cfg"
const WRITTEN := "user://themes_written.cfg"  # checksums of the built-in theme files we wrote
const FONTS := {
	"nunito": "res://ui/fonts/Nunito.ttf",
	"baloo 2": "res://ui/fonts/Baloo2.ttf",
	"baloo2": "res://ui/fonts/Baloo2.ttf",
	"quicksand": "res://ui/fonts/Quicksand.ttf",
}

## Player settings for the interface, saved in the [ui] section of settings.cfg.
var settings := {
	"theme": DEFAULT_THEME,
	"scale": 1.0,  # interface size, 0.75 to 1.5
	"opacity": 1.0,  # window opacity
	"blur": true,  # blur the world behind windows
	"locked": false,  # windows and widgets can't be dragged
	"own_tag": true,  # our own name tag
	"name_tags": true,  # name tags over other characters
	"item_labels": "always",  # always, or alt (only while Alt is held)
	"bubbles": true,  # chat bubbles
	"damage_numbers": true,
}

## id -> {"id", "name", "description", "file", "tokens", "errors"}
var themes := {}
## The theme in use: its merged values, as nested dictionaries (tokens.colors.text ...).
var tokens := {}
var theme: Theme
var _fonts := {}  # path -> FontFile
var _layout := ConfigFile.new()
var _layout_dirty := false
var _save_at := 0


func _ready() -> void:
	var cfg := ConfigFile.new()
	cfg.load(SETTINGS)
	for key in settings:
		settings[key] = cfg.get_value("ui", key, settings[key])
	settings["scale"] = clampf(float(settings["scale"]), 0.75, 1.5)
	_layout.load(LAYOUT)
	reload_themes()


func _process(_delta: float) -> void:
	if _layout_dirty and Time.get_ticks_msec() >= _save_at:
		_layout_dirty = false
		_layout.save(LAYOUT)


## Where players keep theme files: "themes" next to the game's executable, or next to the
## project folder when running from the editor or a source checkout.
func themes_dir() -> String:
	if OS.has_feature("template"):
		return OS.get_executable_path().get_base_dir().path_join("themes")
	return ProjectSettings.globalize_path("res://").path_join("themes")


## Reads every theme file again and rebuilds the look. Copies missing built-in themes
## into the themes folder first.
func reload_themes() -> void:
	var dir := themes_dir()
	DirAccess.make_dir_recursive_absolute(dir)
	# A built-in theme file the player never changed is updated with the game; one they
	# edited is left alone.
	var written := ConfigFile.new()
	written.load(WRITTEN)
	for id in BUILTIN:
		var target := dir.path_join(id + ".toml")
		var text := FileAccess.get_file_as_string(BUILTIN_DIR.path_join(id + ".toml"))
		var current := FileAccess.get_file_as_string(target) if FileAccess.file_exists(target) else ""
		var untouched: bool = current == "" or current.md5_text() == written.get_value("md5", id, "")
		if untouched and current != text:
			var f := FileAccess.open(target, FileAccess.WRITE)
			if f:
				f.store_string(text)
				written.set_value("md5", id, text.md5_text())
	written.save(WRITTEN)
	var raw := {}
	# Built-in copies first, so a theme can always fall back on them.
	for id in BUILTIN:
		raw[id] = _read_toml(BUILTIN_DIR.path_join(id + ".toml"))
		raw[id]["__file"] = ""
	var files := DirAccess.get_files_at(dir) if DirAccess.dir_exists_absolute(dir) else PackedStringArray()
	for file in files:
		if file.get_extension().to_lower() != "toml":
			continue
		var id := file.get_basename().to_lower()
		var data := _read_toml(dir.path_join(file))
		data["__file"] = dir.path_join(file)
		raw[id] = data
	themes.clear()
	var builtin_defaults: Dictionary = raw[DEFAULT_THEME] if not raw[DEFAULT_THEME].has("__error") else _read_toml(BUILTIN_DIR.path_join(DEFAULT_THEME + ".toml"))
	for id in raw:
		themes[id] = _resolve(id, raw, builtin_defaults)
	if not themes.has(settings["theme"]):
		settings["theme"] = DEFAULT_THEME
	apply_theme(settings["theme"])


func _read_toml(path: String) -> Dictionary:
	var text := FileAccess.get_file_as_string(path)
	if text == "" and not FileAccess.file_exists(path):
		return {"__error": "file not found"}
	return RoseToml.parse(text)


## Merges a theme with its base (and the default theme under everything), checks every
## value and keeps the base's value where one is wrong.
func _resolve(id: String, raw: Dictionary, defaults: Dictionary) -> Dictionary:
	var errors: Array[String] = []
	var chain: Array = []
	var at := id
	while at != "" and not chain.has(at) and raw.has(at):
		chain.push_front(at)
		var data: Dictionary = raw[at]
		if data.has("__error"):
			errors.append("%s: %s" % [at + ".toml", data["__error"]])
			break
		at = str(data.get("base", "")).to_lower().trim_suffix(".toml")
		if at != "" and not raw.has(at):
			errors.append("base \"%s\" not found" % at)
	var merged := defaults.duplicate(true)
	for link in chain:
		var data: Dictionary = raw[link]
		if data.has("__error"):
			continue
		_merge(merged, data, "", defaults, errors if link == id else [])
	var file: String = raw[id].get("__file", "")
	return {
		"id": id,
		"name": str(merged.get("name", id)) if chain.size() > 0 and not raw[id].has("__error") else id,
		"description": str(raw[id].get("description", merged.get("description", ""))),
		"file": file,
		"tokens": merged,
		"errors": errors,
	}


func _merge(into: Dictionary, data: Dictionary, prefix: String, defaults: Dictionary, errors: Array) -> void:
	for key in data:
		if str(key).begins_with("__") or key == "base":
			continue
		var value = data[key]
		var known = defaults.get(key)
		var name := prefix + str(key)
		if value is Dictionary:
			if not into.has(key) or not into[key] is Dictionary:
				into[key] = {}
			_merge(into[key], value, name + ".", known if known is Dictionary else {}, errors)
		elif known == null:
			into[key] = value  # extra values are kept; the game just doesn't use them
		elif _valid(known, value):
			into[key] = value
		else:
			errors.append("%s = %s is not valid, using %s" % [name, var_to_str(value), var_to_str(known)])


func _valid(known, value) -> bool:
	if known is String and str(known).begins_with("#") or known is String and str(known).begins_with("rgba"):
		return value is String and parse_color(value) != null
	if known is int or known is float:
		return value is int or value is float
	if known is bool:
		return value is bool
	return value is String


## "#rrggbb", "#rrggbbaa", "rgb(r, g, b)" or "rgba(r, g, b, a)"; null when not a colour.
static func parse_color(text: String):
	var t := text.strip_edges().to_lower()
	if t == "transparent" or t == "none":
		return Color(0, 0, 0, 0)
	if t.begins_with("rgb"):
		var open := t.find("(")
		var close := t.rfind(")")
		if open < 0 or close < open:
			return null
		var parts := t.substr(open + 1, close - open - 1).split(",")
		if parts.size() < 3 or parts.size() > 4:
			return null
		for p in parts:
			if not p.strip_edges().is_valid_float():
				return null
		var a := float(parts[3]) if parts.size() == 4 else 1.0
		return Color(float(parts[0]) / 255.0, float(parts[1]) / 255.0, float(parts[2]) / 255.0, a)
	if Color.html_is_valid(t):
		return Color.html(t)
	return null


func apply_theme(id: String) -> void:
	if not themes.has(id):
		id = DEFAULT_THEME
	settings["theme"] = id
	tokens = themes[id]["tokens"]
	theme = preload("res://scripts/ui/theme_builder.gd").build(self)
	# Controls under a CanvasLayer don't inherit the window's theme, so the theme also
	# goes into Godot's default theme, which every control falls back on.
	get_tree().root.theme = theme
	ThemeDB.get_default_theme().merge_with(theme)
	ThemeDB.fallback_font = theme.default_font
	ThemeDB.fallback_font_size = theme.default_font_size
	get_tree().root.propagate_notification(Control.NOTIFICATION_THEME_CHANGED)
	theme_changed.emit()


## A colour from the current theme, e.g. color("bars.hp").
func color(path: String, fallback := Color.MAGENTA) -> Color:
	var value = value_at(path)
	if value is String:
		var c = parse_color(value)
		if c != null:
			return c
	return fallback


## A number from the current theme, e.g. num("panel.radius").
func num(path: String, fallback := 0.0) -> float:
	var value = value_at(path)
	return float(value) if value is int or value is float else fallback


func value_at(path: String):
	var at = tokens
	for part in path.split("."):
		if not at is Dictionary or not at.has(part):
			return null
		at = at[part]
	return at


func is_dark() -> bool:
	return bool(value_at("dark")) if value_at("dark") is bool else false


## The theme's font at a weight: a built-in family, or a font file in the themes folder.
func font(weight := 0) -> Font:
	var family := str(value_at("font.family")).strip_edges()
	var path: String = FONTS.get(family.to_lower(), "")
	if path == "":
		for ext in ["", ".ttf", ".otf"]:
			var candidate := themes_dir().path_join(family + ext)
			if FileAccess.file_exists(candidate):
				path = candidate
				break
	if path == "":
		path = FONTS["nunito"]
	if not _fonts.has(path):
		var file := FontFile.new()
		if file.load_dynamic_font(path) != OK:
			file = null
		_fonts[path] = file
	var base: FontFile = _fonts[path]
	if base == null:
		return ThemeDB.fallback_font
	if weight <= 0:
		weight = int(num("font.weight", 500))
	var variation := FontVariation.new()
	variation.base_font = base
	var ts := TextServerManager.get_primary_interface()
	variation.variation_opentype = {ts.name_to_tag("wght"): weight}
	return variation


func set_setting(key: String, value) -> void:
	settings[key] = value
	var cfg := ConfigFile.new()
	cfg.load(SETTINGS)
	cfg.set_value("ui", key, value)
	cfg.save(SETTINGS)
	if key == "theme" or key == "opacity":
		apply_theme(settings["theme"])
	settings_changed.emit()


## Saved place of a window or widget: {"anchor": Vector2, "offset": Vector2, "scale",
## "open", "minimised"}, or {} when it was never moved.
func layout_get(id: String) -> Dictionary:
	return _layout.get_value("layout", id, {})


func layout_set(id: String, place: Dictionary) -> void:
	_layout.set_value("layout", id, place)
	_layout_dirty = true
	_save_at = Time.get_ticks_msec() + 500


func reset_layout() -> void:
	_layout.clear()
	_layout.save(LAYOUT)
	get_tree().call_group("ui_movable", "reset_place")
