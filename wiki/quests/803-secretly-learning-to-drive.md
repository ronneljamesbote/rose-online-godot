---
kind: quest
id: 803
name: Secretly Learning to Drive
status: in-game
npcs:
- '[[npcs/1032-akram-minister-mairard|Akram Minister Mairard]]'
- '[[npcs/1142-akram-minister-luce|Akram Minister Luce]]'
steps: 2
source:
  data: LIST_QUEST.STB row 803; QSD triggers 802-03, 803-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Secretly Learning to Drive

Mairard has taught you the history of the Cart. Now, you should ask Luce in Kenji's Beach to teach you about Cart functions.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `802-03`

Happens by talking to [[npcs/1032-akram-minister-mairard|Akram Minister Mairard]].

Checks:

- you have [[quests/802-secretly-learning-to-drive|Secretly Learning to Drive]]

Then:

- works on [[quests/802-secretly-learning-to-drive|Secretly Learning to Drive]]
- you get 1 × [[items/quest/99-mairard-s-certification|Mairard's Certification]]
- the quest becomes [[quests/803-secretly-learning-to-drive|Secretly Learning to Drive]] (progress kept)

### `803-01`

Happens by talking to [[npcs/1142-akram-minister-luce|Akram Minister Luce]].

Checks:

- you have [[quests/803-secretly-learning-to-drive|Secretly Learning to Drive]]

Then:

- works on [[quests/803-secretly-learning-to-drive|Secretly Learning to Drive]]
- you get 1 × [[items/quest/100-luce-s-certification|Luce's certification]]
- the quest becomes [[quests/804-secretly-learning-to-drive|Secretly Learning to Drive]] (progress kept)
