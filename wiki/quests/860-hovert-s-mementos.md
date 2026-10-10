---
kind: quest
id: 860
name: Hovert's Mementos
status: in-game
npcs:
- '[[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]]'
- '[[npcs/1122-guide-of-plain-pein|Guide of Plain Pein]]'
steps: 5
source:
  data: LIST_QUEST.STB row 860; QSD triggers 859-01, 860-01, 860-02, 860-03, 860-04
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Hovert's Mementos

Pein was very pleased that you completed Hovert's final mission. He told you to visit Crune to receive a reward.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `859-01`

Happens by talking to [[npcs/1122-guide-of-plain-pein|Guide of Plain Pein]].

Checks:

- you have [[quests/859-hovert-s-mementos|Hovert's Mementos]]
- you carry ≥ 3 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/859-hovert-s-mementos|Hovert's Mementos]]
- 3 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]] is taken
- 1 × [[items/quest/128-hovert-s-letter|Hovert's Letter]] is taken
- the quest becomes [[quests/860-hovert-s-mementos|Hovert's Mementos]] (progress kept)

### `860-01`

Happens by talking to [[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]].

Checks:

- you have [[quests/860-hovert-s-mementos|Hovert's Mementos]]
- quest switch 1 = 1
- your Job = 1
- your Level ≥ 30
- job variable 0 = 3

Then:

- works on [[quests/860-hovert-s-mementos|Hovert's Mementos]]
- set job variable 0 to 7
- you get [[items/weapon/35-onion-mace|Onion Mace]] (item count: 1)
- you get [[items/jewellery/88-pointed-necklace|Pointed Necklace]] (item count: 1)
- the quest ends (removed from your list)

### `860-02`

Happens by talking to [[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]].

Checks:

- you have [[quests/860-hovert-s-mementos|Hovert's Mementos]]
- quest switch 1 = 1
- your Job = 1
- your Level ≥ 30
- job variable 0 = 3

Then:

- works on [[quests/860-hovert-s-mementos|Hovert's Mementos]]
- set job variable 0 to 7
- you get [[items/weapon/133-battle-axe|Battle Axe]] (item count: 1)
- you get [[items/jewellery/88-pointed-necklace|Pointed Necklace]] (item count: 1)
- the quest ends (removed from your list)

### `860-03`

Happens by talking to [[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]].

Checks:

- you have [[quests/860-hovert-s-mementos|Hovert's Mementos]]
- quest switch 1 = 0
- your Job = 1
- your Level ≥ 30
- job variable 0 = 3

Then:

- works on [[quests/860-hovert-s-mementos|Hovert's Mementos]]
- set job variable 0 to 7
- you get [[items/weapon/35-onion-mace|Onion Mace]] (item count: 1)
- the quest ends (removed from your list)

### `860-04`

Happens by talking to [[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]].

Checks:

- you have [[quests/860-hovert-s-mementos|Hovert's Mementos]]
- quest switch 1 = 0
- your Job = 1
- your Level ≥ 30
- job variable 0 = 3

Then:

- works on [[quests/860-hovert-s-mementos|Hovert's Mementos]]
- set job variable 0 to 7
- you get [[items/weapon/133-battle-axe|Battle Axe]] (item count: 1)
- the quest ends (removed from your list)
