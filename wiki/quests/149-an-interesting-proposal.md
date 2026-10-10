---
kind: quest
id: 149
name: An Interesting Proposal
status: in-game
given_by:
- '[[npcs/1141-righteous-crusader-gallahad|Righteous Crusader Gallahad]]'
npcs:
- '[[npcs/1131-mountain-guide-kay|Mountain Guide Kay]]'
steps: 3
source:
  data: LIST_QUEST.STB row 149; QSD triggers 148-01, 148-02, 149-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# An Interesting Proposal

Gallahad told you that Kay, in the Forest of Wisdom, has an interesting proposal for Visitors and that you should visit him. What ever could this proposal be?  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `148-01`

Happens by talking to [[npcs/1141-righteous-crusader-gallahad|Righteous Crusader Gallahad]].

Checks:

- you have [[quests/148-the-royal-golden-dagger|The Royal Golden Dagger]]

Then:

- works on [[quests/148-the-royal-golden-dagger|The Royal Golden Dagger]]
- 1 × [[items/quest/510-golden-dagger|Golden Dagger]] is taken
- experience, base 40000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/gem/351-peridot-1|Peridot 1]], base count 2 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/gem/331-topaz-1|Topaz 1]], base count 2 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/149-an-interesting-proposal|An Interesting Proposal]] (progress kept)
- set episode variable 0 to 48

### `148-02`

Happens by talking to [[npcs/1141-righteous-crusader-gallahad|Righteous Crusader Gallahad]].

Checks:

- episode variable 0 = 48

Then:

- you get the quest [[quests/149-an-interesting-proposal|An Interesting Proposal]]

### `149-01`

Happens by talking to [[npcs/1131-mountain-guide-kay|Mountain Guide Kay]].

Checks:

- you have [[quests/149-an-interesting-proposal|An Interesting Proposal]]

Then:

- works on [[quests/149-an-interesting-proposal|An Interesting Proposal]]
- experience, base 30000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/150-kay-s-secret|Kay's Secret]]
- set episode variable 0 to 49
