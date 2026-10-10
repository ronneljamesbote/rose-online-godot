---
kind: quest
id: 1057
name: 'Lucid: King of Legend (Second Job Change)'
status: in-game
npcs:
- '[[npcs/1005-righteous-crusader-leonard|Righteous Crusader Leonard]]'
steps: 7
source:
  data: LIST_QUEST.STB row 1057; QSD triggers 1056-01, 1057-01, 1057-31, 1057-32, 1057-33, 1057-34, 1057-35
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Lucid: King of Legend (Second Job Change)

Leonard has told you that the Goblins stole the broken pieces of Lucid's sword after they were hidden in the Goblin Cave. Go into the 2nd level of the Goblin Cave and find all 5 Fragments of Lucid's Sword.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1056-01`

Happens by talking to [[npcs/1005-righteous-crusader-leonard|Righteous Crusader Leonard]].

Checks:

- you have [[quests/1056-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]

Then:

- works on [[quests/1056-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- the quest becomes [[quests/1057-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]] (progress kept)

### `1057-01`

Happens by talking to [[npcs/1005-righteous-crusader-leonard|Righteous Crusader Leonard]].

Checks:

- you have [[quests/1057-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- you carry ≥ 1 × [[items/quest/202-fragment-of-lucid-s-sword-1|Fragment of Lucid's Sword (1)]]
- you carry ≥ 1 × [[items/quest/203-fragment-of-lucid-s-sword-2|Fragment of Lucid's Sword (2)]]
- you carry ≥ 1 × [[items/quest/204-fragment-of-lucid-s-sword-3|Fragment of Lucid's Sword (3)]]
- you carry ≥ 1 × [[items/quest/205-fragment-of-lucid-s-sword-4|Fragment of Lucid's Sword (4)]]
- you carry ≥ 1 × [[items/quest/206-fragment-of-lucid-s-sword-5|Fragment of Lucid's Sword (5)]]

Then:

- works on [[quests/1057-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- the quest becomes [[quests/1058-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]] (progress kept)

### `1057-31`

Checks:

- you have [[quests/1057-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- a random roll 0–99 lands in 0–20
- you carry < 1 × [[items/quest/202-fragment-of-lucid-s-sword-1|Fragment of Lucid's Sword (1)]]

Then:

- works on [[quests/1057-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- you get 1 × [[items/quest/202-fragment-of-lucid-s-sword-1|Fragment of Lucid's Sword (1)]]

If the checks fail, step `1057-32` is tried instead.

### `1057-32`

Checks:

- you have [[quests/1057-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- a random roll 0–99 lands in 0–20
- you carry < 1 × [[items/quest/203-fragment-of-lucid-s-sword-2|Fragment of Lucid's Sword (2)]]

Then:

- works on [[quests/1057-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- you get 1 × [[items/quest/203-fragment-of-lucid-s-sword-2|Fragment of Lucid's Sword (2)]]

If the checks fail, step `1057-33` is tried instead.

### `1057-33`

Checks:

- you have [[quests/1057-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- a random roll 0–99 lands in 0–20
- you carry < 1 × [[items/quest/204-fragment-of-lucid-s-sword-3|Fragment of Lucid's Sword (3)]]

Then:

- works on [[quests/1057-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- you get 1 × [[items/quest/204-fragment-of-lucid-s-sword-3|Fragment of Lucid's Sword (3)]]

If the checks fail, step `1057-34` is tried instead.

### `1057-34`

Checks:

- you have [[quests/1057-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- a random roll 0–99 lands in 0–20
- you carry < 1 × [[items/quest/205-fragment-of-lucid-s-sword-4|Fragment of Lucid's Sword (4)]]

Then:

- works on [[quests/1057-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- you get 1 × [[items/quest/205-fragment-of-lucid-s-sword-4|Fragment of Lucid's Sword (4)]]

If the checks fail, step `1057-35` is tried instead.

### `1057-35`

Checks:

- you have [[quests/1057-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- a random roll 0–99 lands in 0–10
- you carry < 1 × [[items/quest/206-fragment-of-lucid-s-sword-5|Fragment of Lucid's Sword (5)]]

Then:

- works on [[quests/1057-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- you get 1 × [[items/quest/206-fragment-of-lucid-s-sword-5|Fragment of Lucid's Sword (5)]]

If the checks fail, step `1064-31` is tried instead.
