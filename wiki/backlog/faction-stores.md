---
kind: backlog
id: faction-stores
name: Faction Point Shops
status: not-in-game-yet
summary: Four faction NPCs sell faction-only gear to their own members, paid with that faction's points instead of Zulie
npcs:
  - "[[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]]"
  - "[[npcs/1110-ferrell-guild-staff-charrs|Ferrell Guild Staff Charrs]]"
  - "[[npcs/1111-righteous-crusader-huffe|Righteous Crusader Huffe]]"
  - "[[npcs/1112-arumic-researcher-carasia|Arumic Researcher Carasia]]"
source:
  data: LIST_NPC.STB column 20 (store faction number) rows 1109-1112; LIST_SELL.STB "Faction" tabs; LIST_UNION.STB and LIST_UNION_S.STL (factions); LIST_STRING.STL strings 250-253, 338, 381
  reference: iROSE 129_129en client data; NpcStoreTransactionError::NotSameUnion and NotEnoughUnionPoints in crates/rose-game-common/src/messages/server.rs; module/src/npcs.rs
---
# Faction Point Shops

## Factions in short

iROSE calls factions "unions". LIST_UNION.STB has seven: Junon Order (1), Akram Kingdom
(2), Righteous Crusaders (3), Arumic (4), Ferrell Guild (5), Gypsies (6) and Ikaness (7).
A player belongs to at most one (ability "union"). Joining happens through faction quests
(for example the Junon Order asks for level 30 and 30,000 Zulie), and faction quests add
**faction points** for that faction (abilities UnionPoint1 to UnionPoint10, one counter per
faction number). Joining, faction quests and faction points already work in our game
through the quest system; spending the points at the Faction Point Shop does not.

## How it works in iROSE 129

Four NPCs are faction shops: LIST_NPC.STB column 20 holds their faction number. Each has a
"Faction" tab of 25 items (faction clothing and gear) and a dialog choice "I'd like to use
the Faction Point Shop" that opens it.

| NPC | Faction | Points used |
| --- | --- | --- |
| [[npcs/1109-elder-of-junon-order-gorthein\|Elder of Junon Order Gorthein]] | 1 Junon Order | UnionPoint1 |
| [[npcs/1111-righteous-crusader-huffe\|Righteous Crusader Huffe]] | 3 Righteous Crusaders | UnionPoint3 |
| [[npcs/1112-arumic-researcher-carasia\|Arumic Researcher Carasia]] | 4 Arumic | UnionPoint4 |
| [[npcs/1110-ferrell-guild-staff-charrs\|Ferrell Guild Staff Charrs]] | 5 Ferrell Guild | UnionPoint5 |

Rules:

- Only members of that faction can buy. Others get "You have not joined this Faction."
  (string 250), or "You are not affiliated with any Faction." (381) when they have none.
- The shop shows **Required Points** (string 338) instead of a Zulie price, and buying
  takes that many points from the member's points for that faction. Too few points:
  "More Faction Points are required." (252).
- The shop window shows the member's current "Faction Points" (251).

> Open question: how many points an item costs. The shop rows have ordinary Zulie base
> prices (for example 17,000 for Fancy Sky), so the server must turn the price into points
> with a formula that is not in the client data. Check rose-next or rose-offline's
> npc_store system before building.

> Open question: whether selling to a faction shop is allowed, and what it pays.

Some faction rewards are not in the shop but in dialogs, for example the Junon Order trades
160 points for a basic cart set at [[npcs/1088-founder-of-junon-order-raw|Founder of Junon
Order Raw]]; those are quest steps and already work.

Item columns 23 and 26 of the equipment tables ("required faction" for each bonus stat)
would let a bonus apply only to members of a faction; no item in the 129 data uses them.

## What our game does today

`npc_store_transaction` in `module/src/npcs.rs` refuses every purchase at these four NPCs
with "only union members can trade here", even for members.

## Building it

- **Server**: in `npc_store_transaction`, when the NPC has a faction number, check the
  buyer's faction, price the items in points and take the points from the matching
  UnionPoint counter instead of Zulie.
- **Client**: show Required Points and the player's faction points in the store window
  for these NPCs.
- **Data**: the point cost, once known, may need an override per item if the formula is
  not a simple one.
