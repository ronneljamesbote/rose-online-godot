---
kind: quest
id: 1056
name: 'Lucid: King of Legend (Second Job Change)'
status: in-game
npcs:
- '[[npcs/1005-righteous-crusader-leonard|Righteous Crusader Leonard]]'
- '[[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]]'
steps: 2
source:
  data: LIST_QUEST.STB row 1056; QSD triggers 1055-01, 1056-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Lucid: King of Legend (Second Job Change)

There's a story about Lucid where his sword was shattered after he drove it in the ground to signify the reunion of the Seven Knights. You should talk to Leonard, whose family has been charged with keeping the sword fragments, to see if you can find the truth.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1055-01`

Happens by talking to [[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]].

Checks:

- you have [[quests/1055-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- you carry ≥ 1 × [[items/quest/201-lucid-s-crown|Lucid's Crown]]

Then:

- works on [[quests/1055-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- 1 × [[items/quest/201-lucid-s-crown|Lucid's Crown]] is taken
- the quest becomes [[quests/1056-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]] (progress kept)

### `1056-01`

Happens by talking to [[npcs/1005-righteous-crusader-leonard|Righteous Crusader Leonard]].

Checks:

- you have [[quests/1056-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]

Then:

- works on [[quests/1056-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- the quest becomes [[quests/1057-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]] (progress kept)
