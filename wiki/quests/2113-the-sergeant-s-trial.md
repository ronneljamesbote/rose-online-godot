---
kind: quest
id: 2113
name: 'The Sergeant''s Trial'
status: in-game
given_by:
- '[[npcs/1080-caravan-sergeant-brakka|Caravan Sergeant Brakka]]'
level: 10
steps: 4
source:
  data: 'godot/data/quests/2113-the-sergeant-s-trial.tres; quest triggers in godot/data/quest_triggers/: ql21-001/2113-01, ql21-001/2113-02, ql21-001/2113-21, ql21-001/2113-31'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Sergeant's Trial

Brakka, the caravan's sergeant, will make you a Soldier if you prove you can hold your ground. Bring 8 Thorned Vines from the Elder Pumpkins in the Withered Fields.

Part of the [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]] story; see "Quests" on the zone page for the whole chain.

[[npcs/1080-caravan-sergeant-brakka|Caravan Sergeant Brakka]] offers it from level 10, after [[quests/2101-dust-on-the-road|Dust on the Road]].

- **Start:** Bring Brakka 8 Thorned Vines from the Elder Pumpkins in the Withered Fields.
- **End:** Brakka made you a Soldier.

A job trial: only a Visitor can take it, and only one of the four trials at the camp at a time (the other mentors hide theirs while you have one). Finishing it makes you a Soldier right at the camp, the same job change the quests in [[zones/1-canyon-city-of-zant|Zant]] give, so you don't have to go to Zant for it.

When you hand it in you pick one reward: [[items/weapon/1121-sandwind-blade|Sandwind Blade]], [[items/weapon/1122-sandwind-greatsword|Sandwind Greatsword]]. Each pick is its own step below; the steps are the same apart from the item.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2113-01`

Happens by talking to [[npcs/1080-caravan-sergeant-brakka|Caravan Sergeant Brakka]].

Checks:

- your Level ≥ 10
- quest switch 600 = 1 (you finished [[quests/2101-dust-on-the-road|Dust on the Road]])
- quest switch 613 = 0 (you haven't done this quest yet)
- your job is Visitor (job 0)

Then:

- you get the quest [[quests/2113-the-sergeant-s-trial|The Sergeant's Trial]]

### `2113-02`

Happens by talking to [[npcs/1080-caravan-sergeant-brakka|Caravan Sergeant Brakka]] and picking the Sandwind Blade.

Checks:

- you have [[quests/2113-the-sergeant-s-trial|The Sergeant's Trial]]
- your job is Visitor (job 0)
- you carry ≥ 8 × [[items/quest/708-thorned-vine|Thorned Vine]]

Then:

- works on [[quests/2113-the-sergeant-s-trial|The Sergeant's Trial]]
- you get 1 × [[items/weapon/1121-sandwind-blade|Sandwind Blade]]
- 8 × [[items/quest/708-thorned-vine|Thorned Vine]] is taken
- your job becomes Soldier (job 111), as the job change quests in Zant do it
- job variable 0 goes up by 1 (first job done)
- HP and MP are filled
- experience, base 800 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 500 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set quest switch 613 to 1
- the quest ends (removed from your list)

### `2113-21`

Happens by talking to [[npcs/1080-caravan-sergeant-brakka|Caravan Sergeant Brakka]] and picking the Sandwind Greatsword.

Checks:

- you have [[quests/2113-the-sergeant-s-trial|The Sergeant's Trial]]
- your job is Visitor (job 0)
- you carry ≥ 8 × [[items/quest/708-thorned-vine|Thorned Vine]]

Then:

- works on [[quests/2113-the-sergeant-s-trial|The Sergeant's Trial]]
- you get 1 × [[items/weapon/1122-sandwind-greatsword|Sandwind Greatsword]]
- 8 × [[items/quest/708-thorned-vine|Thorned Vine]] is taken
- your job becomes Soldier (job 111), as the job change quests in Zant do it
- job variable 0 goes up by 1 (first job done)
- HP and MP are filled
- experience, base 800 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 500 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set quest switch 613 to 1
- the quest ends (removed from your list)

### `2113-31`

Happens by killing [[monsters/23-elder-pumpkin|Elder Pumpkin]].

Checks:

- you have [[quests/2113-the-sergeant-s-trial|The Sergeant's Trial]]
- a random roll 0–99 lands in 0–49
- you carry < 8 × [[items/quest/708-thorned-vine|Thorned Vine]]

Then:

- works on [[quests/2113-the-sergeant-s-trial|The Sergeant's Trial]]
- you get 1 × [[items/quest/708-thorned-vine|Thorned Vine]]

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Valley of Luxem Tower redesign.
