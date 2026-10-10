---
kind: backlog
id: clan-house
name: Clan house
status: not-in-game-yet
summary: A grade 3 clan rents a private clan house (zone 15) for 1,000,000 Zulie per 30 days, reached from Junon Polis or Eucar
npcs:
  - "[[npcs/1115-clan-owner-burtland|Clan Owner Burtland]]"
  - "[[npcs/1172-akram-ambassador-eliot|Akram Ambassador Eliot]]"
  - "[[npcs/1162-clan-base-camp-soldier-jason|Clan Base Camp Soldier Jason]]"
  - "[[npcs/1161-clan-base-camp-manager-kushard|Clan Base Camp Manager Kushard]]"
source:
  data: QC001.QSD triggers Clan-031 to Clan-036; LIST_SKILL.STB rows 3431-3432; LIST_ZONE.STB row 15 (Agit01, PvP state 11); LIST_STRING.STL string 372
  reference: iROSE 129_129en client data; NPC dialogs of Burtland, Eliot and Jason
---
# Clan house

The clan house ("clan agit") is [[zones/15-zone-15|zone 15]], a small private map for one
[[backlog/clans|clan]].

## How it works in iROSE 129

### Renting it

The master of a grade 3 clan (see
[[backlog/clan-grades-and-skills|Clan grades, points and skills]]) learns the clan skill
[[skills/3431-clan-house|Clan House]] at [[npcs/1115-clan-owner-burtland|Burtland]]:

| Check (trigger `Clan-031`) | Value |
| --- | --- |
| In a clan | yes |
| Position | Master (6) |
| Clan grade | 3 or higher |
| Money | 1,000,000 Zulie, taken |

The skill lasts **30 days** (Burtland's dialog; the skill's 4320 in 10-minute units). It
can only be learned again after it runs out ("You can only relearn this skill once the 30
day period has elapsed", trigger `Clan-032`).

### Going in and out

Any member of a clan that has the skill can enter:

- From [[npcs/1115-clan-owner-burtland|Burtland]] in Junon Polis: trigger `Clan-033` sets
  union variable 7 to 1 and moves you to zone 15.
- From [[npcs/1172-akram-ambassador-eliot|Akram Ambassador Eliot]] in the Magic City of the
  Eucar: trigger `Clan-034` sets union variable 7 to 2 and moves you to zone 15.

Both teleports use position (0, 0), which means the zone's start point.

Inside, [[npcs/1162-clan-base-camp-soldier-jason|Clan Base Camp Soldier Jason]] takes you
back out ("I'd like to leave the Clan House"): if variable 7 is 1 you go to Junon Polis at
(5596.20, 5034.28), if it is 2 to the Magic City of the Eucar at (5357.32, 4991.29); the
variable is reset to 0 (triggers `Clan-035`, `Clan-036`).

[[npcs/1161-clan-base-camp-manager-kushard|Clan Base Camp Manager Kushard]] stands at nine
spots in the house and offers a shop, storage, repair, refining, disassembly and
appraisal.

### Inside the house

- Only members of the clan that owns the house should be in it; each clan has its own copy.
- Some actions are blocked: "You cannot use this in the Clan Base Camp." (string 372).
- The zone's PvP state is 11, a value no other zone uses.

> Open question: how the server gives each clan its own copy of zone 15 (an instance per
> clan or one shared map), which actions string 372 blocks, and what PvP state 11 means (a
> clan war state is the usual guess). See the open question on [[rules/pvp|PvP]].

## What our game does today

Zone 15 exists with its NPCs and can be reached only by GM means; the entry and exit
dialogs fail because there are no clans and no clan skills. PvP state 11 shows as PvP but
nobody can fight there.

## Building it

- **Server**: clan skill 3431 with a 30 day expiry; the `HasClanSkill` check; one zone 15
  instance per clan (or a rule that only that clan's members can be in it); block the
  string 372 actions there; union variable 7 as already used by quests.
- **Client**: nothing new beyond the clan pages; show the remaining rent time in the clan
  skills tab.
- **Data**: none to change.
