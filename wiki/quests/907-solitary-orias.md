---
kind: quest
id: 907
name: Solitary Orias
status: in-game
npcs:
- '[[npcs/1010-designer-keenu|Designer Keenu]]'
- '[[npcs/1073-ikaness-staff-orias|Ikaness Staff Orias]]'
steps: 4
source:
  data: LIST_QUEST.STB row 907; QSD triggers 906-01, 907-01, 907-02, 907-03
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Solitary Orias

Orias is very grateful for your help, but has nothing to give you in the middle of the desert. He's asked you to go to Keenu for your reward.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `906-01`

Happens by talking to [[npcs/1073-ikaness-staff-orias|Ikaness Staff Orias]].

Checks:

- you have [[quests/906-solitary-orias|Solitary Orias]]
- you carry ≥ 5 × [[items/quest/119-stone-of-nature-fire|Stone of Nature (Fire)]]
- you carry ≥ 5 × [[items/quest/120-stone-of-nature-water|Stone of Nature (Water)]]
- you carry ≥ 5 × [[items/quest/121-stone-of-nature-earth|Stone of Nature (Earth)]]

Then:

- works on [[quests/906-solitary-orias|Solitary Orias]]
- 5 × [[items/quest/119-stone-of-nature-fire|Stone of Nature (Fire)]] is taken
- 5 × [[items/quest/120-stone-of-nature-water|Stone of Nature (Water)]] is taken
- 5 × [[items/quest/121-stone-of-nature-earth|Stone of Nature (Earth)]] is taken
- the quest becomes [[quests/907-solitary-orias|Solitary Orias]]

### `907-01`

Happens by talking to [[npcs/1010-designer-keenu|Designer Keenu]].

Checks:

- you have [[quests/907-solitary-orias|Solitary Orias]]
- your Job = 2
- your Level ≥ 20
- job variable 0 = 1

Then:

- works on [[quests/907-solitary-orias|Solitary Orias]]
- you get [[items/weapon/303-animal-rod|Animal Rod]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- add 2 to job variable 0
- the quest ends (removed from your list)

### `907-02`

Happens by talking to [[npcs/1010-designer-keenu|Designer Keenu]].

Checks:

- you have [[quests/907-solitary-orias|Solitary Orias]]
- your Job = 2
- your Level ≥ 20
- job variable 0 = 1

Then:

- works on [[quests/907-solitary-orias|Solitary Orias]]
- you get [[items/weapon/333-sorcerer-s-wand|Sorcerer's Wand]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- add 2 to job variable 0
- the quest ends (removed from your list)

### `907-03`

Happens by talking to [[npcs/1010-designer-keenu|Designer Keenu]].

Checks:

- you have [[quests/907-solitary-orias|Solitary Orias]]
- your Job = 2
- your Level ≥ 20
- job variable 0 = 1

Then:

- works on [[quests/907-solitary-orias|Solitary Orias]]
- Zuly, base 1800 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- add 2 to job variable 0
- the quest ends (removed from your list)
