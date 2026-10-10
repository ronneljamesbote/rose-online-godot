---
kind: quest
id: 1060
name: 'Lucid: King of Legend (Second Job Change)'
status: in-game
npcs:
- '[[npcs/1064-weapon-craftsman-mairath|Weapon Craftsman Mairath]]'
- '[[npcs/1104-historian-jones|Historian Jones]]'
steps: 2
source:
  data: LIST_QUEST.STB row 1060; QSD triggers 1059-02, 1060-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Lucid: King of Legend (Second Job Change)

Mairath has restored Lucid's Sword using the sword fragments and the Token of Friendship. Although the idea of keeping the sword is tempting, you should donate it to the Kingdom's Museum by speaking to Jones.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1059-02`

Happens by talking to [[npcs/1064-weapon-craftsman-mairath|Weapon Craftsman Mairath]].

Checks:

- you have [[quests/1059-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- quest switch 0 = 1

Then:

- works on [[quests/1059-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- you get 1 × [[items/quest/207-lucid-s-sword|Lucid's Sword]]
- the quest becomes [[quests/1060-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]] (progress kept)

### `1060-01`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1060-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]

Then:

- works on [[quests/1060-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- 1 × [[items/quest/207-lucid-s-sword|Lucid's Sword]] is taken
- you get 1 × [[items/quest/209-jones-recommendation|Jones' Recommendation]]
- the quest becomes [[quests/1061-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]] (progress kept)
