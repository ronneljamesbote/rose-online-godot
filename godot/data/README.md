# Game data

The game's own data, made in the Godot editor: one resource (`.tres`) per thing, edited in
the Inspector. It replaces the iROSE tables one kind at a time
(plan: `TODO/own-game-data.md` in the project files). So far: **items**, **monsters**, **NPCs**, **drop tables**, **shop tabs**, **skills**,
**status effects**, **job classes**, **zone settings**, **warp gates**, **crafting recipes**,
**new characters**, **sounds**, **effects**, **skies**, the **interface**'s pictures and
cursors, **quests**, **NPC dialogs** and the **monster AI**.

| Folder | Class | What |
| --- | --- | --- |
| `items/face/` | `RoseItemFace` | Masks, glasses |
| `items/head/` | `RoseItemHead` | Helmets, hats |
| `items/body/` | `RoseItemBody` | Armour, clothes |
| `items/hands/` | `RoseItemHands` | Gloves, gauntlets |
| `items/feet/` | `RoseItemFeet` | Boots, shoes (item 0: the run speed barefoot) |
| `items/back/` | `RoseItemBack` | Back armour, bags, wings, ammunition boxes |
| `items/jewellery/` | `RoseItemJewellery` | Rings, necklaces, earrings |
| `items/weapon/` | `RoseItemWeapon` | Weapons (item 0: fighting unarmed) |
| `items/subweapon/` | `RoseItemSubWeapon` | Shields, support tools |
| `items/consumable/` | `RoseItemConsumable` | Potions, food, scrolls, skill books, repair tools |
| `items/gem/` | `RoseItemGem` | Gems and jewels for sockets |
| `items/material/` | `RoseItemMaterial` | Crafting materials, arrows, bullets, shells |
| `items/quest/` | `RoseItemQuest` | Quest items |
| `items/vehicle/` | `RoseItemVehicle` | Cart and castle gear parts |
| `items/grades/` | `RoseItemGrade` | Refine grades 0 to 9: what each adds, the glow colour |
| `monsters/` | `RoseMonster` | Monsters (and summons, pets): stats, rewards, drop table |
| `npcs/` | `RoseNpc` | Town NPCs (the ones a zone places as NPCs): look, shop tabs |
| `drop_tables/` | `RoseDropTable` | What a monster (or a zone) drops |
| `shop_tabs/` | `RoseShopTab` | One tab of an NPC shop: up to 48 items |
| `skills/<id>-<name>/` | `RoseSkill` | One skill: a resource per level (each level has its own id) |
| `status_effects/` | `RoseStatusEffect` | Buffs, debuffs, recovery and poison over time, stuns ... |
| `job_classes/` | `RoseJobClass` | Sets of jobs a skill or item can require ("Soldier Job") |
| `zones/` | `RoseZoneSettings` | A zone's settings: name, map file, start and revive spots, music, sky, minimap, time of day, PvP, planet, carts |
| `warp_gates/` | `RoseWarpGate` | Where a warp gate sends you: zone and named spot |
| `craft_recipes/` | `RoseCraftRecipe` | A crafting or refining recipe: up to 4 materials (recipes 1-20 refine) |
| `new_characters/` | `RoseNewCharacter` | What a new male or female character starts with: stats, clothes, bag |
| `sounds/<folder>/` | `RoseSound` | One sound: a number, a name and the audio file it plays (folders as the client's `Sound/`: attack, mob, skill ...) |
| `hit_sounds/` | `RoseHitSounds` | What a weapon's hit sounds like on armour, cloth and stone |
| `footsteps/` | `RoseFootsteps` | A zone's footsteps: the step sound for each of its ground textures |
| `effect_files/` | `RoseEffectFile` | One effect the game can show: a number, a name and the effect scene it plays |
| `attack_effects/` | `RoseAttackEffect` | How an attack looks: the effects on the weapon, the swing's trail, the projectile and its flight, the hit effects and sounds |
| `skies/` | `RoseSky` | A sky: the dome around a zone (mesh, day and night picture) and its light at each time of day |
| `quests/` | `RoseQuest` | A quest: its name, text (description, start and end messages) and time limit |
| `quest_triggers/<qsd file>/` | `RoseQuestTrigger` | A quest step: its conditions, its rewards and the step tried when it fails (a folder per iROSE QSD file) |
| `dialogs/` | `RoseDialog` | What a town NPC says when you talk to it: its menus, lines, answers and what they check and do |
| `ai/` | `RoseAi` | How a kind of monster (or town NPC) behaves: its events and what they check and do |
| `interface/interface.tres` | `RoseInterface` | The interface's own pictures (loading screen, character creation) and the mouse cursors with the pixel each clicks at (one resource) |

Monsters and NPCs share one id space (an NPC and a monster never have the same id), because
saved quests and the zone markers store just the number. Links between resources are plain
resource fields: a monster's `drop_table`, an NPC's `shop_tab_1` to `shop_tab_4`, the items
of a drop table or shop tab, and the monsters of a zone's spawn markers (`RoseSpawnSpot`)
and the NPC of an NPC marker (`RoseNpcSpot`), a skill's `required_job_class`, the skills it
needs (`required_skill_1` to `_3`), the status effects it gives (`status_effect_1` and `_2`)
and the monster it summons (`summon_monster`), a warp gate's `target_zone`, a recipe's
materials (`material_1` to `_4`) and a new character's clothes and bag. Sounds are links too:
an item's `equip_sound` and a weapon's `attack_start_sound` and `hit_sounds`, a monster's
`attack_sound` or `die_sound`, a skill's `bullet_fire_sound`, a zone's `footsteps`, and a
`RoseSound`'s `stream` (the audio file); a zone's `music_day` and `music_night` and a sound
spot in a zone scene (`RoseSoundSpot`'s `stream`) link the audio file itself. Effects are
links the same way: a weapon's `attack_effect` and `bullet_effect`, a gem's `gem_effect`,
ammunition's `bullet_effect`, a cart part's `hit_effect` and `bullet_effect`, a monster's
`hand_hit_effect` link a `RoseAttackEffect`; a potion's `effect`, a cart part's
`ride_effect` ... and `dummy_effects`, a monster's `die_effect`, a skill's
`casting_effect_1` to `_4`, `hit_effect` and `hit_dummy_effect_1/2` and a status effect's
`effect` link a `RoseEffectFile`, whose `scene` links the effect scene; an effect spot in a
zone scene (`RoseEffectSpot`'s `scene`) links the effect scene itself. Drag a resource from
the FileSystem dock onto the field to set it. A zone scene's root (`RoseZoneScene`) shows its
zone's settings as `settings` (click it to open them), and a warp gate placed in a zone
(`RoseZoneObject` of kind warp) links its `RoseWarpGate` as `warp_gate`.

A zone needs its settings to be played: a zone scene without a `RoseZoneSettings` of the same
number stops the export. A recipe's first material can be any item of a class
(`raw_material_class`, "Metal"): leave `material_1` empty and set `material_1_quantity`.
Items name their recipe by its `id` (`craft_material` on the item); the refining recipes are
fixed: 1 to 10 raise a weapon from
grade 0 to 10, 11 to 20 armour. A new character's bag counts (`bag_consumable_counts`,
`bag_material_counts`) go with the item at the same place in the list.

Each level of a skill is its own resource with its own id, as characters store the id of
the level they learnt; the levels of a skill sit in one folder named after the first level
(`skills/1041-purify/1041-purify.tres`, `1042-purify.tres`, `1043-purify.tres`) and share
its id in `base_skill_id` (0 on the first level itself). That field and a status effect's
`apply_status_effect_1/2` stay plain numbers, because they usually name the resource itself,
which a resource can't link. A new level is a copy of the last one with the next free id,
`level` one higher and the same `base_skill_id`.

A drop table has 30 `slots` and 4 groups of 5 items. A slot is an item, or a number 1 to 4
that means "one of group N". When a monster drops something the game picks a slot (see
`wiki/rules/drops.md` for how). Spawn and NPC markers also store the plain ids
(`basic_monster_ids`, `npc_id`), so a marker whose monster has no resource keeps its id.

Files are named `<id>-<name>.tres` (`weapon/2-short-sword.tres`, `monsters/101-woopie.tres`).
The name is only for finding them: the game goes by the `id` field and by the class, so a
file can be renamed or moved to any folder of its kind. Hover a field in the Inspector for what it does.

## Sounds

A `RoseSound` plays its `stream`: an audio file of the game. `--import-rose` (see
`../assets/README.md`) turns the client's WAV and OGG files into Godot audio under
`res://assets/rose/sound/`, so run it before editing sounds; without it the links show as
missing in the editor and the game plays the client's files instead. To give something a
new sound, put a WAV or OGG file in `res://assets/ours/` (anywhere), make a new `RoseSound`
with a free number and drag the file onto its `stream`, then link the sound where it should
play (an item's `attack_start_sound` ...). A file in `res://assets/ours/` at the same path as
an imported one (`ours/sound/attack/sword_001.wav`) plays instead of it everywhere.

Sounds are only played by the game, not by the server: the export writes them
(`sounds.toml`, `hit-sounds.toml`, `footsteps.toml`) and the server upload leaves them out.

## Effects

A `RoseEffectFile` plays its `scene`: an effect scene, a `RoseEffect` node with one child per
part (`RoseEffectParticles` for a particle system, `RoseEffectMesh` for a mesh with its
texture, blending and animation). `--import-rose` turns the client's EFT and PTL files into
these scenes (`res://assets/rose/3ddata/effect/levelup_01.scn`) and particle systems
(`.particles.res`, a `RoseParticleSystem` with its `RoseParticleSequence`s and keyframes), so
run it before editing effects. Open an effect scene to see it play in the editor (it starts
again every few seconds); select a part to change its place, its particle system, mesh,
texture or blending in the Inspector, or open a particle system to change its sequences. To
make a new effect, save a copy of a scene (or a new `RoseEffect` scene) in
`res://assets/ours/`, make a new `RoseEffectFile` with a free number, drag the scene onto its
`scene` and link the effect file where it should show. A scene in `res://assets/ours/` at the
same path as an imported one plays instead of it everywhere.

A `RoseAttackEffect` is one row of how attacks look: up to 4 `point_effects` drawn on the
weapon's effect points, the swing's trail (`trail_colour`, a row of the client's trail
colours, and `trail_milliseconds`), the projectile (`bullet_effect`, `bullet_move`: Linear,
Parabola or Immediate, `bullet_speed` in centimetres per second), the effect on the target
(`hit_effect`, and `hit_effect_critical` on a critical hit) and the `fire_sound` and
`hit_sound`. It is named after the first item, monster or skill that uses it.

Effects are only shown by the game, not by the server: the export writes them
(`effect-files.toml`, `attack-effects.toml`) and the server upload leaves them out.

## Icons and models

An item's, skill's and status effect's `icon` is a texture: the Inspector shows its thumbnail,
and the Game data table shows it in the row. `--import-rose` (see `../assets/README.md`) makes
one small `AtlasTexture` per client icon (`res://assets/rose/3ddata/control/res/item1/919.tres`,
`skillicon/`, `stateicon/`), so run it before editing these. Models are paths, not loaded
resources, so opening a resource or the table never loads a mesh: an item's `field_model` (the
model lying on the ground; empty for none, only its name shows), a worn item's `model` (face
items, back items, weapons, shields, cart parts) or `model_male` and `model_female` (hats,
bodies, gloves, shoes), and a monster's or NPC's `model`
(`res://assets/rose/3ddata/npc/list_npc/12.tscn`).

To give something new art, put the file in `res://assets/ours/` (anywhere): a PNG or any
texture for an icon, a scene built like the imported ones for a model. Then pick it with the
field's **Load** (the folder icon): its dialog shows `assets/`, which the FileSystem dock
hides. Two things don't follow a model of our own yet: a weapon's gem point (where a
socketed gem shows), read from the iROSE model list, and a monster's or NPC's motions,
which come from its model's `LIST_NPC.CHR` entry (so a new monster reuses a model, or an edited
copy at the same path in `res://assets/ours/`). The export stops naming the resource when a
model path names no file. Icons and models are only drawn by the game; the server keeps the
paths but draws nothing.

