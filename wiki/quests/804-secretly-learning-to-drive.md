---
kind: quest
id: 804
name: Secretly Learning to Drive
status: in-game
npcs:
- '[[npcs/1096-ferrell-guild-merchant-mildun|Ferrell Guild Merchant Mildun]]'
- '[[npcs/1142-akram-minister-luce|Akram Minister Luce]]'
steps: 2
source:
  data: LIST_QUEST.STB row 804; QSD triggers 803-01, 804-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Secretly Learning to Drive

Luce has explained everything about Cart functions to you. Now you can go back to Junon Polis to ask Mildun to teach you the Drive Cart skill.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `803-01`

Happens by talking to [[npcs/1142-akram-minister-luce|Akram Minister Luce]].

Checks:

- you have [[quests/803-secretly-learning-to-drive|Secretly Learning to Drive]]

Then:

- works on [[quests/803-secretly-learning-to-drive|Secretly Learning to Drive]]
- you get 1 × [[items/quest/100-luce-s-certification|Luce's certification]]
- the quest becomes [[quests/804-secretly-learning-to-drive|Secretly Learning to Drive]] (progress kept)

### `804-01`

Happens by talking to [[npcs/1096-ferrell-guild-merchant-mildun|Ferrell Guild Merchant Mildun]].

Checks:

- you have [[quests/804-secretly-learning-to-drive|Secretly Learning to Drive]]

Then:

- works on [[quests/804-secretly-learning-to-drive|Secretly Learning to Drive]]
- the quest becomes [[quests/805-secretly-learning-to-drive|Secretly Learning to Drive]] (progress kept)
