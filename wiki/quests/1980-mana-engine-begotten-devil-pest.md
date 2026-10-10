---
kind: quest
id: 1980
name: 'Mana Engine Begotten: Devil Pest'
status: in-game
given_by:
- '[[npcs/1104-historian-jones|Historian Jones]]'
monsters:
- '[[monsters/401-junon-s-aqua|Junon''s Aqua]]'
- Junon's Grunter (NPC 402)
- Junon's Tree (NPC 403)
- Junon's Krawfy (NPC 405)
- Junon's KingKong (NPC 406)
- Junon's WormDragon (NPC 408)
- Junon's Golem (NPC 409)
- Junon's Goblin King (NPC 410)
time_limit_minutes: 150
steps: 54
source:
  data: LIST_QUEST.STB row 1980; QSD triggers 1980-01, 1980-11, 1980-12, 1980-13, 1980-14, 1980-15, 1980-16, 1980-17, 1980-21, 1980-22, 1980-23, 1980-24, 1980-25, 1980-26, 1980-27, 1980-31, 1980-32, 1980-33, 1980-34, 1980-35, 1980-36, 1980-37, 1980-41, 1980-42, 1980-43, 1980-44, 1980-45, 1980-46, 1980-47, 1980-48, 1980-49, 1980-50, 1980-51, 1980-52, 1980-53, 1980-54, 1980-55, 1980-56, 1980-57, 1980-58, 1980-59, 1980-60, 1980-61, 1980-62, 1980-63, 1980-64, 1980-65, 1980-66, 1980-67, 1980-68, 1980-69, 1980-70, 1980-71, 1980-72
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Mana Engine Begotten: Devil Pest

