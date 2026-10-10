---
kind: quest
id: 905
name: Solitary Orias
status: in-game
given_by:
- '[[npcs/1010-designer-keenu|Designer Keenu]]'
npcs:
- '[[npcs/1073-ikaness-staff-orias|Ikaness Staff Orias]]'
steps: 2
source:
  data: LIST_QUEST.STB row 905; QSD triggers 905-01, 905-02
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Solitary Orias

Keenu asked if you could help Orias who is having trouble combating the strange forces in the El Verloon Desert.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `905-01`

Happens by talking to [[npcs/1010-designer-keenu|Designer Keenu]].

Checks:

- your Job = 2
- your Level ≥ 20
- job variable 0 = 1

Then:

- you get the quest [[quests/905-solitary-orias|Solitary Orias]]

### `905-02`

Happens by talking to [[npcs/1073-ikaness-staff-orias|Ikaness Staff Orias]].

Checks:

- you have [[quests/905-solitary-orias|Solitary Orias]]

Then:

- works on [[quests/905-solitary-orias|Solitary Orias]]
- the quest becomes [[quests/906-solitary-orias|Solitary Orias]]
