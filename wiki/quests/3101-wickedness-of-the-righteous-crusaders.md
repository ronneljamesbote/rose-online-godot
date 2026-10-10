---
kind: quest
id: 3101
name: Wickedness of the Righteous Crusaders
status: in-game
time_limit_minutes: 60
steps: 3
source:
  data: LIST_QUEST.STB row 3101; QSD triggers 3101-31, 3101-32, PvP1301-220
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Wickedness of the Righteous Crusaders

Those Self Righteous Crusaders have declared war on the Junon Order! Let's make an example of those thrill-seeking warmongers.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3101-31`

Checks:

- you have [[quests/3101-wickedness-of-the-righteous-crusaders|Wickedness of the Righteous Crusaders]]
- your Faction = 1

Then:

- 1 × NPC 354 appear at (5200, 5460) in [[zones/5-junon-cartel|Junon Cartel]]
- then runs step `PvP1301-220`

### `3101-32`

Checks:

- you have [[quests/3101-wickedness-of-the-righteous-crusaders|Wickedness of the Righteous Crusaders]]
- your Faction = 1
- the NPC [[npcs/1086-akram-minister-rodath|Akram Minister Rodath]]
- the NPC's variable 1 = 20

Then:

- set the NPC's variable 1 to 21
- the NPC says: "Oh no! Junon's Doonga has appeared in Zant! Lower leveled Visitors should run away! Someone, please stop him!"
