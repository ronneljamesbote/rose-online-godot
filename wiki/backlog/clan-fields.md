---
kind: backlog
id: clan-fields
name: Clan fields
status: not-in-game-yet
summary: Clan-versus-clan PvP maps on Junon and Luna where your clan are your allies, with a warp that takes a clan's nearby members along
npcs:
  - "[[npcs/1115-clan-owner-burtland|Clan Owner Burtland]]"
  - "[[npcs/1191-shamanist-est|Shamanist Est]]"
source:
  data: LIST_ZONE.STB rows 11 and 59 (PvP state 1, join trigger Clan-003); QC001.QSD triggers Clan-003, Clan-004, Clan-warp01 to Clan-warp05
  reference: iROSE 129_129en client data; Burtland's dialog EM02-035.con; QSD reward TeleportNearbyClanMembers in crates/rose-file-readers/src/qsd.rs
---
# Clan fields

[[zones/11-junon-clan-field|Junon Clan Field]] ("Exclusive Clan Training Camp") and
[[zones/59-luna-clan-field|Luna Clan Field]] are open PvP maps where clans fight each other
and hunt strong monsters.

## How it works in iROSE 129

### Who is an enemy

Both zones have PvP state 1, "everyone except your clan". When you enter either zone its
join trigger `Clan-003` runs: if you are in a [[backlog/clans|clan]] your PvP team number
is set from your clan (`SetTeamNumber` with source Clan), so clan members are allies and
everyone else is an enemy. A player without a clan keeps a team of their own and can be
attacked by anyone. Teams are described on
[[backlog/pvp-teams-and-zone-triggers|PvP teams and zone triggers]].

### Getting there

- **Clan warp to Junon Clan Field** at [[npcs/1115-clan-owner-burtland|Burtland]] ("I'd like
  to use this place for Clan Training Camp"): only a Deputy Master or Master (positions 5 to
  6, trigger `Clan-004`). The warp moves the caller **and every clan member near them**
  (`TeleportNearbyClanMembers`, distance 40) to one of four sectors:

| Choice | Trigger | Position in zone 11 |
| --- | --- | --- |
| Northeast (extra low class) | `Clan-warp01` | (5453.19, 5462.14) |
| Southeast (low class) | `Clan-warp02` | (5517.04, 5002.85) |
| Southwest (middle class) | `Clan-warp03` | (4970.51, 4986.52) |
| Northwest (middle high class) | `Clan-warp04` | (4981.25, 5462.64) |

- **Luna Clan Field** at [[npcs/1191-shamanist-est|Shamanist Est]] ("I'd like to transfer to
  Lunar Clan Field"): any clan member (trigger `Clan-warp05`), alone, to (5095.08, 5129.13)
  in zone 59.
- The zones can also be walked into where the map links allow it.

> Open question: the unit of the warp distance 40 (most likely metres, so members within
> 40 m come along) and whether members must accept first.

## What our game does today

Both zones exist with their monsters. Since there are no clans, "everyone except clan"
counts as "everyone", the join trigger does nothing (teams are not in the game), and both
warps fail their clan checks.

## Building it

- **Server**: clan membership in `module/src/pvp.rs` (`players_hostile`); run zone join
  triggers and team numbers (see
  [[backlog/pvp-teams-and-zone-triggers|PvP teams and zone triggers]]); the
  `TeleportNearbyClanMembers` quest reward; the `ClanPosition` and `HasClan` checks.
- **Client**: show clan members as allies (name colour) in these zones.
- **Data**: none to change.