Years of exhaust from Mana Engines have mutated the Devil Pest virus, creating a new breed of incredibly dangerous monsters. Hunt down these monsters and bring 5 Proofs of Monster Extermination to Jones for a reward. Proofs of Monster Extermination will only drop when you are hunting monsters in a party.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1980-01`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- the NPC [[npcs/1104-historian-jones|Historian Jones]]
- the NPC's variable 0 = 1

Then:

- you get the quest [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]

### `1980-11`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you carry ≥ 5 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]
- you carry ≤ 6 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get [[items/gem/301-garnet-1|Garnet 1]] (item count: 1)
- the quest ends (removed from your list)

### `1980-12`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you carry ≥ 5 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]
- you carry ≤ 6 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get [[items/gem/311-ruby-1|Ruby 1]] (item count: 1)
- the quest ends (removed from your list)

### `1980-13`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you carry ≥ 5 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]
- you carry ≤ 6 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get [[items/gem/321-sapphire-1|Sapphire 1]] (item count: 1)
- the quest ends (removed from your list)

### `1980-14`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you carry ≥ 5 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]
- you carry ≤ 6 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get [[items/gem/331-topaz-1|Topaz 1]] (item count: 1)
- the quest ends (removed from your list)

### `1980-15`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you carry ≥ 5 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]
- you carry ≤ 6 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get [[items/gem/341-emerald-1|Emerald 1]] (item count: 1)
- the quest ends (removed from your list)

### `1980-16`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you carry ≥ 5 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]
- you carry ≤ 6 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get [[items/gem/351-peridot-1|Peridot 1]] (item count: 1)
- the quest ends (removed from your list)

### `1980-17`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you carry ≥ 5 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]
- you carry ≤ 6 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get [[items/gem/361-diamond-1|Diamond 1]] (item count: 1)
- the quest ends (removed from your list)

### `1980-21`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you carry ≥ 7 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]
- you carry ≤ 9 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get [[items/gem/302-garnet-2|Garnet 2]] (item count: 1)
- the quest ends (removed from your list)

### `1980-22`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you carry ≥ 7 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]
- you carry ≤ 9 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get [[items/gem/312-ruby-2|Ruby 2]] (item count: 1)
- the quest ends (removed from your list)

### `1980-23`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you carry ≥ 7 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]
- you carry ≤ 9 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get [[items/gem/322-sapphire-2|Sapphire 2]] (item count: 1)
- the quest ends (removed from your list)

### `1980-24`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you carry ≥ 7 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]
- you carry ≤ 9 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get [[items/gem/332-topaz-2|Topaz 2]] (item count: 1)
- the quest ends (removed from your list)

### `1980-25`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you carry ≥ 7 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]
- you carry ≤ 9 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get [[items/gem/342-emerald-2|Emerald 2]] (item count: 1)
- the quest ends (removed from your list)

### `1980-26`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you carry ≥ 7 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]
- you carry ≤ 9 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get [[items/gem/352-peridot-2|Peridot 2]] (item count: 1)
- the quest ends (removed from your list)

### `1980-27`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you carry ≥ 7 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]
- you carry ≤ 9 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get [[items/gem/362-diamond-2|Diamond 2]] (item count: 1)
- the quest ends (removed from your list)

### `1980-31`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you carry ≥ 10 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get [[items/gem/303-garnet-3|Garnet 3]] (item count: 1)
- the quest ends (removed from your list)

### `1980-32`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you carry ≥ 10 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get [[items/gem/313-ruby-3|Ruby 3]] (item count: 1)
- the quest ends (removed from your list)

### `1980-33`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you carry ≥ 10 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get [[items/gem/323-sapphire-3|Sapphire 3]] (item count: 1)
- the quest ends (removed from your list)

### `1980-34`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you carry ≥ 10 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get [[items/gem/333-topaz-3|Topaz 3]] (item count: 1)
- the quest ends (removed from your list)

### `1980-35`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you carry ≥ 10 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get [[items/gem/343-emerald-3|Emerald 3]] (item count: 1)
- the quest ends (removed from your list)

### `1980-36`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you carry ≥ 10 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get [[items/gem/353-peridot-3|Peridot 3]] (item count: 1)
- the quest ends (removed from your list)

### `1980-37`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you carry ≥ 10 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get [[items/gem/363-diamond-3|Diamond 3]] (item count: 1)
- the quest ends (removed from your list)

### `1980-41`

Happens by killing [[monsters/401-junon-s-aqua|Junon's Aqua]].

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level > 53
- a random roll 0–99 lands in 0–10

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `1980-42` is tried instead.

### `1980-42`

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level ≥ 47
- your Level ≤ 53
- a random roll 0–99 lands in 0–30

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `1980-43` is tried instead.

### `1980-43`

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level ≥ 37
- your Level ≤ 46
- a random roll 0–99 lands in 0–50

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `1980-44` is tried instead.

### `1980-44`

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level < 37
- a random roll 0–99 lands in 0–70

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

### `1980-45`

Happens by killing Junon's Tree (NPC 403).

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level > 66
- a random roll 0–99 lands in 0–10

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `1980-46` is tried instead.

### `1980-46`

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level ≥ 60
- your Level ≤ 66
- a random roll 0–99 lands in 0–30

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `1980-47` is tried instead.

### `1980-47`

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level ≥ 50
- your Level ≤ 59
- a random roll 0–99 lands in 0–50

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `1980-48` is tried instead.

### `1980-48`

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level < 50
- a random roll 0–99 lands in 0–70

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

### `1980-49`

Happens by killing Junon's Grunter (NPC 402).

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level > 72
- a random roll 0–99 lands in 0–10

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `1980-50` is tried instead.

### `1980-50`

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level ≥ 66
- your Level ≤ 72
- a random roll 0–99 lands in 0–30

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `1980-51` is tried instead.

### `1980-51`

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level ≥ 56
- your Level ≤ 65
- a random roll 0–99 lands in 0–50

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `1980-52` is tried instead.

### `1980-52`

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level < 56
- a random roll 0–99 lands in 0–70

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

### `1980-53`

Happens by killing Junon's KingKong (NPC 406).

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level > 82
- a random roll 0–99 lands in 0–10

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `1980-54` is tried instead.

### `1980-54`

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level ≥ 76
- your Level ≤ 82
- a random roll 0–99 lands in 0–30

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `1980-55` is tried instead.

### `1980-55`

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level ≥ 66
- your Level ≤ 75
- a random roll 0–99 lands in 0–50

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `1980-56` is tried instead.

### `1980-56`

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level < 66
- a random roll 0–99 lands in 0–70

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

### `1980-57`

Happens by killing Junon's Krawfy (NPC 405).

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level > 86
- a random roll 0–99 lands in 0–10

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `1980-58` is tried instead.

### `1980-58`

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level ≥ 80
- your Level ≤ 86
- a random roll 0–99 lands in 0–30

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `1980-59` is tried instead.

### `1980-59`

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level ≥ 70
- your Level ≤ 79
- a random roll 0–99 lands in 0–50

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `1980-60` is tried instead.

### `1980-60`

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level < 70
- a random roll 0–99 lands in 0–70

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

### `1980-61`

Happens by killing Junon's WormDragon (NPC 408).

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level > 101
- a random roll 0–99 lands in 0–10

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `1980-62` is tried instead.

### `1980-62`

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level ≥ 95
- your Level ≤ 101
- a random roll 0–99 lands in 0–30

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `1980-63` is tried instead.

### `1980-63`

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level ≥ 85
- your Level ≤ 94
- a random roll 0–99 lands in 0–50

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `1980-64` is tried instead.

### `1980-64`

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level < 85
- a random roll 0–99 lands in 0–70

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

### `1980-65`

Happens by killing Junon's Golem (NPC 409).

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level > 106
- a random roll 0–99 lands in 0–10

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `1980-66` is tried instead.

### `1980-66`

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level ≥ 100
- your Level ≤ 106
- a random roll 0–99 lands in 0–30

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `1980-67` is tried instead.

### `1980-67`

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level ≥ 90
- your Level ≤ 99
- a random roll 0–99 lands in 0–50

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `1980-68` is tried instead.

### `1980-68`

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level < 90
- a random roll 0–99 lands in 0–70

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

### `1980-69`

Happens by killing Junon's Goblin King (NPC 410).

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level > 116
- a random roll 0–99 lands in 0–10

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `1980-70` is tried instead.

### `1980-70`

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level ≥ 110
- your Level ≤ 116
- a random roll 0–99 lands in 0–30

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `1980-71` is tried instead.

### `1980-71`

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level ≥ 100
- your Level ≤ 109
- a random roll 0–99 lands in 0–50

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `1980-72` is tried instead.

### `1980-72`

Checks:

- you have [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- the quest timer > 0
- your Level < 100
- a random roll 0–99 lands in 0–70

Then:

- works on [[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]
