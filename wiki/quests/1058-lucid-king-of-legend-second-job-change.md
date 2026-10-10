---
kind: quest
id: 1058
name: 'Lucid: King of Legend (Second Job Change)'
status: in-game
npcs:
- '[[npcs/1005-righteous-crusader-leonard|Righteous Crusader Leonard]]'
- '[[npcs/1064-weapon-craftsman-mairath|Weapon Craftsman Mairath]]'
steps: 2
source:
  data: LIST_QUEST.STB row 1058; QSD triggers 1057-01, 1058-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Lucid: King of Legend (Second Job Change)

Leonard was greatly pleased that you found all the pieces of Lucid's Sword. Now you should go to Breezy Hills and ask Mairath to restore Lucid's Sword.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

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

### `1058-01`

Happens by talking to [[npcs/1064-weapon-craftsman-mairath|Weapon Craftsman Mairath]].

Checks:

- you have [[quests/1058-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]

Then:

- works on [[quests/1058-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- 1 × [[items/quest/202-fragment-of-lucid-s-sword-1|Fragment of Lucid's Sword (1)]] is taken
- 1 × [[items/quest/203-fragment-of-lucid-s-sword-2|Fragment of Lucid's Sword (2)]] is taken
- 1 × [[items/quest/204-fragment-of-lucid-s-sword-3|Fragment of Lucid's Sword (3)]] is taken
- 1 × [[items/quest/205-fragment-of-lucid-s-sword-4|Fragment of Lucid's Sword (4)]] is taken
- 1 × [[items/quest/206-fragment-of-lucid-s-sword-5|Fragment of Lucid's Sword (5)]] is taken
- the quest becomes [[quests/1059-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]] (progress kept)
