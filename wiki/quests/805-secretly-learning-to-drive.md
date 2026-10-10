---
kind: quest
id: 805
name: Secretly Learning to Drive
status: in-game
npcs:
- '[[npcs/1096-ferrell-guild-merchant-mildun|Ferrell Guild Merchant Mildun]]'
monsters:
- '[[monsters/272-coal-mine-goblin-worker|Coal Mine Goblin Worker]]'
steps: 7
source:
  data: LIST_QUEST.STB row 805; QSD triggers 804-01, 805-01, 805-31, 805-32, 805-33, 805-311, 805-321
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Secretly Learning to Drive

Mildun wants you to hide any documents revealing his involvement in teaching you the Drive Cart skill in the Goblin Cave. Once you've hidden the evidence, return to Mildun.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `804-01`

Happens by talking to [[npcs/1096-ferrell-guild-merchant-mildun|Ferrell Guild Merchant Mildun]].

Checks:

- you have [[quests/804-secretly-learning-to-drive|Secretly Learning to Drive]]

Then:

- works on [[quests/804-secretly-learning-to-drive|Secretly Learning to Drive]]
- the quest becomes [[quests/805-secretly-learning-to-drive|Secretly Learning to Drive]] (progress kept)

### `805-01`

Happens by talking to [[npcs/1096-ferrell-guild-merchant-mildun|Ferrell Guild Merchant Mildun]].

Checks:

- you have [[quests/805-secretly-learning-to-drive|Secretly Learning to Drive]]
- you carry < 1 × [[items/quest/98-mildun-s-recommendation|Mildun's Recommendation]]
- you carry < 1 × [[items/quest/99-mairard-s-certification|Mairard's Certification]]
- you carry < 1 × [[items/quest/100-luce-s-certification|Luce's certification]]

Then:

- works on [[quests/805-secretly-learning-to-drive|Secretly Learning to Drive]]
- you learn [[skills/17-drive-cart|Drive Cart]]
- the quest ends (removed from your list)

### `805-31`

Happens by killing [[monsters/272-coal-mine-goblin-worker|Coal Mine Goblin Worker]].

Checks:

- you have [[quests/805-secretly-learning-to-drive|Secretly Learning to Drive]]
- a random roll 0–99 lands in 0–3
- you carry = 1 × [[items/quest/98-mildun-s-recommendation|Mildun's Recommendation]]

Then:

- works on [[quests/805-secretly-learning-to-drive|Secretly Learning to Drive]]
- 1 × [[items/quest/98-mildun-s-recommendation|Mildun's Recommendation]] is taken
- add 1 to quest variable 0

If the checks fail, step `805-311` is tried instead.

### `805-32`

Checks:

- you have [[quests/805-secretly-learning-to-drive|Secretly Learning to Drive]]
- a random roll 0–99 lands in 0–7
- you carry = 1 × [[items/quest/99-mairard-s-certification|Mairard's Certification]]
- quest variable 1 ≥ 15

Then:

- works on [[quests/805-secretly-learning-to-drive|Secretly Learning to Drive]]
- 1 × [[items/quest/99-mairard-s-certification|Mairard's Certification]] is taken
- add 1 to quest variable 0

If the checks fail, step `805-321` is tried instead.

### `805-33`

Checks:

- you have [[quests/805-secretly-learning-to-drive|Secretly Learning to Drive]]
- a random roll 0–99 lands in 0–7
- you carry = 1 × [[items/quest/100-luce-s-certification|Luce's certification]]
- quest variable 2 ≥ 15

Then:

- works on [[quests/805-secretly-learning-to-drive|Secretly Learning to Drive]]
- 1 × [[items/quest/100-luce-s-certification|Luce's certification]] is taken
- add 1 to quest variable 0

If the checks fail, step `5014-32` is tried instead.

### `805-311`

Checks:

- you have [[quests/805-secretly-learning-to-drive|Secretly Learning to Drive]]
- quest variable 0 ≥ 1
- quest variable 1 < 15

Then:

- works on [[quests/805-secretly-learning-to-drive|Secretly Learning to Drive]]
- add 1 to quest variable 1

If the checks fail, step `805-32` is tried instead.

### `805-321`

Checks:

- you have [[quests/805-secretly-learning-to-drive|Secretly Learning to Drive]]
- quest variable 0 = 2
- quest variable 2 < 15

Then:

- works on [[quests/805-secretly-learning-to-drive|Secretly Learning to Drive]]
- add 1 to quest variable 2

If the checks fail, step `805-33` is tried instead.
