---
kind: quest
id: 959
name: The Essence of Speed
status: in-game
npcs:
- '[[npcs/1009-armor-seller-carrion|Armor Seller Carrion]]'
time_limit_minutes: 30
steps: 7
source:
  data: LIST_QUEST.STB row 959; QSD triggers 959-01, 959-02, 959-03, 959-04, 959-11, 959-12, 959-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Essence of Speed

Carrion will give you one last chance. You have 30 minutes to defeat 10 Aqua Captains. Remember, Carrion isn't going to give you any more chances!  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `959-01`

Happens by talking to [[npcs/1009-armor-seller-carrion|Armor Seller Carrion]].

Checks:

- you have [[quests/958-the-essence-of-speed|The Essence of Speed]]

Then:

- works on [[quests/958-the-essence-of-speed|The Essence of Speed]]
- the quest becomes [[quests/959-the-essence-of-speed|The Essence of Speed]]
- quest switch 36 on

If the checks fail, step `959-02` is tried instead.

### `959-02`

Checks:

- quest switch 35 is on

Then:

- you get the quest [[quests/959-the-essence-of-speed|The Essence of Speed]]
- quest switch 36 on

### `959-03`

Happens by talking to [[npcs/1009-armor-seller-carrion|Armor Seller Carrion]].

Checks:

- you have [[quests/959-the-essence-of-speed|The Essence of Speed]]
- you carry ≥ 10 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/959-the-essence-of-speed|The Essence of Speed]]
- 10 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]] is taken
- you get [[items/weapon/205-white-wing-bow|White Wing Bow]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set job variable 0 to 6
- the quest ends (removed from your list)

### `959-04`

Happens by talking to [[npcs/1009-armor-seller-carrion|Armor Seller Carrion]].

Checks:

- you have [[quests/959-the-essence-of-speed|The Essence of Speed]]
- you carry ≥ 10 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/959-the-essence-of-speed|The Essence of Speed]]
- you get 10 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]
- you get [[items/weapon/404-rake-hand|Rake Hand]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set job variable 0 to 6
- the quest ends (removed from your list)

### `959-11`

Happens by talking to [[npcs/1009-armor-seller-carrion|Armor Seller Carrion]].

Checks:

- you have [[quests/959-the-essence-of-speed|The Essence of Speed]]
- the quest timer ≤ 0

Then:

- works on [[quests/959-the-essence-of-speed|The Essence of Speed]]
- set job variable 0 to 6
- the quest ends (removed from your list)

### `959-12`

Happens by talking to [[npcs/1009-armor-seller-carrion|Armor Seller Carrion]].

Checks:

- you have [[quests/959-the-essence-of-speed|The Essence of Speed]]
- the quest timer > 0

### `959-31`

Checks:

- you have [[quests/959-the-essence-of-speed|The Essence of Speed]]
- you carry < 10 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]
- the quest timer > 0

Then:

- works on [[quests/959-the-essence-of-speed|The Essence of Speed]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]
