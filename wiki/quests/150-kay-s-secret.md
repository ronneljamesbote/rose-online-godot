---
kind: quest
id: 150
name: Kay's Secret
status: in-game
given_by:
- '[[npcs/1131-mountain-guide-kay|Mountain Guide Kay]]'
npcs:
- '[[npcs/1143-ferrell-guild-merchant-bith|Ferrell Guild Merchant Bith]]'
steps: 3
source:
  data: LIST_QUEST.STB row 150; QSD triggers 149-01, 149-02, 150-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Kay's Secret

Kay asks you to get a cup of Lunarian Coffee from Bith in Kenji's Beach if you want to hear more about his secret proposal.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `149-01`

Happens by talking to [[npcs/1131-mountain-guide-kay|Mountain Guide Kay]].

Checks:

- you have [[quests/149-an-interesting-proposal|An Interesting Proposal]]

Then:

- works on [[quests/149-an-interesting-proposal|An Interesting Proposal]]
- experience, base 30000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/150-kay-s-secret|Kay's Secret]]
- set episode variable 0 to 49

### `149-02`

Happens by talking to [[npcs/1131-mountain-guide-kay|Mountain Guide Kay]].

Checks:

- episode variable 0 = 49

Then:

- you get the quest [[quests/150-kay-s-secret|Kay's Secret]]

### `150-01`

Happens by talking to [[npcs/1143-ferrell-guild-merchant-bith|Ferrell Guild Merchant Bith]].

Checks:

- you have [[quests/150-kay-s-secret|Kay's Secret]]

Then:

- works on [[quests/150-kay-s-secret|Kay's Secret]]
- experience, base 30000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/151-lunarian-coffee|Lunarian Coffee]]
- set episode variable 0 to 50
