---
kind: rule
id: logging-out
name: Logging out
status: changed-from-irose
logout_s: 15
fight_s: 10
linger_s: 60
cancels_logout:
  - moving
  - attacking
  - using a skill (sitting, emotes and passive skills don't count)
  - hitting anyone, or being hit or missed by anyone
  - the Cancel button
source:
  code:
    - module/src/logout.rs (request_logout, cancel_logout, in_fight, mark_fight, cancel_for, start_linger, end_linger, tick)
    - module/src/lib.rs (client_connected, client_disconnected, leave_world, deal_damage, move_to_as, attack_as)
    - module/src/skills.rs (cast_skill)
    - godot/scripts/online.gd (logout panel, logged_out)
    - godot/scripts/main.gd (_on_logged_out)
    - godot/scripts/ui/hud/menu_bar.gd
---
# Logging out

Press **Log out** in the menu bar (bottom right, the last button). Your character stands
still and a countdown shows at the top of the screen: after **15 seconds** the character
leaves the world and the game goes back to the start screen.

## Not during a fight

You can't start logging out while you are in a fight. You are in a fight when:

- you hit someone, or someone hit or missed you, in the last **10 seconds** (monsters and,
  in PvP zones, players); or
- a monster has you as its target.

The game then says "You can't log out during a fight".

## What cancels it

During the countdown your character must stay out of trouble. The logout stops, with a
message saying why, when you:

- move,
- attack,
- use a skill (sitting, emotes and passive skills don't count),
- hit anyone, or anyone hits or misses you,
- or press **Cancel**.

Press **Log out** again to start a new countdown.

## Crashes and lost connections

If the game closes without logging out (it crashes, the connection drops, or you close the
window), your character **stays in the world for 60 seconds**. It stands still and no
longer attacks, but monsters and players can still hit it, and it can fall. After 60
seconds it leaves the world like a finished logout. So closing the game is not a way out of
a fight.

Connecting again within those 60 seconds takes the same character back over, where it
stands, with the HP it has left.

A logout countdown that was running when the connection dropped keeps going: if nothing
cancels it, the character leaves when it ends, before the 60 seconds are up.

## When the character leaves

Where it stands, its HP and MP are kept for the next time. A character that leaves while
fallen first gets up at the zone's revive point (see [[rules/death|Death]]). A trade in
progress is cancelled, and friends see you go offline.

## Changed from iROSE

iROSE logs out after a short countdown and takes a dropped character out of the world
straight away. Here the countdown is 15 seconds, it can't start during a fight, and a
dropped character stays for 60 seconds.
