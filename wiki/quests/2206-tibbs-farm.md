---
kind: quest
id: 2206
name: Tibbs' Farm
status: in-game
given_by:
- '[[npcs/1042-farmer-tibbs|Farmer Tibbs]]'
level: 6
steps: 3
source:
  data: 'godot/data/quests/2206-tibbs-farm.tres; quest triggers in godot/data/quest_triggers/: qa22-001/2206-01, qa22-001/2206-02, qa22-001/2206-31'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Tibbs' Farm

Farmer Tibbs fled his farm on the terraces when the Flanae took root in it. He asked for 10 Farm Flanae Petals to show they can be beaten.

Part of the [[zones/22-adventurer-s-plain|Adventurer's Plain]] story; see "Quests" on the zone page for the whole chain.

[[npcs/1042-farmer-tibbs|Farmer Tibbs]] offers it from level 6, after [[quests/2205-ford-watch|Ford Watch]].

- **Start:** Bring Tibbs 10 Farm Flanae Petals from the Flanae on the terraces west of Riverford.
- **End:** Tibbs dares to hope for his farm again.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2206-01`

Happens by talking to [[npcs/1042-farmer-tibbs|Farmer Tibbs]].

Checks:

- quest switch 627 is on (you finished [[quests/2205-ford-watch|Ford Watch]])
- your Level ≥ 6
- quest switch 628 is off (you haven't done this quest yet)

Then:

- you get the quest [[quests/2206-tibbs-farm|Tibbs' Farm]]

### `2206-02`

Happens by talking to [[npcs/1042-farmer-tibbs|Farmer Tibbs]].

Checks:

- you have [[quests/2206-tibbs-farm|Tibbs' Farm]]
- you carry ≥ 10 × [[items/quest/716-farm-flanae-petal|Farm Flanae Petal]]

Then:

- works on [[quests/2206-tibbs-farm|Tibbs' Farm]]
- 10 × [[items/quest/716-farm-flanae-petal|Farm Flanae Petal]] is taken
- experience, base 200 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 250 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get 10 × [[items/consumable/102-banana|Banana]]
- quest switch 628 on
- the quest ends (removed from your list)

### `2206-31`

Happens by killing [[monsters/41-flanae|Flanae]], [[monsters/42-elder-flanae|Elder Flanae]]: their kill steps `5035-31` ([[quests/5035-stockpiling-on-flanae-petals|Stockpiling on Flanae Petals]]), `5035-32` ([[quests/5035-stockpiling-on-flanae-petals|Stockpiling on Flanae Petals]]) run it next, so a kill counts for both quests.

Checks:

- you have [[quests/2206-tibbs-farm|Tibbs' Farm]]
- a random roll 0–99 lands in 0–60
- you carry < 10 × [[items/quest/716-farm-flanae-petal|Farm Flanae Petal]]

Then:

- works on [[quests/2206-tibbs-farm|Tibbs' Farm]]
- you get 1 × [[items/quest/716-farm-flanae-petal|Farm Flanae Petal]]

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Adventurer's Plain redesign.
