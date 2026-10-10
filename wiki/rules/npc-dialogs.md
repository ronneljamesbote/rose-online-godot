---
kind: rule
id: npc-dialogs
name: NPC dialogs
status: in-game
talk_range_m: 3
dialog_closes_beyond_m: 15
npc_variables: 20
npc_variable_max_by_add: 500
npc_idle_script_default_s: 60
npc_idle_script_min_s: 1
npc_local_chat_range_m: 20
world_tick_s: 10
services:
  - { answer: "→ opens the shop", does: "opens the NPC's store window", page: "[[rules/shops|Shops]]" }
  - { answer: "→ opens storage", does: "opens your storage next to the inventory", page: "[[rules/bank|Bank]]" }
  - { answer: "→ opens repair", does: "repair mode in the inventory", page: "[[rules/durability|Durability]]" }
  - { answer: "→ opens refining", does: "opens the refining window", page: "[[rules/refining|Refining]]" }
  - { answer: "→ opens disassembly", does: "opens the disassembly window", page: "[[rules/refining|Refining]]" }
  - { answer: "→ saves this town as where you get up after dying", does: "sets your save point to this zone", page: "[[rules/death|Death]]" }
  - { answer: "→ appraises items", does: "says \"That service is not in the game yet\"", page: "" }
  - { answer: "→ opens item delivery", does: "says \"That service is not in the game yet\"", page: "" }
  - { answer: "→ founds a clan / disbands your clan", does: "does nothing yet (no clans)", page: "" }
source:
  code:
    - godot/rust/src/conversation.rs (Conversation::open, run_menu, choose, ScriptContext::call, format_text)
    - godot/rust/src/net.rs (conversation actions)
    - godot/scripts/conversation_window.gd (TALK_RANGE, keys 1-9)
    - godot/scripts/online.gd (talk_to, _open_npc, TALK_RANGE)
    - module/src/quests.rs (quest_trigger, run_trigger)
    - module/src/npcs.rs (Npc, spawn_npcs, NPC_VARIABLES)
    - module/src/npc_ai.rs (npc_ai_tick, run_trigger, run_npc_trigger, zone_time, is_daytime, npc_message)
    - crates/rose-quest/src/lib.rs (check_trigger, world_ticks)
  data: "NPC dialog files 3Ddata\\Event\\*.con (menus and a Lua 4 script), EVENT string table (the text), LIST_NPC.STB (each NPC's AI file), NPC AI files (*.AIP), zone files (where NPCs stand and which dialog they use)"
---
# NPC dialogs

Left-click a town NPC to talk to it. If you are more than 3 m away your character walks
over first. The NPC says something and you pick an answer, either with the mouse or with
the number keys 1 to 9. Answers can lead to more talk, start or finish a quest, or open a
service such as the shop or your storage. Walking more than 15 m away, or pressing Esc,
ends the conversation.

An NPC that has nothing to say opens its shop straight away, if it has one. Otherwise you
see "<name> has nothing to say".

## How the NPC picks what to say

Every NPC's dialog is a list of menus. A menu holds the NPC's possible messages and your
possible answers, each with an optional **condition** (a check on your character, your
quests or the NPC).

When a menu opens, the game goes through its lines from top to bottom:

- An **NPC message** whose condition passes becomes what the NPC says, and the game opens
  the menu under it to fill in the answers. A later NPC message whose condition also
  passes **replaces** it. So the NPC says the **last** message in the list whose condition
  passes. The data puts the plain greeting first and the quest lines after it, so a quest
  line wins over the greeting when it applies.
- An **answer** whose condition passes is added to the list of answers you can pick.
  Answers whose condition fails are hidden.

A message or answer without a condition always passes. If nothing in the opening menu
passes, or the dialog's opening check says no, the NPC has nothing to say.

When you pick an answer, its **action** runs first (for example a quest step or opening
the shop), then the menu it points to opens with the same rules. If that menu has nothing
that passes, the conversation ends.

The text can contain your name and level; the game fills them in, and keeps the original
bold text and colours.

## Conditions

