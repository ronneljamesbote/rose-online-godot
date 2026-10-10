---
kind: quest
id: 954
name: Why Did Toddy Steal the Jewels?
status: in-game
given_by:
- '[[npcs/1007-gypsy-jewel-seller-mina|Gypsy Jewel Seller Mina]]'
npcs:
- '[[npcs/1063-little-street-vendor-mile|Little Street Vendor Mile]]'
steps: 2
source:
  data: LIST_QUEST.STB row 954; QSD triggers 954-01, 954-02
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Why Did Toddy Steal the Jewels?

Last night, Mina’s jewel shop was robbed. Toddy, the suspect, is said to have run away to the Breezy Hills. It's not too late to try to find him!  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `954-01`

Happens by talking to [[npcs/1007-gypsy-jewel-seller-mina|Gypsy Jewel Seller Mina]].

Checks:

- job variable 0 = 1
- your Job = 3
- your Level ≥ 20

Then:

- you get the quest [[quests/954-why-did-toddy-steal-the-jewels|Why Did Toddy Steal the Jewels?]]

### `954-02`

Happens by talking to [[npcs/1063-little-street-vendor-mile|Little Street Vendor Mile]].

Checks:

- you have [[quests/954-why-did-toddy-steal-the-jewels|Why Did Toddy Steal the Jewels?]]

Then:

- works on [[quests/954-why-did-toddy-steal-the-jewels|Why Did Toddy Steal the Jewels?]]
- the quest becomes [[quests/955-why-did-toddy-steal-the-jewels|Why Did Toddy Steal the Jewels?]]
