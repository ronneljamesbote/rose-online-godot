---
kind: quest
id: 1055
name: 'Lucid: King of Legend (Second Job Change)'
status: in-game
npcs:
- '[[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]]'
monsters:
- '[[monsters/212-krawfy-king|Krawfy King]]'
steps: 4
source:
  data: LIST_QUEST.STB row 1055; QSD triggers 1051-02, 1055-01, 1055-31, 1055-32
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Lucid: King of Legend (Second Job Change)

The world is beginning to forget the legend of King Lucid, one of the Seven Knights. To revive his memory in the hearts of the people, you must find Lucid's Crown.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1051-02`

Happens by talking to [[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]].

Checks:

- you have [[quests/1051-road-to-second-job-change|Road to Second Job Change]]

Then:

- works on [[quests/1051-road-to-second-job-change|Road to Second Job Change]]
- the quest becomes [[quests/1055-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]] (progress kept)

### `1055-01`

Happens by talking to [[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]].

Checks:

- you have [[quests/1055-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- you carry ≥ 1 × [[items/quest/201-lucid-s-crown|Lucid's Crown]]

Then:

- works on [[quests/1055-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- 1 × [[items/quest/201-lucid-s-crown|Lucid's Crown]] is taken
- the quest becomes [[quests/1056-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]] (progress kept)

### `1055-31`

Happens by killing [[monsters/212-krawfy-king|Krawfy King]].

Checks:

- you have [[quests/1055-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- quest variable 0 < 5

Then:

- works on [[quests/1055-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- add 1 to quest variable 0

If the checks fail, step `1055-32` is tried instead.

### `1055-32`

Checks:

- you have [[quests/1055-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- a random roll 0–99 lands in 0–17
- quest variable 0 ≥ 5
- you carry < 1 × [[items/quest/201-lucid-s-crown|Lucid's Crown]]

Then:

- works on [[quests/1055-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- you get 1 × [[items/quest/201-lucid-s-crown|Lucid's Crown]]

If the checks fail, step `1068-31` is tried instead.
