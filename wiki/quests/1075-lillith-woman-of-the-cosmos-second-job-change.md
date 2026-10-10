---
kind: quest
id: 1075
name: Lillith, Woman of the Cosmos (Second Job Change)
status: in-game
npcs:
- '[[npcs/1096-ferrell-guild-merchant-mildun|Ferrell Guild Merchant Mildun]]'
- '[[npcs/1144-residents-hotch|Residents Hotch]]'
steps: 2
source:
  data: LIST_QUEST.STB row 1075; QSD triggers 1074-01, 1075-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Lillith, Woman of the Cosmos (Second Job Change)

After giving Mildun the Family Emblem that belonged to Lillith, he told you the bittersweet story of Lillith's love for Kirnale. Visit Hotch in Kenji's Beach to see if he'll let you read one of the love letters Lillith sent to Kirnale.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1074-01`

Happens by talking to [[npcs/1096-ferrell-guild-merchant-mildun|Ferrell Guild Merchant Mildun]].

Checks:

- you have [[quests/1074-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]
- you carry ≥ 1 × [[items/quest/223-family-emblem|Family Emblem]]

Then:

- works on [[quests/1074-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]
- 1 × [[items/quest/223-family-emblem|Family Emblem]] is taken
- the quest becomes [[quests/1075-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]] (progress kept)

### `1075-01`

Happens by talking to [[npcs/1144-residents-hotch|Residents Hotch]].

Checks:

- you have [[quests/1075-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]

Then:

- works on [[quests/1075-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]
- the quest becomes [[quests/1076-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]] (progress kept)
