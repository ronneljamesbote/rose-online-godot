---
kind: rule
id: controls
name: Controls
status: changed-from-irose
keys:
  - { key: "Left-click ground", does: "walk there; a glowing crystal drops onto the spot" }
  - { key: "Hold left button on the ground", does: "keep running toward the cursor" }
  - { key: "Left-click monster", does: "attack it (also an enemy player in a PvP zone); with Double-click to attack, select it, and attack on a double click or a second click" }
  - { key: "Left-click NPC", does: "walk over and talk" }
  - { key: "Left-click player", does: "target them; opens their shop if they have one" }
  - { key: "Left-click item on the ground or its label", does: "walk over and pick it up" }
  - { key: "Right-click player", does: "menu: party, trade, friend, shop, offer a ride" }
  - { key: "Right-drag", does: "turn the camera" }
  - { key: "Mouse wheel", does: "zoom the camera (2 to 45 m), or the minimap when over it" }
  - { key: "Space", does: "attack the nearest monster" }
  - { key: "S", does: "stop" }
  - { key: "Z", does: "pick up the nearest item" }
  - { key: "X", does: "sit down or stand up" }
  - { key: "1-8", does: "use hotbar slot 1 to 8" }
  - { key: "1-9", does: "pick a dialog answer while talking to an NPC" }
  - { key: "Enter", does: "start typing in chat; Enter again sends" }
  - { key: "C", does: "character window" }
  - { key: "I", does: "inventory" }
  - { key: "K", does: "skills" }
  - { key: "Q", does: "quests" }
  - { key: "P", does: "party" }
  - { key: "F", does: "friends" }
  - { key: "M", does: "zone map" }
  - { key: "O", does: "options (interface, world labels, gameplay, graphics, sound)" }
  - { key: "Log out button (menu bar)", does: "log out after 15 seconds; not during a fight" }
  - { key: "Esc", does: "close the top window, the dialog, or leave repair mode; with nothing open, drop the target" }
  - { key: "Delete", does: "drop the selected bag item" }
  - { key: "Alt", does: "hide ground item labels while held (or show them, as set in Options)" }
camera_zoom_m: { min: 2, max: 45, start: 12 }
window_size_percent: { min: 75, max: 150 }
window_opacity_percent: { min: 40, max: 100 }
cursors:
  - { over: "the ground, buildings, a window", cursor: "blade with a blue gem (classic RES/DEFAULT.CUR)" }
  - { over: "a monster, or a player you may fight in PvP", cursor: "crossed blades with a red-gem crest (classic RES/ATTACK.CUR)" }
  - { over: "a town NPC", cursor: "speech bubble (classic RES/NPC.CUR)" }
  - { over: "another player", cursor: "blade with a gold ring (classic RES/USER.CUR)" }
  - { over: "an item on the ground or its label", cursor: "open gold gauntlet (classic RES/ITEM_PICK.CUR)" }
  - { over: "a bag, storage, skill or hotbar slot with something in it", cursor: "pointing gold gauntlet (classic RES/ITEM_INVEN.CUR)" }
  - { over: "anywhere while dragging an item or a skill", cursor: "glowing blue pointing hand (classic RES/ITEM_INVEM.CUR)" }
  - { over: "anywhere while repairing", cursor: "hammer (classic RES/REPAIR.CUR)" }
  - { over: "the world while the right button is held", cursor: "four-way arrows with a blue gem (classic RES/WHEEL.CUR)" }
cursor_size_px: { small: 32, normal: 40, large: 48, huge: 64, default: large }
cursor_styles: [New, Classic]
sticky_target_default: false
target_ring:
  monster: red
  npc: green
  player: blue (other players, party members, summons)
  pvp_enemy_player: red
  size: the target's model width
  motion: turns slowly and pulses
