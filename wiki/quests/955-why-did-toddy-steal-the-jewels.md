---
kind: quest
id: 955
name: Why Did Toddy Steal the Jewels?
status: in-game
npcs:
- '[[npcs/1007-gypsy-jewel-seller-mina|Gypsy Jewel Seller Mina]]'
- '[[npcs/1063-little-street-vendor-mile|Little Street Vendor Mile]]'
steps: 4
source:
  data: LIST_QUEST.STB row 955; QSD triggers 954-02, 955-01, 955-31, 955-32
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Why Did Toddy Steal the Jewels?

That little boy Mile said that Toddy threw the jewels into the fields. Find 10 of Mina's Rubies. It's likely that the Dalpings have picked them up…  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `954-02`

Happens by talking to [[npcs/1063-little-street-vendor-mile|Little Street Vendor Mile]].

Checks:

- you have [[quests/954-why-did-toddy-steal-the-jewels|Why Did Toddy Steal the Jewels?]]

Then:

- works on [[quests/954-why-did-toddy-steal-the-jewels|Why Did Toddy Steal the Jewels?]]
- the quest becomes [[quests/955-why-did-toddy-steal-the-jewels|Why Did Toddy Steal the Jewels?]]

### `955-01`

Happens by talking to [[npcs/1007-gypsy-jewel-seller-mina|Gypsy Jewel Seller Mina]].

Checks:

- you have [[quests/955-why-did-toddy-steal-the-jewels|Why Did Toddy Steal the Jewels?]]
- you carry ≥ 10 × [[items/quest/113-mina-s-ruby|Mina's Ruby]]

Then:

- works on [[quests/955-why-did-toddy-steal-the-jewels|Why Did Toddy Steal the Jewels?]]
- 10 × [[items/quest/113-mina-s-ruby|Mina's Ruby]] is taken
- the quest becomes [[quests/956-why-did-toddy-steal-the-jewels|Why Did Toddy Steal the Jewels?]]

### `955-31`

Checks:

- you have [[quests/955-why-did-toddy-steal-the-jewels|Why Did Toddy Steal the Jewels?]]
- a random roll 0–99 lands in 0–30
- you carry < 10 × [[items/quest/113-mina-s-ruby|Mina's Ruby]]

Then:

- works on [[quests/955-why-did-toddy-steal-the-jewels|Why Did Toddy Steal the Jewels?]]
- you get 1 × [[items/quest/113-mina-s-ruby|Mina's Ruby]]

If the checks fail, step `906-31` is tried instead.

### `955-32`

Checks:

- you have [[quests/955-why-did-toddy-steal-the-jewels|Why Did Toddy Steal the Jewels?]]
- a random roll 0–99 lands in 0–35
- you carry < 10 × [[items/quest/113-mina-s-ruby|Mina's Ruby]]

Then:

- works on [[quests/955-why-did-toddy-steal-the-jewels|Why Did Toddy Steal the Jewels?]]
- you get 1 × [[items/quest/113-mina-s-ruby|Mina's Ruby]]

If the checks fail, step `906-34` is tried instead.
