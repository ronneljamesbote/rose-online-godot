---
kind: rule
id: controls
name: Controls
status: changed-from-irose
keys:
  - { key: "Left-click ground", does: "walk there" }
  - { key: "Left-click monster", does: "attack it (also an enemy player in a PvP zone)" }
  - { key: "Left-click NPC", does: "walk over and talk" }
  - { key: "Left-click player", does: "target them; opens their shop if they have one" }
  - { key: "Left-click item on the ground or its label", does: "walk over and pick it up" }
  - { key: "Right-click player", does: "menu: party, trade, friend, shop, offer a ride" }
  - { key: "Right-drag", does: "turn the camera" }
  - { key: "Mouse wheel", does: "zoom the camera (2 to 60 m), or the minimap when over it" }
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
  - { key: "O", does: "options (interface, world labels, sound)" }
  - { key: "Esc", does: "close the top window, the dialog, or leave repair mode" }
  - { key: "Delete", does: "drop the selected bag item" }
  - { key: "Alt", does: "hide ground item labels while held (or show them, as set in Options)" }
camera_zoom_m: { min: 2, max: 60, start: 12 }
window_size_percent: { min: 75, max: 150 }
window_opacity_percent: { min: 40, max: 100 }
item_label_range_m: 30
source:
  code:
    - godot/scripts/main.gd (_unhandled_input)
    - godot/scripts/orbit_camera.gd
    - godot/scripts/online.gd (talk_to, select, pickup, open_player_menu)
    - godot/scripts/conversation_window.gd
    - godot/scripts/inventory_window.gd (Slot._gui_input, activate, _drop_selected)
    - godot/scripts/hotbar.gd
    - godot/scripts/store_window.gd
    - godot/scripts/bank_window.gd
    - godot/scripts/chat_window.gd
    - godot/scripts/ui/options_window.gd
    - godot/scripts/ui/ui_window.gd
    - godot/scripts/ui/theme_builder.gd (tooltips)
    - godot/scripts/ui/world_overlay.gd
    - godot/scripts/ui/hud/minimap.gd (to_map)
    - godot/scripts/ui/zone_map.gd
    - godot/scripts/ui/hud/quest_tracker.gd
  data: ""
---
# Controls

The game is played with the mouse, with keys for windows and quick actions. The full list
is in the `keys` table above.

## Mouse

- **Left-click the ground** to walk there.
- **Left-click a monster** to attack it. In a PvP zone, clicking a player you may fight
  attacks them too (see [[rules/pvp|PvP]]).
- **Left-click a town NPC** to walk over (you talk from within 3 m) and open its dialog
  (see [[rules/npc-dialogs|NPC dialogs]]).
- **Left-click another player** to target them, for example for a heal. If they have a
  shop open, its window opens (see [[rules/personal-shops|Personal shops]]).
- **Left-click an item on the ground**, or its name label, to walk over and pick it up.
- **Right-click another player** for a menu: invite to party, trade, add as friend, look
  at their shop, or offer a ride on your cart.
- **Hold the right button and drag** to turn the camera. **Mouse wheel** zooms in and out
  between 2 m and 60 m from your character (12 m at the start). Over the minimap the wheel
  zooms the minimap.

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
| Esc | close the top window |
| Delete | drop the selected bag item on the ground |
| Alt | item labels on the ground: held, it hides them (or shows them, if Options says "Only while Alt is held") |

While a dialog is open, 1 to 9 pick its answers and Esc ends it.

## Items and the hotbar

- In the inventory, **left-click** selects an item; **right-click** or **double-click**
  uses a consumable or equips gear (on a worn item it takes it off). Right-clicking a gem
  sets it in the first worn item with a free socket.
- Drag items to move them between bag slots.
- Drag a skill or a bag item onto the **hotbar** to put it there. Drag between hotbar slots
  to swap them. Right-click a hotbar slot to use it, and Shift + right-click to empty it.
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
- Clicking a quest in the quest tracker opens the quest window.
- The **minimap** (top right) and the **zone map** (M) always have north up. They show you
  as an arrow, town NPCs as green dots (named on the zone map), party members in blue and,
  on the minimap, monsters in red. The map image is the zone's own picture, the same one
  each zone page shows, with 2.5 m per pixel; its top left corner is the zone page's map
  `left` and `top`.
- **Options** (O) has three tabs:
  - **Interface**: the theme (four built-in themes, or your own theme files in the
    `themes` folder next to the game), interface size (75% to 150%), window opacity (40%
    to 100%), blur behind windows, lock windows in place, and reset layout. Window
    opacity does not fade tooltips and menus: they are solid on every theme (a theme
    file can change that with `opacity` under `[tooltip]`).
  - **World labels**: your own name tag, chat bubbles, damage numbers, and whether item
    labels on the ground show always or only while Alt is held. Labels show for items
    within 30 m.
  - **Sound**: music and sound effect volume.

## Playing offline

Without signing in you can walk around: left-click to move, S to stop, Space swings the
sword, O opens the sound volume window.

## Changed from iROSE

- The interface is new: the themes, movable and resizable windows and the options above
  are not part of the iROSE client.
