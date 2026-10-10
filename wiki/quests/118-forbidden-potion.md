---
kind: quest
id: 118
name: Forbidden Potion
status: in-game
given_by:
- '[[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]]'
monsters:
- '[[monsters/67-queen-honeybee|Queen HoneyBee]]'
- '[[monsters/93-fighter-rackie|Fighter Rackie]]'
steps: 6
source:
  data: LIST_QUEST.STB row 118; QSD triggers 117-01, 117-02, 118-01, 118-31, 118-32, 118-33
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Forbidden Potion

Bellia asks you to get 20 Sharp Stingers and 20 Wild Animal Claws to make the secret medicine. Hunt Queen Bibis and Queen HoneyBees to find Sharp Stingers, and Fighter Rackies for the Wild Animal Claws.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `117-01`

Happens by talking to [[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]].

Checks:

- you have [[quests/117-proof|Proof]]
- you carry = 1 × [[items/quest/602-gypsy-s-permit|Gypsy's Permit]]

Then:

- works on [[quests/117-proof|Proof]]
- 1 × [[items/quest/602-gypsy-s-permit|Gypsy's Permit]] is taken
- the quest becomes [[quests/118-forbidden-potion|Forbidden Potion]] (progress kept)
- set episode variable 0 to 15

### `117-02`

Happens by talking to [[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]].

Checks:

- episode variable 0 = 15

Then:

- you get the quest [[quests/118-forbidden-potion|Forbidden Potion]]

### `118-01`

Happens by talking to [[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]].

Checks:

- you have [[quests/118-forbidden-potion|Forbidden Potion]]
- you carry ≥ 20 × [[items/quest/607-wild-animal-claw|Wild Animal Claw]]
- you carry ≥ 20 × [[items/quest/608-sharp-stinger|Sharp Stinger]]

Then:

- works on [[quests/118-forbidden-potion|Forbidden Potion]]
- 20 × [[items/quest/607-wild-animal-claw|Wild Animal Claw]] is taken
- 20 × [[items/quest/608-sharp-stinger|Sharp Stinger]] is taken
- you get 1 × [[items/quest/602-gypsy-s-permit|Gypsy's Permit]]
- experience, base 2500 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/123-apple-juice|Apple Juice]], base count 30 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/119-forbidden-potion|Forbidden Potion]] (progress kept)
- set episode variable 0 to 16

### `118-31`

Checks:

- you have [[quests/118-forbidden-potion|Forbidden Potion]]
- a random roll 0–99 lands in 0–70
- you carry < 20 × [[items/quest/608-sharp-stinger|Sharp Stinger]]

Then:

- works on [[quests/118-forbidden-potion|Forbidden Potion]]
- you get 1 × [[items/quest/608-sharp-stinger|Sharp Stinger]]

### `118-32`

Happens by killing [[monsters/67-queen-honeybee|Queen HoneyBee]].

Checks:

- you have [[quests/118-forbidden-potion|Forbidden Potion]]
- a random roll 0–99 lands in 0–70
- you carry < 20 × [[items/quest/608-sharp-stinger|Sharp Stinger]]

Then:

- works on [[quests/118-forbidden-potion|Forbidden Potion]]
- you get 1 × [[items/quest/608-sharp-stinger|Sharp Stinger]]

### `118-33`

Happens by killing [[monsters/93-fighter-rackie|Fighter Rackie]].

Checks:

- you have [[quests/118-forbidden-potion|Forbidden Potion]]
- a random roll 0–99 lands in 0–60
- you carry < 20 × [[items/quest/607-wild-animal-claw|Wild Animal Claw]]

Then:

- works on [[quests/118-forbidden-potion|Forbidden Potion]]
- you get 1 × [[items/quest/607-wild-animal-claw|Wild Animal Claw]]

If the checks fail, step `5008-33` is tried instead.