## Skies and the interface

A zone's `sky` links a `RoseSky`: the dome drawn around the zone (`mesh`, a path like the
models, so opening the sky draws no preview of it; `texture_day` and `texture_night`, which
the game blends through the evening and back in the morning) and the colour of the light at
each time of day (`map_ambient_*` on the ground and buildings, `character_ambient_*` and
`character_diffuse_*` on characters and monsters). Several zones share one (sky 0 is every
zone on Junon above ground). The numbers count from 0 as iROSE's LIST_SKY.STB rows do.

`interface/interface.tres` (a `RoseInterface`) holds the interface's own pictures, the
loading screen's `loading_picture` and character creation's `character_create_picture`, and a
picture and click point (`cursor_attack` and `cursor_attack_click`, in pixels from the
top-left corner at normal size) for each mouse cursor. `--import-rose` imports the client's
loading picture (`res://assets/rose/3ddata/control/res/loading.rtex`) and cursors
(`res://assets/rose/res/attack.res` ...); a PNG of our own in `res://assets/ours/` works the
same way. An empty picture is the client's own. The rest of the interface (windows, fonts,
colours) is our own and lives in `res://ui/` and the themes.

Skies and the interface are only drawn by the game: the export writes them (`skies.toml`,
`interface.toml`) and the server upload leaves them out.

