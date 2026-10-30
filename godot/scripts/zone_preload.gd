## Reads a zone's files ahead on background threads while our character stands near a warp
## gate to it (online.gd says which), so walking through only has to make the nodes.
## Walking away lets the files go again. Does nothing per frame while idle.
extends Node

var zone_id := 0  # the zone being read ahead (0: none)
var _wanted := {}  # its files (path -> true)
var _pending: PackedStringArray = []  # requested and not fetched yet, wanted or not
var _held: Array[Resource] = []  # the files read, kept loaded until used or let go


func _ready() -> void:
	set_process(false)


## Read zone `id` ahead (0 or a zone without scenes: stop reading ahead).
func want(id: int) -> void:
	if id == zone_id:
		return
	let_go()
	var files := RoseZone.zone_files(id) if id > 0 else PackedStringArray()
	if files.is_empty():
		return
	zone_id = id
	print("rose: reading zone %d ahead" % id)
	for path in files:
		_wanted[path] = true
		if not _pending.has(path) and ResourceLoader.load_threaded_request(path, "", true) == OK:
			_pending.append(path)
	set_process(not _pending.is_empty())


## Stop reading ahead and let the files read go (a file still being read is let go when
## it is done).
func let_go() -> void:
	if zone_id != 0:
		print("rose: no longer reading zone %d ahead" % zone_id)
	zone_id = 0
	_wanted.clear()
	_held.clear()


func _process(_delta: float) -> void:
	for path in _pending.duplicate():
		var status := ResourceLoader.load_threaded_get_status(path)
		if status == ResourceLoader.THREAD_LOAD_IN_PROGRESS:
			continue
		_pending.remove_at(_pending.find(path))
		# Every request needs its get, or Godot keeps the file for good.
		var resource: Resource = ResourceLoader.load_threaded_get(path) if status == ResourceLoader.THREAD_LOAD_LOADED else null
		if resource and _wanted.has(path):
			_held.append(resource)
	if _pending.is_empty():
		set_process(false)
