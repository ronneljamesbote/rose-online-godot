## The menu, bottom right: one button per window with its key.
extends PanelContainer

## icon, name, key, method on online.gd / main.gd that toggles the window
const ENTRIES := [
	["character", "Character", "C"],
	["inventory", "Inventory", "I"],
	["skills", "Skills", "K"],
	["quests", "Quests", "Q"],
	["party", "Party", "P"],
	["friends", "Friends", "F"],
	["map", "Map", "M"],
	["options", "Options", "O"],
	["logout", "Log out", ""],
]

signal open(which: String)


func _ready() -> void:
	theme_type_variation = "HudPanel"
	mouse_filter = Control.MOUSE_FILTER_STOP
	var row := HBoxContainer.new()
	row.add_theme_constant_override("separation", 2)
	add_child(row)
	for entry in ENTRIES:
		var b := _MenuButton.new()
		b.icon_name = entry[0]
		b.tooltip_text = "%s (%s)" % [entry[1], entry[2]] if entry[2] != "" else entry[1]
		b.theme_type_variation = "FlatButton"
		b.focus_mode = Control.FOCUS_NONE
		b.custom_minimum_size = Vector2(34, 34)
		var which: String = entry[0]
		b.pressed.connect(func(): open.emit(which))
		row.add_child(b)


## Thin line icons, 24 by 24, drawn white and tinted with the theme's menu colour.
const ICONS := {
	"character": '<path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/>',
	"inventory": '<path d="M6 2L3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6l-3-4z"/><line x1="3" y1="6" x2="21" y2="6"/><path d="M16 10a4 4 0 0 1-8 0"/>',
	"skills": '<polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/>',
	"quests": '<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/><line x1="10" y1="9" x2="8" y2="9"/>',
	"party": '<path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/>',
	"friends": '<path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"/>',
	"map": '<polygon points="1 6 1 22 8 18 16 22 23 18 23 2 16 6 8 2 1 6"/><line x1="8" y1="2" x2="8" y2="18"/><line x1="16" y1="6" x2="16" y2="22"/>',
	"logout": '<path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/><polyline points="16 17 21 12 16 7"/><line x1="21" y1="12" x2="9" y2="12"/>',
	"options": '<circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>',
}
const ICON_SIZE := 18.0
const ICON_STROKE := 1.6

## A flat button with a small line icon in the theme's menu colour.
class _MenuButton extends Button:
	## Rendered icons by name and pixel size, so each is only drawn from its SVG once.
	static var _textures := {}

	var icon_name := ""

	func _draw() -> void:
		# Rendered at the size it shows on screen, so it stays sharp at any interface size.
		var px := maxi(8, roundi(ICON_SIZE * get_global_transform().get_scale().x))
		var texture := _icon(icon_name, px)
		if texture == null:
			return
		var at := (size - Vector2(ICON_SIZE, ICON_SIZE)) * 0.5
		draw_texture_rect(texture, Rect2(at.round(), Vector2(ICON_SIZE, ICON_SIZE)), false, UI.color("menu.icon"))

	static func _icon(which: String, px: int) -> Texture2D:
		var key := "%s@%d" % [which, px]
		if not _textures.has(key):
			var svg := '<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#ffffff" stroke-width="%s" stroke-linecap="round" stroke-linejoin="round">%s</svg>' % [ICON_STROKE, ICONS.get(which, "")]
			var image := Image.new()
			if image.load_svg_from_string(svg, px / 24.0) != OK:
				return null
			image.generate_mipmaps()
			_textures[key] = ImageTexture.create_from_image(image)
		return _textures[key]
