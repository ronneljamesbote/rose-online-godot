# Demos

Scripted runs for tests, screenshots and videos. They drive the game the way a player
would (clicks, windows, the server's reducers) and print what happens, each line starting
with `rose net demo:` (online) or `rose demo:` (offline).

They run from the Godot editor or `godot --path godot -- ...`. The Windows export leaves this
folder out (`exclude_filter` in `../export_presets.cfg`), so ROSE.exe can't run them; asked
to, it prints "demos aren't in this build".

## How it fits together

- `main.gd` calls `runner.gd` for `--net-demo` once the character is in the world, and for
  `--demo` and `--effect` once the zone has loaded.
- `runner.gd` picks the file from the demo's name, adds it under main and calls `run()`.
- `demo.gd` is the base of every demo: `main`, `online`, `options` (the command line),
  `origin` (where the character joined) and shared helpers.
- A new demo is a new file `NAME.gd` that extends `demo.gd`, plus one line in
  `runner.gd`'s `NET_DEMOS`.

## Online: `--net-demo=NAME`

| Name | File | What it does and its options |
|---|---|---|
| `square` (default), `line` | `route.gd` | Walks round the start point 4 m out, or back and forth 5 m east, forever. |
| `fight` | `fight.gd` | Fights the nearest monster until it dies, then the next; every third fight starts with a swing cancelled by a step aside. Picks up drops unless `--no-pickup`. |
| `kite` | `kite.gd` | Hits the nearest monster that fights back, then keeps stepping away (`--kite-step` metres) and prints the monster's wind-ups. Waits `--demo-wait` seconds and walks `--walk-to=X,Z;X,Z` first. |
| `walls` | `walls.gd` | Runs out `--wall-reach` metres in `--wall-directions` directions and back, printing where it stopped, then quits. |
| `warp` | `warp.gd` | Walks into the nearest warp gate, three times. |
| `shop` | `shop.gd` | Walks to the nearest store, buys the first item of each tab (`--buy=TEXT` picks what), reads skill books and scrolls, sells one item back. |
| `skills` | `skills.gd` | Puts the first and last active skills on the hotbar and uses them (a buff, then an attack) between normal attacks. |
| `talk` | `talk.gd` | Walks to the NPC matching `--npc=NAME` and answers `--answers=1,2,...`. Besides numbers: `q` prints the quest log, `talk` opens the conversation again, `deposit` and `withdraw` move the first item in or out of the bank, `pick=NAME` puts a bag item in the refine or disassemble window, `work` presses its button, `skill=NAME` uses a skill, `use=NAME`, `equip=NAME` and `unequip` use, wear and take off the weapon, `gem=NAME` sets a gem, `act=SLOT` right-clicks and `show=SLOT` prints an inventory slot (`e6` equipped weapon, `v3` vehicle arms, else a bag item name), `stats` prints attack and defence, `wait` waits six seconds. |
| `party` | `party.gd` | With `--invite=NAME` invites that player, without accepts the first invitation (after `--accept-after` seconds); `--rules=XP,ITEMS` sets the rules; then fights as `fight`. |
| `craft` | `craft.gd` | Opens the first craft skill and makes the item matching `--craft=NAME`, `--times=N` times. |
| `trade` | `trade.gd` | With `--trade-with=NAME` asks that player, without accepts the first request; puts up `--offer=ITEM` and `--zuly=N`, locks, waits `--hold` seconds and trades. |
| `pvp` | `pvp.gd` | Attacks the player `--fight=NAME` (in a PvP zone) and prints both HPs; `--leave-party` leaves the party first. |
| `chat` | `chat.gd` | After `--chat-wait` seconds says each line of `--say=LINE\|LINE` every `--say-every` seconds; `--sit-for=SECONDS` sits meanwhile and prints HP and MP. |
| `hotkey` | `hotkey.gd` | Puts Mana Bolt on hotbar key 1, equips a rod or staff, walks up to the nearest monster and presses 1 `--presses` times (default 6) 0.12 s apart, then prints how many casts started (one is right). Needs a Muse with Mana Bolt. |
| `hotbar-items` | `hotbar_items.gd` | Puts the first two consumables on hotbar keys 1 and 2, moves the first to bag slot 11, then drops it; prints the hotbar after each step (key 1 follows the item, then empties). Needs two consumables in the bag. |
| `hotbar-clear` | `hotbar_clear.gd` | Puts two basic skills on hotbar keys 1 and 2 and the first bag item on key 3, prints where each slot is on screen, then prints the hotbar whenever it changes while you (or xdotool) drag slots: dragged off the hotbar a slot empties, dropped on another slot it moves there. |
| `steps` | `steps.gd` | Runs `--steps="STEP;STEP;..."`, one action each: `wait S`, `skill ID [TARGET]`, `attack nearest`, `select NAME`, `move DX DZ`, `accept`, `waitdead NAME`, `status`, `revive save\|here`, `invite NAME`, `leaveparty`, `friend NAME`, `friendaccept`, `unfriend NAME`, `friends [open]`, `shop TITLE\|PAGE,INDEX,QTY,PRICE\|...`, `shopsetup`, `wantsearch TEXT\|QTY\|PRICE`, `shopopen TITLE`, `closeshop`, `browse NAME`, `buy NAME N QTY`, `sell NAME N QTY`, `ride NAME`, `rideaccept`, `rideleave`, `bag`, `money`, `equip PAGE INDEX`, `use PAGE INDEX`, `unequippart N`, `drive`, `inventory [close] [TAB]`, `vehicle`, `savepoint`, `me`. TARGET is `nearest`, `self`, `none`, a player's name or `of:NAME` (what that player fights). The full list is at the top of `steps.gd`. |

Two clients on one PC: `--headless ... --name=Alice --profile=alice --net-demo` in one
process and `... --name=Bob --profile=bob --weapon=bow --net-demo=line --net-log` in
another.

## Offline

- `--demo` (`offline.gd`): run, a full swing, a swing cancelled by moving, a combo, then
  quit. With `--write-movie out.avi --fixed-fps 30` Godot records the clip.
- `--effect=PATH,PATH...` (`effect.gd`): plays these EFT effects in a row in front of the
  camera, each again once it ends. `--screenshot-after=SECONDS` waits before `--screenshot`.
