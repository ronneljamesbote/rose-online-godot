## The line at the top left that names the client and server: their mode (dev or
## production) and version (git commit). It shows on the start screen, the loading screen and
## in game, above everything else, and never takes clicks. main.gd hands it the connection,
## and after each zone load the line also says how that zone was built (zone_line()). One
## line, so it never runs into the player frame under it.
extends CanvasLayer

const AMBER := "#f0b030"

## RoseNet of the current connection, or null.
var net: Node
## The zone part at the end of the line, or "" before any zone is loaded.
var zone := ""
var _label: RichTextLabel
var _shown := ""
var _next_check := 0.0


func _ready() -> void:
	layer = 110  # over the loading screen (100)
	_label = RichTextLabel.new()
	_label.bbcode_enabled = true
	_label.fit_content = true
	_label.autowrap_mode = TextServer.AUTOWRAP_OFF
	_label.scroll_active = false
	_label.mouse_filter = Control.MOUSE_FILTER_IGNORE
	_label.position = Vector2(12, 6)
	_label.add_theme_font_size_override("normal_font_size", 14)
	_label.add_theme_color_override("default_color", Color(1, 1, 1))
	_label.add_theme_color_override("font_outline_color", Color(0, 0, 0, 0.85))
	_label.add_theme_constant_override("outline_size", 4)
	add_child(_label)
	_refresh()


func _process(delta: float) -> void:
	_next_check -= delta
	if _next_check <= 0.0:
		_next_check = 1.0
		_refresh()


func _refresh() -> void:
	var info: Dictionary = {}
	if is_instance_valid(net) and net.has_method("server_info"):
		info = net.server_info()
	var text := line(BuildInfo.mode(), BuildInfo.version(), info.get("mode", ""), info.get("version", ""))
	if zone != "":
		text += " · " + zone
	if text != _shown:
		_shown = text
		_label.text = text


## "DEV · client abc1234 · server abc1234". The server part is amber when its commit differs
## from the client's; when the modes differ both are named ("DEV client x · PRODUCTION
## server y"). An empty server mode means not connected (yet).
static func line(client_mode: String, client_version: String, server_mode: String, server_version: String) -> String:
	if server_mode == "":
		return "%s · client %s · server not connected" % [client_mode.to_upper(), client_version]
	var server := "server %s" % server_version
	if server_version != client_version:
		server = "[color=%s]%s[/color]" % [AMBER, server]
	if server_mode == client_mode:
		return "%s · client %s · %s" % [client_mode.to_upper(), client_version, server]
	return "%s client %s · %s %s" % [client_mode.to_upper(), client_version, server_mode.to_upper(), server]


## "Zone 23 Breezy Hills · Godot scene", or "· old loader" for a zone built from the ROSE
## map files. When the zone has a scene but it was not used, the reason follows in amber:
## "old loader (assets missing)" means the converted assets were not imported.
static func zone_line(zone_id: int, zone_name: String, stats: Dictionary) -> String:
	var zone_text := ("Zone %d %s" % [zone_id, zone_name]).strip_edges()
	if stats.get("source", "") == "scene":
		return "%s · Godot scene" % zone_text
	var fallback: String = stats.get("fallback", "")
	if fallback == "":
		return "%s · old loader" % zone_text
	return "%s · [color=%s]old loader (%s)[/color]" % [zone_text, AMBER, fallback]
