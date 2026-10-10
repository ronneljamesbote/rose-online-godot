---
kind: quest
id: 1987
name: The Truth of the Golden Dagger (2)
status: in-game
steps: 2
source:
  data: LIST_QUEST.STB row 1987; QSD triggers 1986-02, 1987-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Truth of the Golden Dagger (2)

You should ask Winters, a Soldier in Junon Polis, about the Golden Dagger.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1986-02`

Checks:

- you have [[quests/1987-the-truth-of-the-golden-dagger-2|The Truth of the Golden Dagger (2)]]
- quest switch 133 is off

Then:

- works on [[quests/1987-the-truth-of-the-golden-dagger-2|The Truth of the Golden Dagger (2)]]
- experience: 5000 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest becomes [[quests/1988-pavrick-the-craftsman-on-lunar|Pavrick, the Craftsman on Lunar]] (progress kept)
- quest switch 133 on

### `1987-01`

Checks:

- quest switch 131 is on
- quest switch 132 is on
- quest switch 133 is off

Then:

- you get the quest [[quests/1987-the-truth-of-the-golden-dagger-2|The Truth of the Golden Dagger (2)]]
- works on [[quests/1987-the-truth-of-the-golden-dagger-2|The Truth of the Golden Dagger (2)]]
- you get 1 × [[items/quest/510-golden-dagger|Golden Dagger]]