attack_click: [Click to attack, Double-click to attack]
attack_click_default: Click to attack
item_label_range_m: 30
source:
  code:
    - godot/scripts/main.gd (_unhandled_input, _steer)
    - godot/scripts/orbit_camera.gd
    - godot/scripts/online.gd (move_to, click_ground, steer_to, talk_to, select, clear_target, pickup, open_player_menu)
    - godot/scripts/target_marker.gd
    - godot/scripts/conversation_window.gd
    - godot/scripts/inventory_window.gd (Slot._gui_input, Slot._notification, activate, _drop_selected)
    - godot/scripts/hotbar.gd (dropped, dragged_off)
    - module/src/items.rs (move_item)
    - module/src/character.rs (Player::set_inventory)
    - godot/scripts/store_window.gd
    - godot/scripts/bank_window.gd
    - godot/scripts/chat_window.gd
    - godot/scripts/ui/options_window.gd
    - godot/scripts/ui/hud/menu_bar.gd
    - godot/scripts/ui/ui_window.gd
    - godot/scripts/ui/theme_builder.gd (tooltips)
    - godot/scripts/ui/hud/hover_tooltip.gd
    - godot/scripts/ui/hint_label.gd
    - godot/scripts/character_window.gd (HINTS)
    - godot/scripts/ui/world_overlay.gd
    - godot/scripts/ui/hud/minimap.gd (to_map)
    - godot/scripts/ui/zone_map.gd
    - godot/scripts/ui/hud/quest_tracker.gd
    - godot/scripts/ui/hud/player_frame.gd
    - godot/scripts/ui/hud/portrait_camera.gd
    - godot/scripts/ui/game_cursor.gd
    - godot/rust/src/cursor.rs
    - godot/data/interface/interface.tres (the cursors and their click points)
    - godot/assets/ours/cursors/ (the painted cursor pictures)
    - godot/scripts/ui/version_line.gd
    - godot/scripts/build_info.gd
  data: ""
---
# Controls

The game is played with the mouse, with keys for windows and quick actions. The full list
is in the `keys` table above.

## Mouse

- **Left-click the ground** to walk there. A glowing cyan crystal (iROSE's marker,
  `3DDATA/EFFECT/FLAT_POINT_01.EFT`) drops onto the spot. It goes the moment you
  arrive or stop, or when a click sends you somewhere else (another spot, a monster, an NPC
  or an item); otherwise it fades out by itself after about a second and a half.
- **Hold the left button** after clicking the ground to keep running toward the cursor;
  move the mouse to steer. Holding only steers: passing over a monster, NPC or item does
  not attack, talk or pick up. When you let go, the crystal drops where you will stop.
- **Walking drops your target** (a click or holding the button on the ground), as in
  iROSE: skills then need a new target. With **Sticky target** on (Options, Gameplay tab;
  off by default) walking keeps it: the attack stops, but the target stays selected, its
  frame stays up and skills still go to it. Clicking another monster or player changes it,
  and Esc with no window open drops it.
- **A glowing ring on the ground** marks your target, sized to it and lying on the ground
  under it (on slopes, stairs and bridges too). Its colour says what it is: red a monster
  (or a player you may fight in a PvP zone), green an NPC, blue another player or a party
  member. It turns slowly and pulses gently, and goes when the target is dropped, dies or
  leaves.
- **Left-click a monster** to attack it. In a PvP zone, clicking a player you may fight
  attacks them too (see [[rules/pvp|PvP]]). With **Double-click to attack** (Options,
  Gameplay tab; the default is Click to attack) the first click only selects the monster:
  its frame comes up and skills go to it, but you don't walk or swing. Double-clicking it,
  or clicking it again while it is selected, attacks. A monster that is already your
  target (one you were fighting, or one that attacked you) is attacked by a single click.
  NPCs, players and items work the same in both modes.
- **Left-click a town NPC** to walk over (you talk from within 3 m) and open its dialog
  (see [[rules/npc-dialogs|NPC dialogs]]).
- **Left-click another player** to target them, for example for a heal. If they have a
  shop open, its window opens (see [[rules/personal-shops|Personal shops]]).
- **Left-click an item on the ground**, or its name label, to walk over and pick it up.
- **Right-click another player** for a menu: invite to party, trade, add as friend, look
  at their shop, or offer a ride on your cart.
- **Hold the right button and drag** to turn the camera. **Mouse wheel** zooms in and out
  between 2 m and 45 m from your character (12 m at the start). Over the minimap the wheel
  zooms the minimap. Buildings, rocks and the ground itself pull the camera in closer
  instead of letting it pass through them, so it never looks at the world from below, and
  a click always lands on the top of the ground you see.

### The mouse cursor

The game has its own painted cursors, in the same shapes as ROSE's (the pictures are in
`godot/assets/ours/cursors/`, 128 pixels; the interface settings in
`godot/data/interface/interface.tres` link each one and set the pixel it clicks at), and
the cursor shows what a left click will do (the `cursors` table above):

