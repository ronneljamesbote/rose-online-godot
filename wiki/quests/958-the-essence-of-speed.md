---
kind: quest
id: 958
name: The Essence of Speed
status: in-game
npcs:
- '[[npcs/1009-armor-seller-carrion|Armor Seller Carrion]]'
time_limit_minutes: 15
steps: 8
source:
  data: LIST_QUEST.STB row 958; QSD triggers 958-01, 958-02, 958-03, 958-04, 958-11, 958-12, 958-31, 959-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Essence of Speed

Carrion is a little disappointed but he'll give you another chance. Go and defeat 10 Aqua Captains in 15 minutes.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `958-01`

Happens by talking to [[npcs/1009-armor-seller-carrion|Armor Seller Carrion]].

Checks:

- you have [[quests/957-the-essence-of-speed|The Essence of Speed]]

Then:

- works on [[quests/957-the-essence-of-speed|The Essence of Speed]]
- the quest becomes [[quests/958-the-essence-of-speed|The Essence of Speed]]
- quest switch 35 on

If the checks fail, step `958-02` is tried instead.

### `958-02`

Checks:

- quest switch 34 is on

Then:

- you get the quest [[quests/958-the-essence-of-speed|The Essence of Speed]]
- quest switch 35 on

### `958-03`

Happens by talking to [[npcs/1009-armor-seller-carrion|Armor Seller Carrion]].

Checks:

- you have [[quests/958-the-essence-of-speed|The Essence of Speed]]
- you carry ≥ 10 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/958-the-essence-of-speed|The Essence of Speed]]
- 10 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]] is taken
- you get [[items/weapon/205-white-wing-bow|White Wing Bow]] (item count: 1)
- you get [[items/consumable/21-mana-vial-s|Mana Vial (S)]] (item count: 10)
- set job variable 0 to 6
- the quest ends (removed from your list)

### `958-04`

Happens by talking to [[npcs/1009-armor-seller-carrion|Armor Seller Carrion]].

Checks:

- you have [[quests/958-the-essence-of-speed|The Essence of Speed]]
- you carry ≥ 10 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/958-the-essence-of-speed|The Essence of Speed]]
- 10 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]] is taken
- you get [[items/weapon/404-rake-hand|Rake Hand]] (item count: 1)
- you get [[items/consumable/21-mana-vial-s|Mana Vial (S)]] (item count: 10)
- set job variable 0 to 6
- the quest ends (removed from your list)

### `958-11`

Happens by talking to [[npcs/1009-armor-seller-carrion|Armor Seller Carrion]].

Checks:

- you have [[quests/958-the-essence-of-speed|The Essence of Speed]]
- the quest timer ≤ 0

### `958-12`

Happens by talking to [[npcs/1009-armor-seller-carrion|Armor Seller Carrion]].

Checks:

- you have [[quests/958-the-essence-of-speed|The Essence of Speed]]
- the quest timer > 0

### `958-31`

Checks:

- you have [[quests/958-the-essence-of-speed|The Essence of Speed]]
- you carry < 10 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]
- the quest timer > 0

Then:

- works on [[quests/958-the-essence-of-speed|The Essence of Speed]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `959-31` is tried instead.

### `959-01`

Happens by talking to [[npcs/1009-armor-seller-carrion|Armor Seller Carrion]].

Checks:

- you have [[quests/958-the-essence-of-speed|The Essence of Speed]]

Then:

- works on [[quests/958-the-essence-of-speed|The Essence of Speed]]
- the quest becomes [[quests/959-the-essence-of-speed|The Essence of Speed]]
- quest switch 36 on

If the checks fail, step `959-02` is tried instead.
