## The GM commands for the suggestions while typing: name, arguments, what it does.
## Same list as the server's (module/src/gm.rs, COMMANDS), which runs them and answers
## /help; wiki/rules/gm-commands.md describes each one.
extends RefCounted

const LIST := [
	["help", "[command]", "lists the commands, or explains one"],
	["info", "", "zone, position, and the id of what you target"],
	["item", "type number [amount]", "puts an item in the bag"],
	["zuly", "amount", "gives money (a negative amount takes it)"],
	["xp", "amount", "gives experience"],
	["level", "n", "sets the level with its stat and skill points"],
	["job", "id", "0 Visitor, 111 Soldier, 121 Knight, 122 Champion, 211 Muse, 221 Mage, 222 Cleric, 311 Hawker, 321 Raider, 322 Scout, 411 Dealer, 421 Bourgeois, 422 Artisan"],
	["skill", "id", "learns a skill, skipping its requirements"],
	["learnall", "", "learns every skill of the job line at its highest level"],
	["resetstats", "", "stats back to their starting values, spent points back"],
	["resetskills", "", "removes learned skills, spent skill points back"],
	["resetlevel", "", "back to a level 1 Visitor"],
	["stat", "name value", "sets STR, DEX, INT, CON, CHA or SEN"],
	["tp", "zone [x y]", "warps to a zone, at its start or at a spot"],
	["zones", "", "lists zone ids and names"],
	["goto", "npc-id", "stands you next to an NPC"],
	["heal", "", "full HP and MP"],
	["god", "", "turns taking no damage on or off"],
	["speed", "percent", "changes walking speed (100 is normal)"],
	["spawn", "monster-id [count]", "spawns monsters around you"],
	["kill", "", "kills your target"],
	["respawn", "", "clears all monsters so spawn points refill"],
	["quest", "give id [item amount] | clear | trigger name", "quest helpers"],
	["jobvar", "index value", "sets a job variable"],
	["stamina", "value", "sets stamina"],
	["durability", "slot life", "sets an equipped item's life"],
]


## The commands whose name starts with what was typed after the slash.
static func matching(typed: String) -> Array:
	var word := typed.trim_prefix("/").get_slice(" ", 0).to_lower()
	var typed_more := typed.contains(" ")
	var out := []
	for c in LIST:
		if (typed_more and c[0] == word) or (not typed_more and c[0].begins_with(word)):
			out.append(c)
	return out