Conditions are small scripts in the dialog file. The ones the game understands:

| Condition on the page | What it checks |
| --- | --- |
| you have / you don't have *quest* | whether the quest is in your quest list |
| you carry N × *item* for *quest* | how many of a quest item that quest holds |
| quest switch N = N | one of your 1,024 on/off quest switches |
| job variable, episode variable | your quest progress counters (see [[rules/quests\|Quests]]) |
| your level, your job, your union | your character (job is the raw job number) |
| the NPC's quest switch | the NPC's own variable 0, which its time-of-day script sets (below) |
| quest step `X` passes | the checks of that quest step all pass right now |

"Quest step `X` passes" uses the same checks as the quest itself (see
[[rules/quests|Quests]]). Your game checks them to decide what to show; random rolls
always count as passing there, because only the server rolls dice.

## Actions

An answer's action can do one or more of these:

- **Run a quest step.** If the step's checks pass on your game, it asks the server to run
  it. The server checks everything again and then gives the rewards, so nothing can be
  skipped. A dead character can't run quest steps.
- **Open a service.** The shop, storage, repair, refining and disassembly windows open for
  the NPC you are talking to. Appraisal and item delivery show "That service is not in the
  game yet". Clan answers do nothing yet.
- **Set your save point** to this zone (where you get up after dying, see
  [[rules/death|Death]]).

NPC motions and effects while talking are not shown yet.

## How NPC pages show dialogs

Each page in [[npcs|NPCs]] has a **Dialog** section that writes the whole dialog out as a
nested list:

- `**NPC:**` lines are what the NPC says. When several sit at the same level, the last one
  whose condition passes is the one you see.
- `**You:**` lines under it are your answers.
- Text in italics and brackets after a line, like *(if your level ≤ 15)*, is its
  condition. Several conditions are separated by `;` and all must pass.
- `→` after an answer is its action, for example `→ quest step 5009-01 passes; runs quest
  step 5009-01` (the step is checked, then run) or `→ opens the shop`.
- Lines indented under an answer are what happens after you pick it.

The `services` field at the top of an NPC page lists what its dialog can open, and `shop`
lists the store's items (see [[rules/shops|Shops]]).

Service answers:

| Answer on the page | What happens |
| --- | --- |
| → opens the shop | the store window (see [[rules/shops\|Shops]]) |
| → opens storage | your storage (see [[rules/bank\|Bank]]) |
| → opens repair | repair mode (see [[rules/durability\|Durability]]) |
| → opens refining | refining (see [[rules/refining\|Refining]]) |
| → opens disassembly | disassembly (see [[rules/refining\|Refining]]) |
| → saves this town as where you get up after dying | sets your save point |
| → appraises items, → opens item delivery | not in the game yet |

## NPC scripts and the time of day

Besides its dialog, a town NPC can have an AI script. Every so often (the interval comes
from the script, 60 seconds if it has none and never faster than once a second) the NPC
runs the first event of its idle script whose checks pass. The script can:

- check the time of day in the zone, day or night, the real date, weekday and time of day,
  a random roll, or its own variables or another NPC's variables in the same zone;
- set or change its own or another NPC's variables. Each NPC has 20 variables, all 0 when
  the server starts. Adding to another NPC's variable stops at 500, and taking away stops
  at 0;
- say something: local chat reaches players within 20 m, a shout the whole zone;
- run a quest step on itself (only NPC checks and rewards apply there).

Town NPCs use this to switch their quests on and off. For example [[npcs/1001-village-chief-cornell|Cornell]] in Zant offers
the Candle Ghost hunt only while his quest switch (variable 0) is 1, which his AI script switches. The dialog shows this as *(if the NPC's quest switch = 1)*.

Game time runs in world ticks of 10 seconds. A zone's day is a cycle of ticks from the
zone data; day, evening and night are parts of that cycle.

> Open question: quest checks on "the zone's time" (ObjectZoneTime) use the world time without the zone's day cycle, while NPC scripts use the zone's day cycle. Should both use the zone's day cycle?

The server has one channel, so script checks on the channel number always see channel 1.
