## Base of every scripted run in res://demos/: a node added under main.gd that reads the
## game through it. A demo overrides run().
extends Node

const Main := preload("res://scripts/main.gd")

var main: Main
var options: Dictionary  # main.gd's command-line options
var origin := Vector3.ZERO  # where the character joined, for --net-demo
var online: Node3D:  # online.gd, when connected
	get:
		return main.online


func run() -> void:
	pass


## [page, index] of the first bag item whose name has this text, else [].
func _find_bag_item(text: String) -> Array:
	var pages: Array = online.net.get_inventory()["pages"]
	for page in pages.size():
		for i in pages[page].size():
			var item = pages[page][i]
			if item != null and text.to_lower() in String(item["name"]).to_lower():
				return [page, i]
	return []
