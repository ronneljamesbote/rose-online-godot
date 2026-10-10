---
kind: quest
id: 1076
name: Lillith, Woman of the Cosmos (Second Job Change)
status: in-game
npcs:
- '[[npcs/1144-residents-hotch|Residents Hotch]]'
steps: 3
source:
  data: LIST_QUEST.STB row 1076; QSD triggers 1075-01, 1076-01, 1076-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Lillith, Woman of the Cosmos (Second Job Change)

Hotch has told you that the Krawfies stole the love letters written by Lillith that he has collected. You've got to recover 5 Bottled Love Letters from Krawfy Kings.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1075-01`

Happens by talking to [[npcs/1144-residents-hotch|Residents Hotch]].

Checks:

- you have [[quests/1075-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]

Then:

- works on [[quests/1075-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]
- the quest becomes [[quests/1076-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]] (progress kept)

### `1076-01`

Happens by talking to [[npcs/1144-residents-hotch|Residents Hotch]].

Checks:

- you have [[quests/1076-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]
- you carry ≥ 5 × [[items/quest/224-bottled-love-letter|Bottled Love Letter]]

Then:

- works on [[quests/1076-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]
- the quest becomes [[quests/1077-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]] (progress kept)

### `1076-31`

Checks:

- you have [[quests/1076-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]
- a random roll 0–99 lands in 0–40
- you carry < 5 × [[items/quest/224-bottled-love-letter|Bottled Love Letter]]

Then:

- works on [[quests/1076-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]
- you get 1 × [[items/quest/224-bottled-love-letter|Bottled Love Letter]]
