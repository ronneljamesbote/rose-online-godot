---
kind: quest
id: 957
name: The Essence of Speed
status: in-game
given_by:
- '[[npcs/1009-armor-seller-carrion|Armor Seller Carrion]]'
time_limit_minutes: 10
steps: 7
source:
  data: LIST_QUEST.STB row 957; QSD triggers 957-01, 957-02, 957-03, 957-11, 957-12, 957-31, 958-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Essence of Speed

Reasoning that speed is essential for Hawkers, Carrion wants to test your quickness with a special hunting challenge. He'll give you a reward if you defeat 10 Aqua Captains in 10 minutes.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `957-01`

Happens by talking to [[npcs/1009-armor-seller-carrion|Armor Seller Carrion]].

Checks:

- your Level ≥ 30
- job variable 0 ≥ 2
- job variable 0 ≤ 3
- your Job = 3

Then:

- you get the quest [[quests/957-the-essence-of-speed|The Essence of Speed]]
- quest switch 34 on

### `957-02`

Happens by talking to [[npcs/1009-armor-seller-carrion|Armor Seller Carrion]].

Checks:

- you have [[quests/957-the-essence-of-speed|The Essence of Speed]]
- you carry ≥ 10 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/957-the-essence-of-speed|The Essence of Speed]]
- 10 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]] is taken
- you get [[items/weapon/205-white-wing-bow|White Wing Bow]] (item count: 1)
- you get [[items/jewellery/85-pierced-necklace|Pierced Necklace]] (item count: 1)
- set job variable 0 to 6
- the quest ends (removed from your list)

### `957-03`

Happens by talking to [[npcs/1009-armor-seller-carrion|Armor Seller Carrion]].

Checks:

- you have [[quests/957-the-essence-of-speed|The Essence of Speed]]
- you carry ≥ 10 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/957-the-essence-of-speed|The Essence of Speed]]
- 10 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]] is taken
- you get [[items/weapon/404-rake-hand|Rake Hand]] (item count: 1)
- you get [[items/jewellery/85-pierced-necklace|Pierced Necklace]] (item count: 1)
- set job variable 0 to 6
- the quest ends (removed from your list)

### `957-11`

Happens by talking to [[npcs/1009-armor-seller-carrion|Armor Seller Carrion]].

Checks:

- you have [[quests/957-the-essence-of-speed|The Essence of Speed]]
- the quest timer ≤ 0

### `957-12`

Happens by talking to [[npcs/1009-armor-seller-carrion|Armor Seller Carrion]].

Checks:

- you have [[quests/957-the-essence-of-speed|The Essence of Speed]]
- the quest timer > 0

### `957-31`

Checks:

- you have [[quests/957-the-essence-of-speed|The Essence of Speed]]
- the quest timer > 0
- you carry < 10 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/957-the-essence-of-speed|The Essence of Speed]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `958-31` is tried instead.

### `958-01`

Happens by talking to [[npcs/1009-armor-seller-carrion|Armor Seller Carrion]].

Checks:

- you have [[quests/957-the-essence-of-speed|The Essence of Speed]]

Then:

- works on [[quests/957-the-essence-of-speed|The Essence of Speed]]
- the quest becomes [[quests/958-the-essence-of-speed|The Essence of Speed]]
- quest switch 35 on

If the checks fail, step `958-02` is tried instead.
