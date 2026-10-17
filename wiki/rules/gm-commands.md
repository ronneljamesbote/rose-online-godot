---
kind: rule
id: gm-commands
name: GM commands (dev servers only)
status: changed-from-irose
who_may_use: every player on a dev server
production_servers: no GM commands (the server is built without them)
dev_server_port: 3100
max_level: 250
spawn_max_count: 50
speed_percent: 10 to 1000
commands:
  - { command: "/help [command]", does: "Lists the commands, or explains one" }
  - { command: "/info", does: "Your zone, position, level, job and entity id, and the id of what you target" }
  - { command: "/item type number [amount]", does: "Puts an item in the bag (iROSE item type and number)" }
  - { command: "/zuly amount", does: "Gives Zuly (a negative amount takes it)" }
  - { command: "/xp amount", does: "Gives experience, levelling up as usual" }
  - { command: "/level n", does: "Sets the level, with the stat and skill points of the levels gained or taken away" }
  - { command: "/job id", does: "Changes the job, following the job rules below" }
  - { command: "/skill id", does: "Learns a skill (that exact skill and level), skipping its requirements" }
  - { command: "/learnall", does: "Learns every skill of the job line at its highest level, free" }
  - { command: "/resetstats", does: "Stats back to their starting values, stat points spent on them back" }
  - { command: "/resetskills", does: "Removes every skill but the starting ones, skill points spent on them back" }
  - { command: "/resetlevel", does: "Back to a level 1 Visitor" }
  - { command: "/stat name value", does: "Sets STR, DEX, INT, CON, CHA or SEN" }
  - { command: "/tp zone [x y]", does: "Warps to a zone, at its start or at a spot (metres, as /info shows them)" }
  - { command: "/zones", does: "Lists zone ids and names" }
  - { command: "/goto npc-id", does: "Stands you 2 m from a town NPC" }
  - { command: "/heal", does: "Full HP and MP" }
  - { command: "/god", does: "Turns god mode on or off: attacks and skills do no damage to you" }
  - { command: "/speed percent", does: "Walking speed in percent of normal (10 to 1000)" }
  - { command: "/spawn monster-id [count]", does: "Spawns up to 50 monsters around you; they don't respawn" }
  - { command: "/kill", does: "Kills your target, with the usual experience and drops" }
  - { command: "/respawn", does: "Removes every monster so the spawn points refill" }
  - { command: "/quest give id [item amount]", does: "Gives a quest, with an amount of a quest item" }
  - { command: "/quest clear", does: "Drops every active quest" }
  - { command: "/quest trigger name", does: "Runs a quest trigger" }
  - { command: "/jobvar index value", does: "Sets job variable 0, 1 or 2 (job change quests)" }
  - { command: "/stamina value", does: "Sets stamina" }
  - { command: "/durability slot life", does: "Sets the life of a worn item (slot 0 face, 1 head, 2 body, 3 back, 4 hands, 5 feet, 6 weapon, 7 off-hand, 8 necklace, 9 ring, 10 earring)" }
jobs:
  - { id: 0, job: "Visitor", sets: "job variables cleared" }
  - { id: "111, 211, 311, 411", job: "Soldier, Muse, Hawker, Dealer", sets: "job variable 0 = 1, job variable 1 = 0" }
  - { id: "121, 122", job: "Knight, Champion (Soldier line)", sets: "job variable 0 = 1, job variable 1 = 2" }
  - { id: "221, 222", job: "Mage, Cleric (Muse line)", sets: "job variable 0 = 1, job variable 1 = 2" }
  - { id: "321, 322", job: "Raider, Scout (Hawker line)", sets: "job variable 0 = 1, job variable 1 = 2" }
  - { id: "421, 422", job: "Bourgeois, Artisan (Dealer line)", sets: "job variable 0 = 1, job variable 1 = 2" }