- the **blade with a blue gem** over the ground, buildings and windows;
- **crossed blades** over a monster, or a player you may fight in a PvP zone;
- a **speech bubble** over a town NPC;
- the **blade with a gold ring** over another player;
- an **open gold gauntlet** over an item on the ground or its label;
- the **pointing gold gauntlet** over a bag, storage, skill or hotbar slot that holds
  something you can drag, and the **glowing blue hand** while you drag it;
- a **hammer** everywhere while repairing (an NPC's repair menu or a hammer in use);
- **four-way arrows** while the right button is held to turn the camera.

The blades click at their tip, the hands at the fingertip, the hammer at its head's face,
the bubble at its smallest bubble and the arrows at their centre.

Window title bars and corner grips keep the system's move and resize arrows. Options
(**Cursor Size**) can turn the game cursor off and set how wide it is drawn: Small (32
pixels), Normal (40), Large (48, the default) or Huge (64); the pictures are scaled
smoothly. **Cursor Style** picks New (the painted cursors) or Classic (the client's own
32 pixel cursor files, as iROSE drew them, enlarged in whole steps so they stay sharp:
once at Small and Normal, twice at Large and Huge). Without the cursor pictures the
system cursor stays.

## Keys

| Key | What it does |
| --- | --- |
| Space | attack the nearest monster |
| S | stop moving |
| Z | pick up the nearest item |
| X | sit down or stand up (see [[rules/recovery\|Recovery]]) |
| 1 to 8 | use the skill or item in that hotbar slot |
| Enter | type in chat; Enter sends |
| C | character window (stats, see [[rules/stats\|Stats]]) |
| I | inventory |
| K | skills (see [[rules/skills\|Skills]]) |
| Q | quests (see [[rules/quests\|Quests]]) |
| P | party (see [[rules/party\|Party]]) |
| F | friends (see [[rules/friends\|Friends]]) |
| M | zone map |
| O | options |
| Log out (menu bar) | log out after 15 seconds, not during a fight (see [[rules/logging-out\|Logging out]]) |
| Esc | close the top window; with none open, drop the target |
| Delete | drop the selected bag item on the ground |
| Alt | item labels on the ground: held, it hides them (or shows them, if Options says "Only while Alt is held") |

While a dialog is open, 1 to 9 pick its answers and Esc ends it.

## Items and the hotbar

- In the inventory, **left-click** selects an item; **right-click** or **double-click**
  uses a consumable or equips gear (on a worn item it takes it off). Right-clicking a gem
  sets it in the first worn item with a free socket.
- Drag items to move them between bag slots.
- Drag a skill or a bag item onto the **hotbar** to put it there. Drag between hotbar slots
  to swap them. Right-click a hotbar slot to use it. To empty a slot, drag it off the
  hotbar and let go anywhere else (the world, a window, an empty spot), or Shift +
  right-click it; only the shortcut goes, the skill or item itself stays where it was.
  A bag item's shortcut follows the item when you move it to another bag slot, and empties
  once the item leaves the bag (dropped, used up, sold, stored, traded or worn).
  Each filled slot shows its key (1 to 8) in the top-left corner, stacked items show their
  count, and a skill that is cooling down shows a dark sweep and the seconds left.
- While another window is open, right-clicking a bag item acts on it:

| Window open | Right-click | Shift + right-click |
| --- | --- | --- |
| NPC store | sell one | sell the whole stack |
| Storage | store the whole stack | store one |
| Trade | offer the whole stack | offer one |
| Refining or disassembly | pick the item | |

In the store, right-click buys one and Shift + right-click buys ten (see
[[rules/shops|Shops]]). In storage, right-click takes the whole stack back and Shift +
right-click takes one (see [[rules/bank|Bank]]).

## Chat

Press Enter, type and press Enter to send. The first character picks who hears it (see
[[rules/chat|Chat]]):

| Start with | Goes to |
| --- | --- |
| nothing | players nearby |
| `!` | shout to the whole zone |
| `#` | your party |
| `@name ` | a whisper to that player |
| `/r ` | a whisper back to whoever whispered you last |

Esc or clicking outside the chat box stops typing.

## Windows and the interface

- Every window has a title bar to drag it, a minimise and a close button, and a corner
  grip to resize it from 75% to 150%. Windows and HUD pieces snap to the screen edges and
  remember where they were and whether they were open.
- Pointing at a stat or value in the character window (C), or at the stat and skill
  points, shows a tooltip saying what it does and what one more point of a stat adds.
- The **version line** at the very top left names the client and the server you are on:
  their mode (DEV or PRODUCTION) and version (the git commit they were built from), for
  example "DEV · client 3cad34b · server 3cad34b". It shows on the start screen ("server
  not connected"), the loading screen and in game, and can't be clicked. When the server's
  commit differs from the client's, the server part is amber; when the modes differ, both
  are named ("DEV client 3cad34b · PRODUCTION server 9ac7031"). Once a zone is loaded the
  line ends with the zone and how the client built it: "Zone 23 Breezy Hills · Godot scene"
  for a zone made from its Godot scene, "Zone 2 City of Junon Polis · old loader" for one
  read from the iROSE map files. A zone that has a scene but was read from the map files
  anyway says why in amber: "old loader (assets missing)" when the converted assets were
  not imported into the client. The dev client is the one for testing, with the
  [[rules/gm-commands|GM commands]].
- The **player frame** (top left, under the version line) shows your character's head in the round portrait, with
  your level under it. The camera follows the head, so it stays in the middle of the
  circle, and the picture shows your real face, hair and gear. When you take damage the
  portrait flashes red and shakes. Options can make it **Live** (the idle motion plays;
  the default) or **Static** (a still picture, taken again when your look changes).
- Clicking a quest in the quest tracker opens the quest window.
- Pointing at a monster, a town NPC or an item on the ground (or its label) shows a tooltip
  in the bottom right corner of the screen, above the menu: a monster's level and life, an
  NPC's name and whether it has a store, an item's stats and whether you may pick it up.
- The **minimap** (top right) and the **zone map** (M) always have north up. They show you
  as an arrow, town NPCs as green dots (named on the zone map), party members in blue and,
  on the minimap, monsters in red. The map image is the zone's own picture, the same one
  each zone page shows, with 2.5 m per pixel; its top left corner is the zone page's map
  `left` and `top`.
- **Options** (O) has five tabs:
  - **Interface**: the theme (four built-in themes, or your own theme files in the
    `themes` folder next to the game), interface size (75% to 150%), window opacity (40%
    to 100%), blur behind windows, the character portrait (Live or Static), Cursor Size
    (the game's mouse cursor on or off, and its size), Cursor Style (New or Classic), lock windows in place, and reset
    layout. Window opacity fades tooltips and menus too, along with the windows and HUD
    panels (their drop shadow and the light line along their top edge fade with them).
  - **World labels**: your own name tag, chat bubbles, damage numbers, and whether item
    labels on the ground show always or only while Alt is held. Labels show for items
    within 30 m.
  - **Gameplay**: Click to attack or Double-click to attack (Click to attack by default),
    and Sticky target (walking keeps your target; off by default).
  - **Graphics**: Shadows: Off, Low (shadows drawn up to 80 m away), Medium (120 m, the
    default) or High (200 m, softer edges). Off keeps the sun's shading without shadows.
  - **Sound**: music and sound effect volume.

## Playing offline

Without signing in you can walk around: left-click to move, S to stop, Space swings the
sword, O opens the options (sound volume and shadows).

## Changed from iROSE

- The interface is new: the themes, movable and resizable windows and the options above
  are not part of the iROSE client. So is the version line.
- Light and shadows: iROSE draws shadows baked into the maps (lightmaps) and lights
  everything from one fixed side. Here a real sun moves with the zone's time of day (the
  moon at night) and casts shadows from buildings, trees, the ground's hills, players,
  monsters, NPCs and carts; surfaces turned away from it are darker. The colours of each
  time of day are iROSE's (from the zone's sky). Only things in your own 160 m block and
  the 8 around it cast shadows, and small decorations (grass, flowers, small rocks) only
  within 40 m of your block. The colours the lightmaps gave the world are kept without the
  lightmaps' shadows: the ground's colour (Zant's blue canal, Birth Island's sea floor) and
  one warm colour per part of each building and decoration.
- The ring under the target is new.
- Sticky target is new; with it off, walking drops the target as in iROSE. Double-click
  to attack is new too; Click to attack is iROSE's way. Esc dropping
  the target is new too.
- The cursors are new pictures in iROSE's shapes (iROSE's own are the Classic style),
  chosen the same way, except: pushing the mouse against the left
  or right screen edge does not turn the camera (so the edge arrows `LEFT.CUR` and
  `RIGHT.CUR` are not used); the gold and blue hands show over items you can drag and
  while dragging (iROSE never used them); the cursor can be enlarged and turned off; and the hands and the bubble click
  at the fingertip and the bubble's tail instead of the top-left corner.
