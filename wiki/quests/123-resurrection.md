---
kind: quest
id: 123
name: Resurrection
status: in-game
npcs:
- '[[npcs/1097-tavern-owner-harin|Tavern Owner Harin]]'
steps: 2
source:
  data: LIST_QUEST.STB row 123; QSD triggers 122-01, 123-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Resurrection

The hideous monster that appeared after sprinkling the Resurrection Medicine was carrying a small necklace and an emblem that's similar to the one you found while investigating the incident involving the cursed mushrooms. You'd better show these to Harin.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `122-01`

Checks:

- you have [[quests/122-resurrection|Resurrection]]

Then:

- works on [[quests/122-resurrection|Resurrection]]
- you get 1 × [[items/quest/601-enigmatic-emblem|Enigmatic Emblem]]
- you get 1 × [[items/quest/612-small-necklace|Small Necklace]]
- the quest becomes [[quests/123-resurrection|Resurrection]] (progress kept)

### `123-01`

Happens by talking to [[npcs/1097-tavern-owner-harin|Tavern Owner Harin]].

Checks:

- you have [[quests/123-resurrection|Resurrection]]
- you carry = 1 × [[items/quest/601-enigmatic-emblem|Enigmatic Emblem]]
- you carry = 1 × [[items/quest/612-small-necklace|Small Necklace]]

Then:

- works on [[quests/123-resurrection|Resurrection]]
- 1 × [[items/quest/601-enigmatic-emblem|Enigmatic Emblem]] is taken
- 1 × [[items/quest/612-small-necklace|Small Necklace]] is taken
- experience: 6000 (XP, not scaled by level, see [[rules/quests|Quests]])
- you get [[items/consumable/352-junon-polis-return-scroll|Junon Polis Return Scroll]] (item count: 5)
- you get [[items/head/314-joker-jester|Joker Jester]] (item count: 1)
- you get [[items/body/214-islamic-dress|Islamic Dress]] (item count: 1)
- you get [[items/hands/214-gloves-of-iguje|Gloves of Iguje]] (item count: 1)
- you get [[items/feet/214-land-walkers|Land Walkers]] (item count: 1)
- the quest becomes [[quests/124-magic-of-anima-lake|Magic of Anima Lake]] (progress kept)
- set episode variable 0 to 20