source:
  code:
    - module/src/gm.rs (gm, COMMANDS, reset_stats, reset_skills, reset_level, unequip_unwearable)
    - module/Cargo.toml (the dev feature)
    - module/src/lib.rs (server_info, deal_damage for god mode)
    - module/src/character.rs (player_stats for /speed)
    - compose.yaml (rose-server-dev, profile dev)
    - godot/gm/gm_chat.gd, godot/gm/gm_commands.gd
    - godot/rust/src/net.rs (gm_command, feature dev)
---
# GM commands (dev servers only)

GM commands are for testing and development. They only exist on a **dev server**, a server
built with the `dev` option: a production server is built without them, so nothing on it
can switch them on. On a dev server **every player** may use them, and nobody needs an
account to sign in. The dev server runs next to the production one (port **3100**, its own
world), see `server/README.md`.

Type a command in the [[rules/chat|chat box]], starting with `/`. While you type, the
commands that match show above the box; **Tab** fills in the first one. The answer comes
back in the chat log as lines starting with "GM:". `/r` still answers a whisper.

A command acts on your own character. Put `@Name` at the end to act on another player
instead, for example `/level 50 @Lowbie`. `/info` and `/kill` use what you have targeted.

The commands only work in a client that has them: the dev client, or the game run from the
Godot editor with the dev extension. The production client sends lines starting with `/` as
chat. A dev client on a production server gets "this server has no GM commands".

## Commands

The table at the top lists every command. Numbers are typed as numbers; item types are
iROSE's (1 face, 2 head, 3 body, 4 hands, 5 feet, 6 back, 7 jewellery, 8 weapon, 9 off-hand,
10 consumable, 11 gem, 12 material, 13 quest, 14 vehicle).

## Job rules

`/job id` changes the job the way the job change quests would have left it:

- A **first job** sets job variable 0 to 1 (first job done) and job variable 1 to 0 (no
  second job). So stepping down from a second job to its first job removes the second job.
- A **second job** sets job variable 0 to 1 and job variable 1 to 2 (second job done), the
  values the second job change quests set. Its first job comes with it: `/job 121` is a
  Soldier who became a Knight.
- `/job 0` makes a Visitor and clears the job variables.
- Any other id is refused, with the list of valid ones.

Skills the new job can't use are removed and their skill points come back: a skill stays
when it has no job class, or its job class takes the new job (or, for a second job, its
first job). So a Champion changed to Soldier keeps the Soldier skills and loses the
Champion ones.

## Resets

- **`/resetstats`**: STR, DEX, INT, CON, CHA and SEN go back to the starting values
  (STR 15, DEX 15, INT 15, CON 15, CHA 10, SEN 10, see [[rules/stats|Stats]]) and every
  point bought above them comes back, at what each point cost (its value ÷ 5, rounded
  down). Unspent points stay, so points from quests aren't lost.
- **`/resetskills`**: every skill but the starting ones (sit, pick up and the other basic
  actions) is removed, and the skill points spent on each of its levels come back.
- **`/resetlevel`**: level 1 Visitor with no experience, starting stats and skills and no
  points to spend: a new character apart from its items, Zuly and quests. Use `/job` after
  it for a job.
- **`/level n`**: going up adds the stat and skill points of each level gained, as
  levelling would (and runs the level up quest triggers). Going down first resets stats and
  skills, then takes away the points of the levels lost. Experience goes to 0.

Skills that `/learnall` or `/skill` gave are free, so removing them (by a reset, `/job` or
`/level`) gives no skill points back.

After a reset, `/job` or `/level`, the hotbar slots of removed skills are emptied, HP and MP
are filled, and worn gear the character can't wear any more goes to the bag. If the bag
is full the item stays on and the chat says which one.

## Learn all

`/learnall` takes every skill whose job class takes the current job, or for a second job
also its first job, and learns its highest level for free, in prerequisite order. Stat and
level requirements are skipped. Skills of other lines, clan skills and skills tied to no
job aren't given, and skills already known are raised instead of added twice. A Visitor has
no job line, so it refuses.

## Changed from iROSE

iROSE has GM commands for game masters on the live server. Ours are a testing tool that
only exists on dev servers, for everyone on them; production servers have none.

> Open question: points set with `/stat` count as bought, so `/resetstats` gives back the points their cost would have been.
