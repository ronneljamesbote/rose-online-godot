---
kind: quest
id: 1008
name: The Humble Smith
status: in-game
given_by:
- '[[npcs/1004-ferrell-guild-staff-crow|Ferrell Guild Staff Crow]]'
steps: 7
source:
  data: LIST_QUEST.STB row 1008; QSD triggers 1008-01, 1008-02, 1008-03, 1008-04, 1008-05, 1008-31, 1008-32
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Humble Smith

Crow told you that he can make you an Iron Rifle if you bring him 10 Aqua Molars. You can get Aqua Molars from Aqua Rangers and Aqua Hunters.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1008-01`

Happens by talking to [[npcs/1004-ferrell-guild-staff-crow|Ferrell Guild Staff Crow]].

Checks:

- job variable 0 = 3
- your Job = 4
- your Level ≥ 30

Then:

- you get the quest [[quests/1008-the-humble-smith|The Humble Smith]]

### `1008-02`

Happens by talking to [[npcs/1004-ferrell-guild-staff-crow|Ferrell Guild Staff Crow]].

Checks:

- you have [[quests/1008-the-humble-smith|The Humble Smith]]
- you carry ≥ 10 × [[items/quest/118-aqua-molar|Aqua Molar]]

Then:

- works on [[quests/1008-the-humble-smith|The Humble Smith]]
- 10 × [[items/quest/118-aqua-molar|Aqua Molar]] is taken
- set quest switch 0 to 1

### `1008-03`

Happens by talking to [[npcs/1004-ferrell-guild-staff-crow|Ferrell Guild Staff Crow]].

Checks:

- you have [[quests/1008-the-humble-smith|The Humble Smith]]
- quest switch 0 = 1

Then:

- set quest switch 0 to 0

### `1008-04`

Happens by talking to [[npcs/1004-ferrell-guild-staff-crow|Ferrell Guild Staff Crow]].

Checks:

- you have [[quests/1008-the-humble-smith|The Humble Smith]]
- quest switch 0 = 1

Then:

- works on [[quests/1008-the-humble-smith|The Humble Smith]]
- set job variable 0 to 6
- you get [[items/weapon/234-iron-rifle|Iron Rifle]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `1008-05`

Happens by talking to [[npcs/1004-ferrell-guild-staff-crow|Ferrell Guild Staff Crow]].

Checks:

- you have [[quests/1008-the-humble-smith|The Humble Smith]]
- quest switch 0 = 1

Then:

- works on [[quests/1008-the-humble-smith|The Humble Smith]]
- set job variable 0 to 6
- you get [[items/weapon/262-basic-launcher|Basic Launcher]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `1008-31`

Checks:

- you have [[quests/1008-the-humble-smith|The Humble Smith]]
- a random roll 0–99 lands in 0–40
- you carry < 10 × [[items/quest/118-aqua-molar|Aqua Molar]]

Then:

- works on [[quests/1008-the-humble-smith|The Humble Smith]]
- you get 1 × [[items/quest/118-aqua-molar|Aqua Molar]]

### `1008-32`

Checks:

- you have [[quests/1008-the-humble-smith|The Humble Smith]]
- you carry < 10 × [[items/quest/118-aqua-molar|Aqua Molar]]
- a random roll 0–99 lands in 0–40

Then:

- works on [[quests/1008-the-humble-smith|The Humble Smith]]
- you get 1 × [[items/quest/118-aqua-molar|Aqua Molar]]
