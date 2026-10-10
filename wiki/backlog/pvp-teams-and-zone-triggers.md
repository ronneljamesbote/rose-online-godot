---
kind: backlog
id: pvp-teams-and-zone-triggers
name: PvP teams and zone triggers
status: not-in-game-yet
summary: Players carry a team number that decides who can fight whom, and zones run quest triggers when a player enters, kills or dies there
npcs:
  - "[[npcs/1090-righteous-crusader-gawain|Righteous Crusader Gawain]]"
source:
  data: LIST_ZONE.STB columns 22-24 (join, kill and death trigger); PvP-A.qsd, PvP13-01.qsd (PvP1301-340), PvP10.qsd, QC001.qsd (Clan-003), QP201.qsd (2051-10 to 2051-14); ability 34 (team number); LIST_STRING.STL string 591
  reference: iROSE 129_129en client data; QSD reward SetTeamNumber and TriggerForZoneTeam in crates/rose-file-readers/src/qsd.rs; module/src/pvp.rs and module/src/quests.rs
---
# PvP teams and zone triggers

Two engine features that the iROSE PvP content is built on.

## How it works in iROSE 129

### Team numbers

Every character and monster has a team number (ability 34). In a PvP zone two players are
enemies when their team numbers differ (and the zone's PvP state allows fighting at all;
see [[rules/pvp|PvP]]). Normal players are team 2. Quests change it:

- `SetTeamNumber` with source **Unique**: a team of your own, so everyone else, party
  members included, is an enemy.
- Source **Clan**: your clan's number, so your clan are allies.
- Source **Party**: your party's number.
- Setting ability 34 directly (faction wars use 11 and 13).

Spawned monsters can be given a team too (`SpawnMonster` has a team number), which is how
war crystals belong to a side. In PvP zones you can only form a party with your own team
("Only team members can form a party in PVP Zones.", string 591).

### Zone triggers

LIST_ZONE.STB gives each zone up to three quest triggers, run on the player:

| Column | When it runs |
| --- | --- |
| 22 | the player enters the zone |
| 23 | the player kills another player in the zone |
| 24 | the player dies in the zone |

Values in the 129 data:

| Zones | Join | Kill | Death |
| --- | --- | --- | --- |
| Most normal zones and towns | `PvP1301-340` (team back to 2) | | |
| [[zones/5-junon-cartel\|Junon Cartel]] | `PvP1301-320` (Cartel war team) | | |
| [[zones/6-crusader-training-camp\|Crusader Training Camp]] | `PvP-A-010` (unique team) | | |
| [[zones/8-lion-s-plains\|Lion's Plains]], [[zones/9-zone-9\|zone 9]] | `PvP10-331` (war team) | `PvP10-341` | `PvP10-351` |
| [[zones/11-junon-clan-field\|Junon Clan Field]], [[zones/59-luna-clan-field\|Luna Clan Field]] | `Clan-003` (clan team) | | |
| [[zones/56-forgotten-temple-b1\|Forgotten Temple (B1)]] and B2 | `2051-10` | `2051-13` | `2051-14` |

So:

- **Training ground** ([[npcs/1090-righteous-crusader-gawain|Gawain]]: "everyone except you
  is your enemy"): you pay 1,000 Zulie (levels 50-69) or 2,000 Zulie (70 and up), get two
  Vital Water potions and are moved to Crusader Training Camp; its join trigger gives you a
  unique team, so your party can fight you too.
- **Forgotten Temple**: entering with the quest and a Resurrection Spellbook sets your team
  (your party's team when you are in a party, otherwise your own) and your get-up point.
  A player kill gives a Proof of Thief Extermination and adds 1 to quest variable 9; dying
  takes one Proof and one Resurrection Spellbook and takes 1 from the variable.
- **Faction wars**: see [[backlog/faction-wars|Faction Wars]].

### Running a trigger for a whole team

The quest reward `TriggerForZoneTeam` (zone, team, trigger) runs a trigger on every player
in that zone with that team number. Faction war referees use it to reward the winners and
losers.

## What our game does today

`module/src/pvp.rs` decides enemies from the zone's PvP state and parties only. The
`SetTeamNumber` reward does nothing (`module/src/quests.rs`), `TriggerForZoneTeam` is not
handled (and the two files that use it do not load, see
[[backlog/faction-wars|Faction Wars]]), and zone join, kill and death triggers are never run.
The Crusader Training Camp is "everyone except your party" instead of everyone.

## Building it

- **Server**: a team number on players (default 2) and monsters; `players_hostile` compares
  teams in PvP zones; `SetTeamNumber` and ability 34 in quest rewards and checks; run the
  zone's join trigger on every zone change, the kill trigger on the killer and the death
  trigger on the victim of a player kill; `TriggerForZoneTeam`; the party rule of string 591.
- **Client**: show enemies by team (name colour) in PvP zones.
- **Data**: none to change; the triggers are already in LIST_ZONE.STB (read by
  `crates/rose-data-irose/src/zone_database.rs` as join, kill and dead trigger).
