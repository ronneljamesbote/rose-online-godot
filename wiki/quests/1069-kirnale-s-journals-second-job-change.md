---
kind: quest
id: 1069
name: Kirnale's Journals (Second Job Change)
status: in-game
npcs:
- '[[npcs/1094-armor-merchant-saki|Armor Merchant Saki]]'
- '[[npcs/1104-historian-jones|Historian Jones]]'
steps: 2
source:
  data: LIST_QUEST.STB row 1069; QSD triggers 1068-01, 1069-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Kirnale's Journals (Second Job Change)

After bringing Kirnale's relics to Saki, Saki suggested that these artifacts be placed in the care of Jones, the historian in Junon Polis. You should go and visit Jones and perhaps you'll get the chance to read some of Kirnale's Journals.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1068-01`

Happens by talking to [[npcs/1094-armor-merchant-saki|Armor Merchant Saki]].

Checks:

- you have [[quests/1068-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- you carry ≥ 1 × [[items/quest/220-kirnale-s-compass|Kirnale's Compass]]
- you carry ≥ 1 × [[items/quest/221-portable-sundial|Portable Sundial]]

Then:

- works on [[quests/1068-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- the quest becomes [[quests/1069-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]] (progress kept)

### `1069-01`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1069-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]

Then:

- works on [[quests/1069-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- 1 × [[items/quest/220-kirnale-s-compass|Kirnale's Compass]] is taken
- 1 × [[items/quest/221-portable-sundial|Portable Sundial]] is taken
- the quest becomes [[quests/1070-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]] (progress kept)
