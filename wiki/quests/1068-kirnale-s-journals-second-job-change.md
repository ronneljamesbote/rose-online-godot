---
kind: quest
id: 1068
name: Kirnale's Journals (Second Job Change)
status: in-game
npcs:
- '[[npcs/1094-armor-merchant-saki|Armor Merchant Saki]]'
steps: 5
source:
  data: LIST_QUEST.STB row 1068; QSD triggers 1053-02, 1068-01, 1068-31, 1068-32, 1068-33
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Kirnale's Journals (Second Job Change)

When you inquired about the Second Job, Saki told you about Kirnale, the legendary explorer. Upon hearing of Kirnale's exploits, you wanted to see the long lost Portable Sundial and Kirnale's Compass for yourself. There's a good chance that Krawfy King in Kenji's Beach may have these items...  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1053-02`

Happens by talking to [[npcs/1094-armor-merchant-saki|Armor Merchant Saki]].

Checks:

- you have [[quests/1053-road-to-second-job-change|Road to Second Job Change]]

Then:

- works on [[quests/1053-road-to-second-job-change|Road to Second Job Change]]
- the quest becomes [[quests/1068-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]] (progress kept)

### `1068-01`

Happens by talking to [[npcs/1094-armor-merchant-saki|Armor Merchant Saki]].

Checks:

- you have [[quests/1068-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- you carry ≥ 1 × [[items/quest/220-kirnale-s-compass|Kirnale's Compass]]
- you carry ≥ 1 × [[items/quest/221-portable-sundial|Portable Sundial]]

Then:

- works on [[quests/1068-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- the quest becomes [[quests/1069-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]] (progress kept)

### `1068-31`

Checks:

- you have [[quests/1068-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- quest variable 0 < 5

Then:

- works on [[quests/1068-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- add 1 to quest variable 0

If the checks fail, step `1068-32` is tried instead.

### `1068-32`

Checks:

- you have [[quests/1068-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- a random roll 0–99 lands in 0–30
- quest variable 0 ≥ 5
- you carry < 1 × [[items/quest/220-kirnale-s-compass|Kirnale's Compass]]

Then:

- works on [[quests/1068-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- you get 1 × [[items/quest/220-kirnale-s-compass|Kirnale's Compass]]

If the checks fail, step `1068-33` is tried instead.

### `1068-33`

Checks:

- you have [[quests/1068-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- a random roll 0–99 lands in 0–30
- quest variable 0 ≥ 5
- you carry < 1 × [[items/quest/221-portable-sundial|Portable Sundial]]

Then:

- works on [[quests/1068-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- you get 1 × [[items/quest/221-portable-sundial|Portable Sundial]]

If the checks fail, step `1076-31` is tried instead.
