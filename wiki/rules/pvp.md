---
kind: rule
id: pvp
name: PvP
status: changed-from-irose
pvp_states:
  - { value: 0, meaning: "No PvP: players can't hurt each other" }
  - { value: 1, meaning: "Everyone except your clan (no clans yet, so everyone)" }
  - { value: 2, meaning: "Everyone except your party" }
  - { value: 3, meaning: "Everyone" }
  - { value: 11, meaning: "Clan war zone: no fighting between players yet" }
pvp_zones:
  - { zone: "[[zones/5-junon-cartel|Junon Cartel]]", pvp: "everyone except your party" }
  - { zone: "[[zones/6-crusader-training-camp|Crusader Training Camp]]", pvp: "everyone except your party" }
  - { zone: "[[zones/8-lion-s-plains|Lion's Plains]]", pvp: "everyone except your party" }
  - { zone: "[[zones/9-zone-9|Zone 9]]", pvp: "everyone except your party" }
  - { zone: "[[zones/11-junon-clan-field|Junon Clan Field]]", pvp: "everyone (except your clan)" }
  - { zone: "[[zones/56-forgotten-temple-b1|Forgotten Temple B1]]", pvp: "everyone (except your clan)" }
  - { zone: "[[zones/59-luna-clan-field|Luna Clan Field]]", pvp: "everyone (except your clan)" }
  - { zone: "[[zones/15-zone-15|Zone 15]]", pvp: "clan war zone (no fighting yet)" }
revive_shield_s: 30
xp_lost_when_killed_by_player: 0
xp_for_killing_a_player: 0
source:
  code:
    - module/src/pvp.rs (zone_pvp, players_hostile, can_attack)
    - module/src/lib.rs (attack, combat tick, deal_damage, kill)
    - module/src/skills.rs (target checks, controller)
    - module/src/death.rs (player_died, revive, is_shielded)
    - module/src/shop.rs (open_shop)
    - godot/rust/src/net.rs (zone_pvp, is_enemy_player)
    - godot/scripts/ui/world_overlay.gd
    - godot/scripts/ui/hud/minimap.gd
  data: LIST_ZONE.STB column 18 (PvP state)
---
# PvP

Players can fight each other only in the zones marked for PvP. Everywhere else players
can't hurt each other, and attacks or harmful skills on another player are refused.

## Which zones allow PvP

Each zone has a PvP state in its row of LIST_ZONE.STB:

- **0**: no PvP. This is most zones, including every town.
- **2, everyone except your party**: you can attack any player who isn't in your party.
- **1, everyone except your clan**: clans aren't in the game yet, so everyone can attack
  everyone.
- **3, everyone**: everyone can attack everyone. No zone uses this value today.
- **11, clan war zone**: no fighting between players yet (clan wars aren't in the game), but
  the zone still counts as a PvP zone for the other rules on this page.

The zones with PvP are listed in the data block above. Zone pages show the same value as
their `pvp` field. The minimap shows "PvP" next to the zone name in any of these zones, and
the name tags of players you may attack turn red.

## Fighting other players

- Attacks and skills against a player use the same [[rules/combat|combat]] formulas as
  against a monster. There is no PvP damage reduction.
- Your summons count as you: they can attack the players you can attack, and players can
  attack your summons where they could attack you.
- Skills that only work on allies (heals, buffs) can't be used on a player who is your
  enemy here, and skills for your party only work on party members.
- Hidden players can't be picked as a target by their enemies, unless the skill reveals
  them (see [[rules/status-effects|Status effects]]).
- An attack on a player stops when they die, leave the zone or join your party.
- Being hit by a player wears your gear the same as being hit by a monster
  ([[rules/durability|Durability]]), and damage shields send back part of a player's hit.

## Dying in PvP

- Both players see "*winner* defeated *loser*".
- Dying to a player costs **no experience**. Only a death to a monster costs experience
  (see [[rules/death|Death]]). A summon's kill counts as its owner's, so it costs nothing
  either.
- Killing a player gives **no experience** and no drops. Players drop nothing when they die.
- After getting up in a PvP zone (by choice or after waiting), you can't be hurt for
  **30 seconds**.

## Other rules in PvP zones

- [[rules/personal-shops|Personal shops]] can't be opened in a PvP zone (including the clan
  war zone).

## Changed from iROSE

- Clans don't exist yet, so "except your clan" zones let everyone fight, and the clan war
  zone has no fighting.

> Open question: Zone 15 (PvP state 11) shows "PvP" on the minimap and blocks personal shops, but players can't fight there. Should it count as a PvP zone at all until clan wars exist?
