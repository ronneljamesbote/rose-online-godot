---
kind: quest
id: 2116
name: 'The Broker''s Trial'
status: in-game
given_by:
- '[[npcs/1124-caravan-broker-harlan|Caravan Broker Harlan]]'
npcs:
- '[[npcs/1057-caravan-trader-sella|Caravan Trader Sella]]'
- '[[npcs/1074-caravan-smith-garro|Caravan Smith Garro]]'
level: 10
steps: 6
source:
  data: 'godot/data/quests/2116-the-broker-s-trial.tres; quest triggers in godot/data/quest_triggers/: ql21-001/2116-01, ql21-001/2116-02, ql21-001/2116-21, ql21-001/2116-03, ql21-001/2116-04, ql21-001/2116-31'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Broker's Trial

Harlan, the caravan's broker, will make you a Dealer if you can find goods and learn what they're worth. Bring 6 Plump Pumpkin Seeds, and ask Sella and Garro what they'd pay.

Part of the [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]] story; see "Quests" on the zone page for the whole chain.

[[npcs/1124-caravan-broker-harlan|Caravan Broker Harlan]] offers it from level 10, after [[quests/2101-dust-on-the-road|Dust on the Road]].

- **Start:** Bring Harlan 6 Plump Pumpkin Seeds from the Big Pumpkins, and ask Sella and Garro in the camp what they'd pay.
- **End:** Harlan made you a Dealer.

A job trial: only a Visitor can take it, and only one of the four trials at the camp at a time (the other mentors hide theirs while you have one). Finishing it makes you a Dealer right at the camp, the same job change the quests in [[zones/1-canyon-city-of-zant|Zant]] give, so you don't have to go to Zant for it.

When you hand it in you pick one reward: [[items/weapon/1127-broker-s-pistol|Broker's Pistol]], [[items/weapon/1128-broker-s-launcher|Broker's Launcher]]. Each pick is its own step below; the steps are the same apart from the item.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2116-01`

Happens by talking to [[npcs/1124-caravan-broker-harlan|Caravan Broker Harlan]].

Checks:

- your Level ≥ 10
- quest switch 600 = 1 (you finished [[quests/2101-dust-on-the-road|Dust on the Road]])
- quest switch 616 = 0 (you haven't done this quest yet)
- your job is Visitor (job 0)

Then:

- you get the quest [[quests/2116-the-broker-s-trial|The Broker's Trial]]

### `2116-02`

Happens by talking to [[npcs/1124-caravan-broker-harlan|Caravan Broker Harlan]] and picking the Broker's Pistol.

Checks:

- you have [[quests/2116-the-broker-s-trial|The Broker's Trial]]
- your job is Visitor (job 0)
- you carry ≥ 6 × [[items/quest/711-plump-pumpkin-seed|Plump Pumpkin Seed]]
- quest variable 0 = 1 (you talked to [[npcs/1057-caravan-trader-sella|Caravan Trader Sella]])
- quest variable 1 = 1 (you talked to [[npcs/1074-caravan-smith-garro|Caravan Smith Garro]])

Then:

- works on [[quests/2116-the-broker-s-trial|The Broker's Trial]]
- you get 1 × [[items/weapon/1127-broker-s-pistol|Broker's Pistol]]
- 6 × [[items/quest/711-plump-pumpkin-seed|Plump Pumpkin Seed]] is taken
- your job becomes Dealer (job 411), as the job change quests in Zant do it
- job variable 0 goes up by 1 (first job done)
- HP and MP are filled
- experience, base 800 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 500 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set quest switch 616 to 1
- the quest ends (removed from your list)

### `2116-21`

Happens by talking to [[npcs/1124-caravan-broker-harlan|Caravan Broker Harlan]] and picking the Broker's Launcher.

Checks:

- you have [[quests/2116-the-broker-s-trial|The Broker's Trial]]
- your job is Visitor (job 0)
- you carry ≥ 6 × [[items/quest/711-plump-pumpkin-seed|Plump Pumpkin Seed]]
- quest variable 0 = 1 (you talked to [[npcs/1057-caravan-trader-sella|Caravan Trader Sella]])
- quest variable 1 = 1 (you talked to [[npcs/1074-caravan-smith-garro|Caravan Smith Garro]])

Then:

- works on [[quests/2116-the-broker-s-trial|The Broker's Trial]]
- you get 1 × [[items/weapon/1128-broker-s-launcher|Broker's Launcher]]
- 6 × [[items/quest/711-plump-pumpkin-seed|Plump Pumpkin Seed]] is taken
- your job becomes Dealer (job 411), as the job change quests in Zant do it
- job variable 0 goes up by 1 (first job done)
- HP and MP are filled
- experience, base 800 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 500 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set quest switch 616 to 1
- the quest ends (removed from your list)

### `2116-03`

Happens by talking to [[npcs/1057-caravan-trader-sella|Caravan Trader Sella]].

Checks:

- you have [[quests/2116-the-broker-s-trial|The Broker's Trial]]
- quest variable 0 = 0

Then:

- works on [[quests/2116-the-broker-s-trial|The Broker's Trial]]
- set quest variable 0 to 1

### `2116-04`

Happens by talking to [[npcs/1074-caravan-smith-garro|Caravan Smith Garro]].

Checks:

- you have [[quests/2116-the-broker-s-trial|The Broker's Trial]]
- quest variable 1 = 0

Then:

- works on [[quests/2116-the-broker-s-trial|The Broker's Trial]]
- set quest variable 1 to 1

### `2116-31`

Happens by killing [[monsters/22-big-pumpkin|Big Pumpkin]].

Checks:

- you have [[quests/2116-the-broker-s-trial|The Broker's Trial]]
- a random roll 0–99 lands in 0–49
- you carry < 6 × [[items/quest/711-plump-pumpkin-seed|Plump Pumpkin Seed]]

Then:

- works on [[quests/2116-the-broker-s-trial|The Broker's Trial]]
- you get 1 × [[items/quest/711-plump-pumpkin-seed|Plump Pumpkin Seed]]

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Valley of Luxem Tower redesign.
