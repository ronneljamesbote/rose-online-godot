## Starts the scripted run asked for on the command line. main.gd calls start() for
## --net-demo=NAME once online, and for --demo and --effect=PATH,... offline.
extends RefCounted

const NET_DEMOS := {
	"fight": "fight.gd",
	"kite": "kite.gd",
	"leash": "leash.gd",
	"walls": "walls.gd",
	"warp": "warp.gd",
	"shop": "shop.gd",
	"skills": "skills.gd",
	"talk": "talk.gd",
	"party": "party.gd",
	"craft": "craft.gd",
	"trade": "trade.gd",
	"pvp": "pvp.gd",
	"chat": "chat.gd",
	"glow": "glow.gd",
	"steps": "steps.gd",
	"hotkey": "hotkey.gd",
	"sticky": "sticky.gd",
	"hotbar-items": "hotbar_items.gd",
	"hotbar-clear": "hotbar_clear.gd",
}


## --demo=NAME (offline); any other name, or none, runs offline.gd.
const OFFLINE_DEMOS := {
	"camera": "camera.gd",
	"interface": "interface.gd",
}


## which is "net-demo" (online, me is our character), "demo" or "effect".
static func start(main: Node3D, which: String, me: Node3D = null) -> void:
	var file := "effect.gd"
	if which == "demo":
		file = OFFLINE_DEMOS.get(str(main.options["demo"]), "offline.gd")
	if which == "net-demo":
		# Any other name (square, line, none) walks a route.
		file = NET_DEMOS.get(main.options["net-demo"], "route.gd")
	var demo: Node = load("res://demos/" + file).new()
	demo.name = "Demo"
	demo.main = main
	demo.options = main.options
	if me != null:
		demo.origin = me.position
	main.add_child(demo)
	demo.run()
