---
kind: quest
id: 1059
name: 'Lucid: King of Legend (Second Job Change)'
status: in-game
npcs:
- '[[npcs/1064-weapon-craftsman-mairath|Weapon Craftsman Mairath]]'
monsters:
- '[[monsters/164-stone-golem|Stone Golem]]'
steps: 5
source:
  data: LIST_QUEST.STB row 1059; QSD triggers 1058-01, 1059-01, 1059-02, 1059-31, 1059-32
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Lucid: King of Legend (Second Job Change)

Mairath said that the magic in the Token of Friendship is necessary to restore Lucid's Sword. You might be able to find the Token of Friendship in the Gorge of Silence.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

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

### `1059-01`

Happens by talking to [[npcs/1064-weapon-craftsman-mairath|Weapon Craftsman Mairath]].

Checks:

- you have [[quests/1059-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- you carry ≥ 1 × [[items/quest/208-token-of-friendship|Token of Friendship]]

Then:

- works on [[quests/1059-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- set quest switch 0 to 1
- 1 × [[items/quest/208-token-of-friendship|Token of Friendship]] is taken

### `1059-02`

Happens by talking to [[npcs/1064-weapon-craftsman-mairath|Weapon Craftsman Mairath]].

Checks:

- you have [[quests/1059-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- quest switch 0 = 1

Then:

- works on [[quests/1059-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- you get 1 × [[items/quest/207-lucid-s-sword|Lucid's Sword]]
- the quest becomes [[quests/1060-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]] (progress kept)

### `1059-31`

Happens by killing [[monsters/164-stone-golem|Stone Golem]].

Checks:

- you have [[quests/1059-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- quest variable 2 < 20

Then:

- works on [[quests/1059-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- add 1 to quest variable 2

If the checks fail, step `1059-32` is tried instead.

### `1059-32`

Checks:

- you have [[quests/1059-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- a random roll 0–99 lands in 0–10
- quest variable 2 ≥ 20
- you carry < 1 × [[items/quest/208-token-of-friendship|Token of Friendship]]

Then:

- works on [[quests/1059-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- you get 1 × [[items/quest/208-token-of-friendship|Token of Friendship]]

If the checks fail, step `1066-31` is tried instead.
