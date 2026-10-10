---
kind: quest
id: 858
name: Hovert's Mementos
status: in-game
given_by:
- '[[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]]'
npcs:
- '[[npcs/1122-guide-of-plain-pein|Guide of Plain Pein]]'
monsters:
- '[[monsters/172-aqua-warrior|Aqua Warrior]]'
- '[[monsters/175-aqua-ranger|Aqua Ranger]]'
- '[[monsters/176-aqua-hunter|Aqua Hunter]]'
steps: 10
source:
  data: LIST_QUEST.STB row 858; QSD triggers 858-01, 858-02, 858-03, 858-31, 858-32, 858-33, 858-34, 858-37, 858-38, 858-39
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Hovert's Mementos

Crune has related the legend of Hovert, the greatest Soldier who ever lived. It seems he left behind some mementos around Anima Lake after he passed away.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `858-01`

Happens by talking to [[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]].

Checks:

- job variable 0 = 3
- your Job = 1
- your Level ≥ 30

Then:

- you get the quest [[quests/858-hovert-s-mementos|Hovert's Mementos]]

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

### `858-31`

Happens by killing [[monsters/172-aqua-warrior|Aqua Warrior]].

Checks:

- you have [[quests/858-hovert-s-mementos|Hovert's Mementos]]
- you carry < 1 × [[items/quest/122-old-horse-saddle|Old Horse Saddle]]
- a random roll 0–99 lands in 0–10

Then:

- works on [[quests/858-hovert-s-mementos|Hovert's Mementos]]
- you get 1 × [[items/quest/122-old-horse-saddle|Old Horse Saddle]]

If the checks fail, step `858-32` is tried instead.

### `858-32`

Checks:

- you have [[quests/858-hovert-s-mementos|Hovert's Mementos]]
- quest variable 0 < 10
- you carry ≥ 1 × [[items/quest/122-old-horse-saddle|Old Horse Saddle]]

Then:

- works on [[quests/858-hovert-s-mementos|Hovert's Mementos]]
- add 1 to quest variable 0

If the checks fail, step `858-33` is tried instead.

### `858-33`

Checks:

- you have [[quests/858-hovert-s-mementos|Hovert's Mementos]]
- quest variable 0 ≥ 10
- you carry < 1 × [[items/quest/124-single-leather-boot|Single Leather Boot]]
- a random roll 0–99 lands in 0–10

Then:

- works on [[quests/858-hovert-s-mementos|Hovert's Mementos]]
- you get 1 × [[items/quest/124-single-leather-boot|Single Leather Boot]]

If the checks fail, step `121-31` is tried instead.

### `858-34`

Happens by killing [[monsters/175-aqua-ranger|Aqua Ranger]].

Checks:

- you have [[quests/858-hovert-s-mementos|Hovert's Mementos]]
- you carry < 1 × [[items/quest/123-rusted-iron-helmet|Rusted Iron Helmet]]
- a random roll 0–99 lands in 0–10

Then:

- works on [[quests/858-hovert-s-mementos|Hovert's Mementos]]
- you get 1 × [[items/quest/123-rusted-iron-helmet|Rusted Iron Helmet]]

If the checks fail, step `1008-31` is tried instead.

### `858-37`

Happens by killing [[monsters/176-aqua-hunter|Aqua Hunter]].

Checks:

- you have [[quests/858-hovert-s-mementos|Hovert's Mementos]]
- you carry < 1 × [[items/quest/125-old-leather-diary|Old Leather Diary]]
- a random roll 0–99 lands in 0–10

Then:

- works on [[quests/858-hovert-s-mementos|Hovert's Mementos]]
- you get 1 × [[items/quest/125-old-leather-diary|Old Leather Diary]]

If the checks fail, step `858-38` is tried instead.

### `858-38`

Checks:

- you have [[quests/858-hovert-s-mementos|Hovert's Mementos]]
- quest variable 2 < 20
- you carry ≥ 1 × [[items/quest/125-old-leather-diary|Old Leather Diary]]

Then:

- works on [[quests/858-hovert-s-mementos|Hovert's Mementos]]
- add 1 to quest variable 2

If the checks fail, step `858-39` is tried instead.

### `858-39`

Checks:

- you have [[quests/858-hovert-s-mementos|Hovert's Mementos]]
- quest variable 2 ≥ 20
- you carry < 1 × [[items/quest/127-hovert-s-necklace|Hovert's Necklace]]
- a random roll 0–99 lands in 0–7

Then:

- works on [[quests/858-hovert-s-mementos|Hovert's Mementos]]
- you get 1 × [[items/quest/127-hovert-s-necklace|Hovert's Necklace]]
- set quest switch 1 to 1

If the checks fail, step `1008-32` is tried instead.
