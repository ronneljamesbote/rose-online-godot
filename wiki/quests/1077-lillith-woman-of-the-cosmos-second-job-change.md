---
kind: quest
id: 1077
name: Lillith, Woman of the Cosmos (Second Job Change)
status: in-game
npcs:
- '[[npcs/1104-historian-jones|Historian Jones]]'
- '[[npcs/1144-residents-hotch|Residents Hotch]]'
steps: 2
source:
  data: LIST_QUEST.STB row 1077; QSD triggers 1076-01, 1077-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Lillith, Woman of the Cosmos (Second Job Change)

After retrieving the love letters, Hotch has told you to bring them to Jones, the historian in Junon Polis, so that he can take good care of these historical relics.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1076-01`

Happens by talking to [[npcs/1144-residents-hotch|Residents Hotch]].

Checks:

- you have [[quests/1076-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]
- you carry ≥ 5 × [[items/quest/224-bottled-love-letter|Bottled Love Letter]]

Then:

- works on [[quests/1076-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]
- the quest becomes [[quests/1077-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]] (progress kept)

### `1077-01`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1077-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]

Then:

- works on [[quests/1077-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]
- 5 × [[items/quest/224-bottled-love-letter|Bottled Love Letter]] is taken
- the quest becomes [[quests/1078-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]] (progress kept)
