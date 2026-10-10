---
kind: quest
id: 802
name: Secretly Learning to Drive
status: in-game
given_by:
- '[[npcs/1096-ferrell-guild-merchant-mildun|Ferrell Guild Merchant Mildun]]'
npcs:
- '[[npcs/1032-akram-minister-mairard|Akram Minister Mairard]]'
steps: 2
source:
  data: LIST_QUEST.STB row 802; QSD triggers 802-01, 802-03
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Secretly Learning to Drive

You asked Mildun to teach you the Drive Cart skill, but he has directed you to Mairard. You should go and meet Mairard as soon as you can.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `802-01`

Happens by talking to [[npcs/1096-ferrell-guild-merchant-mildun|Ferrell Guild Merchant Mildun]].

Checks:

- your Level ≥ 50
- you don't have [[skills/17-drive-cart|Drive Cart]]

Then:

- you get the quest [[quests/802-secretly-learning-to-drive|Secretly Learning to Drive]]
- works on [[quests/802-secretly-learning-to-drive|Secretly Learning to Drive]]
- you get 1 × [[items/quest/98-mildun-s-recommendation|Mildun's Recommendation]]

### `802-03`

Happens by talking to [[npcs/1032-akram-minister-mairard|Akram Minister Mairard]].

Checks:

- you have [[quests/802-secretly-learning-to-drive|Secretly Learning to Drive]]

Then:

- works on [[quests/802-secretly-learning-to-drive|Secretly Learning to Drive]]
- you get 1 × [[items/quest/99-mairard-s-certification|Mairard's Certification]]
- the quest becomes [[quests/803-secretly-learning-to-drive|Secretly Learning to Drive]] (progress kept)