## Quests

A quest (`quests/<id>-<name>.tres`, a `RoseQuest`) is its `id` (saved characters store it, so
don't change it), `name`, `description`, `start_message`, `end_message` and `time_limit` in
seconds (0: none). The game runs a quest by its **steps**: a `RoseQuestTrigger` in
`quest_triggers/`, named like `5051-34` (the name is what NPC dialogs, monster deaths and
`levelup_<level>` ask for, so a rename must change those too). A step has:

- `conditions`: blocks that must all pass, such as `RoseConditionSelectQuest` (works on
  this quest), `RoseConditionQuestItem` (carries `count` of an `item`, compared with
  `operator`) or `RoseConditionAbilityValue` (your level, stats, Zuly). 31 kinds.
- `rewards`: blocks given in order, such as `RoseRewardAddItem`, `RoseRewardCalculatedMoney`
  (Zuly from a reward formula, `equation`, and a `value`), `RoseRewardSpawnMonster` or
  `RoseRewardNpcMessage` (its `text` is written in the block). 40 kinds.
- `next_trigger`: the step tried when this one fails, so one answer can try several steps.

In the Inspector, click **Conditions** or **Rewards**, then a block to open it; **Add
Element** and the empty slot's menu list only the condition or reward blocks. A block links
what it is about: a quest, an item (any `items/` class), a monster or NPC, a skill
(`RoseSkill`) or a zone (`RoseZoneSettings`); drag one from the FileSystem dock. Blocks that
pick from a list (`operator`, `ability`, `variable_type` ...) show a menu. Each block's class
help (hover, or **Help** in the editor) says what it does and which blocks the server
doesn't run yet (clans, teams, Lua, monster spawn switches, delayed and zone-team steps).

The blocks are saved inside the step's file, so 1,177 steps are 1,177 small files and
opening one loads only the quests, items and steps it links.

## NPC dialogs

A dialog (`dialogs/<key>-<name>.tres`, a `RoseDialog`) is what a town NPC says. An NPC marker
in a zone scene (`RoseNpcSpot`) names it by its `key` (iROSE's CON file name, `EM01-003.con`),
which its `dialog` field shows as the resource (drag a dialog onto it to change the NPC's
dialog). `name` is only a note of who it is for. A dialog holds:

- `menus`: numbered from 0 (`RoseDialogMenu`). Menu 0 is what you see first.
- each menu's `lines` (`RoseDialogLine`, listed by how they start: "NPC: Welcome ...", "You:
  Yes ..."): `kind` Npc (something the NPC says) or Answer (a choice), the `text` (`<NAME>`
  and `<LEVEL>` are filled in, `{br}` breaks the line, `{b}` bold, `{fc=N}` a colour) and
  `next_menu`, the menu it leads to (-1: none; for an NPC line, the menu holding its
  answers).
- a line's `show_if` and an answer's `on_choose`: lists of cases (`RoseDialogCase`). The first
  case whose `conditions` all pass counts: it runs its `actions` and, in `show_if`, shows the
  line when its `show` is on. No case passing hides the line. An empty `show_if` always
  shows. The dialog's own `open_if` decides the same way whether the NPC talks at all.

Of the lines of a menu that show, the **last** NPC line is what the NPC says (put the plain
greeting first and the quest lines after it); the answers are the lines of the menu it leads
to. Conditions are `RoseDialogIf...` blocks (9 kinds: a quest step passes, you have a quest,
quest items you carry, your level or another value, the NPC's quest switch, a quest switch,
an episode, job or planet variable, a quest's switch or variable, how many quests you have);
actions are `RoseDialogDo...` blocks (5 kinds: run a quest step, open a window such as the
shop or storage, set the save point, and the clan ones that do nothing yet). A quest step
block links a `RoseQuestTrigger`, so the step it names is checked when you export. See
`wiki/rules/npc-dialogs.md` for how they run.

The iROSE dialogs were CON files with a Lua script deciding all this; `--convert-data`
turned each script into these cases, checked against the old scripts on every path
(`crates/rose-data-export/tests/dialogs_round_trip.rs`). Dialogs are only read by the game:
the export writes `dialogs.toml` and the server upload leaves it out (the server checks a
quest step again when the game asks to run it).

## Monster AI

An AI (`ai/<id>-<name>.tres`, a `RoseAi`, named after iROSE's AIP file: `23-at_sm.tres`) is how a
kind of monster behaves; a monster or NPC links it as its `ai` (several share one; empty: it
only fights back). It holds `idle_interval` (seconds between two runs of `on_idle`),
`new_target_chance` (the percent chance that a monster already fighting reacts to someone
new hitting it) and six lists of events (`RoseAiEvent`): `on_created`, `on_idle`,
`on_attack`, `on_damaged`, `on_kill` and `on_dead`. When the server runs a list, the
**first** event whose `conditions` all pass runs its `actions`, from the top. The Inspector
and the Game data table list each event by its blocks (`If FindCharacter, Random: AttackFound`,
`Always: MoveRandomly` without conditions), after its `name` if it has one, and rename it as
its blocks change.

Conditions are `RoseAiIf...` blocks (23 kinds: find a character nearby, the damage just
taken, a distance, health percent, a random roll, its own, its target's or its attacker's
values, status effects, day or night, the zone's or the world's time, and for town NPCs
another NPC's variables and the real date); actions are `RoseAiDo...` blocks (25 kinds: move,
attack, call allies, use a skill, and for town NPCs set a variable, say something in the chat
or run a quest step on itself). Distances are centimetres, except `CallAllies` (metres, as in
iROSE). Each block's class help says what it does and which ones the server doesn't run yet
(spawning, transforming, dropping items, clans, owners, `on_created`, `on_kill`, `on_dead`).

A block names a monster or NPC (`npc`), a skill (`skill`) or a quest step (`trigger`) as the
resource in the Inspector (drag one in), but stores its number or name (`npc_id`, `skill_id`,
`trigger_name`): a link there would let an AI link back to its own monster through a skill or
quest step that summons it, which Godot can't load. Items and zones are plain links. The
export checks that every number and name it stores is there. See
`wiki/rules/monster-behaviour.md` for how the server runs them.

The AI is only read by the server: the export writes `monster-ai.toml`, the upload sends it in
place of FILE_AI.STB, the AIP files and AI_S.STB, and the game client doesn't load it.

## Changing something

1. Open the resource, change the field in the Inspector, save. (Or change many in the
   Game data table, or a dialog or quest steps in the Graphs screen, below.)
2. Export, so the server and the game get it:

   ```sh
   godot --headless --path godot -- --export-data          # writes ../export/
   godot --headless --path godot -- --export-data=check    # fails when it is out of date
   ```

3. Commit the resource and `export/` together, with the wiki page of the item,
   monster, NPC, zone or rule (the wiki is the spec, see `AGENTS.md`).

A new item (or monster, NPC, drop table, shop tab) is a new resource of the right class
with an `id` no other one of that class has (for example duplicate an item in the FileSystem dock and change its id). Two items of
one class with the same id stop the export with an error naming both files. Ids are what
saved characters, shops, drop lists and quests store, so don't change the id of an item
players may have.

The export writes `export/data/items/<class>.toml` and `export/data/monsters.toml`,
`npcs.toml`, `drop-tables.toml`, `shop-tabs.toml`, `skills.toml`, `status-effects.toml`,
`job-classes.toml`, `zones.toml`, `warp-gates.toml`, `craft-recipes.toml`,
`new-characters.toml`, `sounds.toml`, `hit-sounds.toml`, `footsteps.toml`,
`effect-files.toml`, `attack-effects.toml`, `quests.toml`, `quest-triggers.toml`,
`dialogs.toml` and `monster-ai.toml` (layout in
`crates/rose-data-export/src/items.rs`, `npcs.rs`, `skills.rs`, `world.rs`, `sounds.rs`,
`effects.rs`, `quests.rs`, `dialogs.rs` and `ai.rs`), and in the same run the zone scenes' spawns and NPCs
(`export/zones/`) and the animations' hit markers (`export/motions.toml`), see
`../../export/README.md`. A link is written as the linked resource's kind and id (`"weapon/3"`,
`drop_table = 147`, `required_skill_1 = 821`, `attack_start_sound = 38`, `attack_effect = 111`),
and an audio file, an effect scene, an icon or a model as its path
(`res://assets/rose/sound/attack/sword_001.res`). The server's upload sends it (as JSON) in
place of the iROSE item, NPC (`LIST_NPC`), drop (`ITEM_DROP`), shop (`LIST_SELL`), skill
(`LIST_SKILL`), status effect (`LIST_STATUS`), job class (`LIST_CLASS`), zone (`LIST_ZONE`),
warp (`WARP`), recipe (`LIST_PRODUCT`), new character (`INIT_AVATAR`), quest (`LIST_QUEST`
and the QSD files) and AI (`FILE_AI`, the AIP files) tables, and the game
reads it from the repository (or from `export/data/` next to `ROSE.exe` in the Windows
build). Both build scripts refuse to build when it is out
of date. To list what differs from iROSE:
`cargo run --release -p rose-stdb-import -- compare /path/to/data.idx`.

## Many at once: the Game data table

The **Game data** screen at the top of the editor (next to 2D, 3D and Script; the plugin
`addons/rose_data_tools/`) shows every resource of one kind as a row and its Inspector fields
as columns, like a spreadsheet:

- **Kind**: the dropdown lists each class found in `data/` with its folder and count
  (`items/weapon (950)`, `monsters (545)`, `skills (1782)` ...).
- **Columns**: the tabs are the Inspector's groups ("Combat", "Drops" ...); the id and name
  always show. `Columns` hides single ones. Click a column name to sort by it.
- **Filter**: words to find anywhere, or a field and a value: `level>=30`, `run_speed<400`,
  `name~jelly` (contains), `drop_table=` (no link). All words must match.
- **Change a cell**: double-click it (or press Enter), type, Enter. With several rows
  selected (Shift or Ctrl click) the value goes into all of them. Tick boxes and dropdowns
  work as in the Inspector. Drop a resource from the FileSystem dock on a link cell
  (a monster's `drop_table`) to link it (in every selected row too); right-click to clear
  it.
- **Change...**: set, add to or multiply one field in the selected rows or in every shown
  row ("multiply `base_price` by 1.1 for all one-handed swords"). Whole-number fields are
  rounded.
- **Spreadsheet**: Ctrl+C copies the selected cells, right-click "Copy table" copies the
  shown rows with the column names. Change them in a spreadsheet and paste back with
  Ctrl+V: a table that starts with the column names finds each row by its id; other cells
  paste from the selected cell on. Cells that didn't change, read-only ones and ones that
  aren't a value of their column (a word in a number field) are left as they are. Links
  copy as their path (`res://data/drop_tables/147-woopie.tres`).
- **Duplicate**: copies the selected rows to new files next to them with the next free ids
  (monsters and NPCs count together), named `<id>-<name>.tres`.
- Selecting a row shows the resource in the Inspector, and a change made there shows in the
  table. Every change can be undone (Ctrl+Z). Changed rows have an orange id until saved:
  `Save` or Ctrl+S writes them. Then export as below.

## Dialogs and quest steps as a graph: the Graphs screen

The **Graphs** screen at the top of the editor (next to Game data; the plugin
`addons/rose_data_tools/graphs/`) draws one NPC dialog, or the quest steps of one folder of
`quest_triggers/`, as boxes and wires. Pick it from the two dropdowns, double-click a dialog
or step file in the FileSystem dock, or press **Open in Graphs** at the top of its
Inspector. Only that dialog or folder is loaded, a few dozen boxes a frame.

- **A dialog**: one box per menu, its lines as rows ("NPC: ...", "You: ..."). Each line's
  port on the right is wired to its `next_menu`; an answer that ends the talk is wired to
  **End**. Amber ports and wires are lines with a `show_if` (they only show when it passes).
  Under a line, chips say what its cases check and do: "If Job >= 111", "Do step 1051-01",
  "Do open Shop" (the first two cases, then "+N").
- **Quest steps**: one box per step, its conditions ("If ...") and rewards ("Then ...") as
  rows. "If it fails, try" is wired to `next_trigger`, a "run step" reward to the step it
  names. Steps of other folders are grey boxes; double-click one to open its folder.
- **Editing**: click a row or chip to edit it in the Inspector (conditions and actions are
  edited there); the box redraws when you change it. Double-click a dialog line to change
  its words. Drag a wire from a port to another box to change where it leads (drop it on
  empty space: nowhere). Right-click a line to move it up or down, add a line below, make
  it an answer or an NPC line, or delete it; right-click the background to add a menu or a
  step (a step gets a new file in the folder). Select a menu and press Delete to delete it:
  the menus after it move up a number and every line pointing at them follows; lines that
  led to the deleted menu end the talk. Menu 0 stays.
- **Find**: selects the boxes whose words, chips or names contain the text; Enter goes to
  the next. **Used by** (quest steps) lists the dialogs, steps, AIs, monsters and zone
  scenes that run the selected step, by searching the files' text (nothing is loaded);
  double-click one to open it.
- **Where boxes sit**: the graph places them in columns, in the order the talk (or the
  steps) goes. A box you drag keeps its place: it is saved as `graph_position` in that menu
  or step (only once moved; **Auto layout** forgets it). The game and `--export-data` don't
  use it, so moving boxes never changes the export.
- Every change can be undone (Ctrl+Z); `Save` or Ctrl+S writes the changed files. Then
  export as below.

## Where it came from

`--convert-data` wrote these files once from the iROSE tables (run headless, with the
client data):

```sh
godot --headless --path godot -- --convert-data --data-idx=/path/to/data.idx
```

It converts only the kinds that have no resources here yet, because the others are the
source now; `--convert-data=force` writes them all again from iROSE (keeping each file's
uid) and loses every change made since. Converted and exported, every value of all 4,172
items and the 10 grades, the 545 monsters, 117 NPCs, 265 drop tables, 83 shop tabs, 1,782
skill levels, 60 status effects and 44 job classes was the same as in iROSE
(`rose-stdb-import compare`), except one shop place: tab 98 listed `material/158`, an item
that doesn't exist (the game skipped it already), so it was left out. The same holds for the
38 zones, 57 warp gates, 472 recipes and 2 new characters (2026-10-23), except warp gate 40,
which led to zone 29, a zone without a map file (the game could never send anyone there), so
it was left out. After the conversion the new characters got the starting weapons the game
used to add in code (a Short Sword in hand, a Short Bow and 999 Wooden Arrows in the bag), so
`compare` lists them as changed from iROSE.

For zones the converter kept the LIST_ZONE rows that have a map file and the columns the game
reads; it left out the Korean name (column 0 of the text table), the underground flag
(column 0), the camera type (21), the join, kill and death triggers (22 to 24), the weather
(27), the party experience values (28, 29) and the revive zone and position (31 to 33), which
nothing in the game reads (see the backlog pages that would use them). A zone's name and
description, which iROSE keeps in `LIST_ZONE_S.STL`, are fields of the zone. Recipes are
named after the items made with them ("Iron Helm, Criker Hat"; 1 to 20 "Refining weapons from
grade 0 to 1" ...). A new character's slots are INIT_AVATAR's columns 6 to 13 (wearing),
14 to 23 (bag equipment) and the five consumable and five material places with their counts
that follow.

Each field is a value the game reads, by the name the game's code gives it. The converter
left out the columns nothing in the game reads: the Korean name (column 0 of each table),
the artists' model notes (columns 1 and 2) and a few unused columns (weapons 44, bodies 33
and 34, hands 33, heads and feet 34, vehicle parts 34, 51 and 52). Units are iROSE's
(ranges in centimetres, cooldowns in seconds). Some consumable fields came from one iROSE
column (its "data value"), so a potion also shows that number as a skill id or fuel; the
game reads only the field the item's class uses.

For monsters and NPCs the converter left out the Korean name and model notes (`LIST_NPC`
columns 0, 1, 36 and 37) and the columns the game doesn't read. The iROSE table used some
columns for different things in monsters and NPCs; each class has its own named fields for
them (a monster's `drop_table` is the column an NPC uses for its minimap icon). Drop table
rows that no monster or zone used were left out, and each drop table is named after the
monsters that use it (or "zone ..." for a zone's own table). `ITEM_DROP` column 30 was never
read by the game, so it isn't in the drop tables.

For skills, status effects and job classes the converter left out the Korean names
(column 0) and the columns the game doesn't read (the skill table's script, hit count,
area hit and casting sound columns, the status table's sounds, end effect and control
columns; the full lists are the unused `stb_column!` lines in
`crates/rose-data-irose/src/skill_database.rs` and `status_effect_database.rs`). iROSE uses
some skill columns for two things; each has a named field, so a skill shows the same number
in both: column 6 is `cast_range`, `basic_command` (basic actions) and `required_planet`
(never read), column 9 is `power` and `item_make_number` (crafting skills). Cooldowns are in
milliseconds (`cooldown_ms`, iROSE's steps of 200 ms) and motion speeds in percent. A job
class keeps at most eight jobs (class 71 lists ten in iROSE; the game always read eight).
The blank `LIST_CLASS` rows have no resource: a class number with no resource takes every
job, as a blank row did (items ask for classes 9 and 10, which are blank).

For sounds (2026-10-24) the converter kept the `FILE_SOUND` rows that something links, and of
the others the ones whose file the client has: 344 of 1,284 rows. 950 rows name a file the
client doesn't have (and nothing plays them) and were left out; sound 125 is linked by skills
but has no file, so it is kept with an empty `stream`, silent as before. The 37
`LIST_HITSOUND` rows (columns: armour, cloth, stone) became the hit sound sets, named after
the weapon kind of their sounds, and the 5 `LIST_STEPSOUND` columns the footstep sets, named
after the zones that use them. Weapon 1061 named hit sound row 131, which doesn't exist (it
played nothing), so it has no `hit_sounds`; two zone scenes had sound spots with the file
`NULL`, which now have no `stream`. The zones' music links the imported files. `compare`
shows no other difference for sounds.

For effects (2026-10-24) the converter kept the `FILE_EFFECT` rows whose file the client has
and the ones something links: 332 of 2,002 rows (the client data has 326 of the 349 files
the table names; 23 rows nothing links name files it doesn't have and were left out, and
`RAINBOW01.EFT` can't be read, so effect 1173 is kept with no `scene`, showing nothing as
before). Of the 578 `LIST_EFFECT` rows it kept the 272 that have something in them or that
something links, without the columns the game doesn't read (1, the type; 7, 12 and 14). A
monster's `create_effect_index` stays a plain number (the game doesn't show it). The effect
fields of items, monsters, skills and status effects were renamed when they became links
(`effect_id` is now a weapon's `attack_effect`, `bullet_effect_id` `bullet_effect`,
`effect_file_id` a potion's or status effect's `effect`, `hit_effect_file_id` a skill's
`hit_effect`, `die_effect_file_id` `die_effect` ...). `compare` shows no difference for
effects.

For icons and models (2026-10-24) the numbers that pointed at art became links: an item's
`icon_index` its `icon`, its `field_model_index` its `field_model`, a skill's `icon_number` and
a status effect's `icon_id` their `icon`. Worn items got a `model` (or `model_male` and
`model_female`), which iROSE took from the item's number in the part list of its kind, and
monsters and NPCs a `model`, which iROSE took from their own number in `LIST_NPC.CHR`. A link
is empty where iROSE had no model: 333 items name a ground model the list doesn't have (they
showed nothing and still don't) and three NPCs have no `LIST_NPC.CHR`
entry. An NPC's minimap and face icon and a monster's hand weapon parts stay numbers, and so
do the faces and hair of characters. `compare` shows no difference for art.

For quests (2026-10-24) the converter read `LIST_QUEST.STB` and its text table
(`LIST_QUEST_S.STL`) and every QSD file the client lists (`3DDATA/QUESTDATA`). It wrote the
224 quests that have text or a time limit or that a step names (1953, 5062 and 5063 have no
row; they got a quest with only the number), and the 1,177 steps, each in the folder of its
QSD file (`QN-002.QSD` → `qn-002/`; a step name in two files keeps the last, as the game
did). A block's numbers became links where they name something; three `SpawnMonster`
rewards named monsters that aren't in the game (354 in `3101-31`, 413 in `QP101-401`, 417 in
`QP101-501`, which spawned nothing) and were left empty. An NPC message's text, which iROSE
looks up by number in the quest string table, is written in the block. Times are seconds
(iROSE counted ticks of 10 s). `compare` shows no difference for quests.
