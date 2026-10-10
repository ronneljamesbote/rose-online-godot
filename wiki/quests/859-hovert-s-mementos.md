---
kind: quest
id: 859
name: Hovert's Mementos
status: in-game
npcs:
- '[[npcs/1122-guide-of-plain-pein|Guide of Plain Pein]]'
monsters:
- '[[monsters/173-aqua-captain|Aqua Captain]]'
steps: 4
source:
  data: LIST_QUEST.STB row 859; QSD triggers 858-02, 858-03, 859-01, 859-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Hovert's Mementos

After giving Hovert's mementos to Pein, he asked you to fulfill his grandfather's last request. Go and defeat 3 Aqua Captains!  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `858-02`

Happens by talking to [[npcs/1122-guide-of-plain-pein|Guide of Plain Pein]].

Checks:

- you have [[quests/858-hovert-s-mementos|Hovert's Mementos]]
- quest switch 1 = 1
- you carry ≥ 1 × [[items/quest/122-old-horse-saddle|Old Horse Saddle]]
- you carry ≥ 1 × [[items/quest/123-rusted-iron-helmet|Rusted Iron Helmet]]
- you carry ≥ 1 × [[items/quest/124-single-leather-boot|Single Leather Boot]]
- you carry ≥ 1 × [[items/quest/125-old-leather-diary|Old Leather Diary]]
- you carry ≥ 1 × [[items/quest/127-hovert-s-necklace|Hovert's Necklace]]

Then:

- works on [[quests/858-hovert-s-mementos|Hovert's Mementos]]
- 1 × [[items/quest/122-old-horse-saddle|Old Horse Saddle]] is taken
- 1 × [[items/quest/123-rusted-iron-helmet|Rusted Iron Helmet]] is taken
- 1 × [[items/quest/124-single-leather-boot|Single Leather Boot]] is taken
- 1 × [[items/quest/125-old-leather-diary|Old Leather Diary]] is taken
- 1 × [[items/quest/127-hovert-s-necklace|Hovert's Necklace]] is taken
- you get 1 × [[items/quest/128-hovert-s-letter|Hovert's Letter]]
- the quest becomes [[quests/859-hovert-s-mementos|Hovert's Mementos]] (progress kept)

### `858-03`

Happens by talking to [[npcs/1122-guide-of-plain-pein|Guide of Plain Pein]].

Checks:

- you have [[quests/858-hovert-s-mementos|Hovert's Mementos]]
- you carry ≥ 1 × [[items/quest/122-old-horse-saddle|Old Horse Saddle]]
- you carry ≥ 1 × [[items/quest/123-rusted-iron-helmet|Rusted Iron Helmet]]
- you carry ≥ 1 × [[items/quest/124-single-leather-boot|Single Leather Boot]]
- you carry ≥ 1 × [[items/quest/125-old-leather-diary|Old Leather Diary]]

Then:

- works on [[quests/858-hovert-s-mementos|Hovert's Mementos]]
- 1 × [[items/quest/122-old-horse-saddle|Old Horse Saddle]] is taken
- 1 × [[items/quest/123-rusted-iron-helmet|Rusted Iron Helmet]] is taken
- 1 × [[items/quest/124-single-leather-boot|Single Leather Boot]] is taken
- 1 × [[items/quest/125-old-leather-diary|Old Leather Diary]] is taken
- you get 1 × [[items/quest/128-hovert-s-letter|Hovert's Letter]]
- the quest becomes [[quests/859-hovert-s-mementos|Hovert's Mementos]] (progress kept)

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

### `859-31`

Happens by killing [[monsters/173-aqua-captain|Aqua Captain]].

Checks:

- you have [[quests/859-hovert-s-mementos|Hovert's Mementos]]
- you carry < 3 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/859-hovert-s-mementos|Hovert's Mementos]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `957-31` is tried instead.
