---
kind: quest
id: 3102
name: Arrogance of the Junon Order
status: in-game
time_limit_minutes: 60
steps: 3
source:
  data: LIST_QUEST.STB row 3102; QSD triggers 3102-31, 3102-32, PvP1301-120
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Arrogance of the Junon Order

Those schemers in the Junon Order have been raising an army in secret! We've got to stop them before they grow too powerful!  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3102-31`

Checks:

- you have [[quests/3102-arrogance-of-the-junon-order|Arrogance of the Junon Order]]
- your Faction = 3

Then:

- 1 × [[monsters/353-rider-captain|Rider Captain]] appear at (5200, 5099) in [[zones/5-junon-cartel|Junon Cartel]]
- then runs step `PvP1301-120`

### `3102-32`

Checks:

- you have [[quests/3102-arrogance-of-the-junon-order|Arrogance of the Junon Order]]
- your Faction = 3
- the NPC [[npcs/1086-akram-minister-rodath|Akram Minister Rodath]]
- the NPC's variable 1 = 20

Then:

- set the NPC's variable 1 to 22
- the NPC says: "The protective spirit of the Righteous Crusaders has been defeated. The Junon Order has won the battle."
