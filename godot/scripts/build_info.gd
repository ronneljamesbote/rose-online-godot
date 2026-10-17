## Which client this is and the commit it was built from, for the version line.
## The build scripts write res://build_version.txt into the export; a run from the editor or
## a source checkout reads the commit from git instead, and counts as the dev client.
class_name BuildInfo

const VERSION_FILE := "res://build_version.txt"
const DEV_SERVER := "ws://127.0.0.1:3100"
const PRODUCTION_SERVER := "ws://127.0.0.1:3000"

static var _version := ""


## The dev client: exported with the "dev" feature (the "Windows Desktop Dev" preset), or
## not exported at all.
static func is_dev() -> bool:
	return OS.has_feature("dev") or not OS.has_feature("template")


## "dev" or "production".
static func mode() -> String:
	return "dev" if is_dev() else "production"


## The server the start screen suggests: the dev server for the dev client.
static func default_server() -> String:
	return DEV_SERVER if is_dev() else PRODUCTION_SERVER


## The git commit, 7 characters, or "unknown".
static func version() -> String:
	if _version == "":
		_version = _find_version()
	return _version


static func _find_version() -> String:
	if OS.has_feature("template"):
		if FileAccess.file_exists(VERSION_FILE):
			return FileAccess.get_file_as_string(VERSION_FILE).strip_edges()
		return "unknown"
	var root := ProjectSettings.globalize_path("res://").path_join("..").simplify_path()
	var out := []
	if OS.execute("git", ["-C", root, "rev-parse", "--short=7", "HEAD"], out) == 0 and out.size() > 0:
		var hash: String = out[0].strip_edges()
		if hash != "":
			return hash
	# No git on PATH: read .git/HEAD and the branch it points to.
	var git := root.path_join(".git")
	var head := FileAccess.get_file_as_string(git.path_join("HEAD")).strip_edges()
	if head.begins_with("ref: "):
		var ref := head.trim_prefix("ref: ")
		head = FileAccess.get_file_as_string(git.path_join(ref)).strip_edges()
		if head == "":
			for line in FileAccess.get_file_as_string(git.path_join("packed-refs")).split("\n"):
				if line.ends_with(" " + ref):
					head = line.get_slice(" ", 0)
	return head.left(7) if head.length() >= 7 else "unknown"
