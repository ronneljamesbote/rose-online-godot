---
kind: quest
id: 801
name: Junon Planet Identification
status: in-game
given_by:
- '[[npcs/1032-akram-minister-mairard|Akram Minister Mairard]]'
npcs:
- '[[npcs/1002-akram-kingdom-minister-warren|Akram Kingdom Minister Warren]]'
steps: 5
source:
  data: LIST_QUEST.STB row 801; QSD triggers 801-01, 851-01, 901-01, 951-01, 1001-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Junon Planet Identification

You can get a job after reaching character Level 10. Bring your identification papers to Warren in Zant, and he will give you more information.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `801-01`

Happens by talking to [[npcs/1032-akram-minister-mairard|Akram Minister Mairard]].

Checks:

- job variable 0 < 1

Then:

- you get the quest [[quests/801-junon-planet-identification|Junon Planet Identification]]
- works on [[quests/801-junon-planet-identification|Junon Planet Identification]]
- you get 1 × [[items/quest/101-identification|Identification]]
- set quest switch 0 to 1

### `851-01`

Happens by talking to [[npcs/1002-akram-kingdom-minister-warren|Akram Kingdom Minister Warren]].

Checks:

- your Level ≥ 10
- your Job = 0
- you have [[quests/801-junon-planet-identification|Junon Planet Identification]]
- you carry = 1 × [[items/quest/101-identification|Identification]]

Then:

- works on [[quests/801-junon-planet-identification|Junon Planet Identification]]
- 1 × [[items/quest/101-identification|Identification]] is taken
- the quest becomes [[quests/851-solider-job-change-quest|Solider Job Change Quest]]

### `901-01`

Happens by talking to [[npcs/1002-akram-kingdom-minister-warren|Akram Kingdom Minister Warren]].

Checks:

- your Level ≥ 10
- your Job = 0
- you have [[quests/801-junon-planet-identification|Junon Planet Identification]]
- you carry = 1 × [[items/quest/101-identification|Identification]]

Then:

- works on [[quests/801-junon-planet-identification|Junon Planet Identification]]
- 1 × [[items/quest/101-identification|Identification]] is taken
- the quest becomes [[quests/901-muse-job-change-quest|Muse Job Change Quest]]

### `951-01`

Happens by talking to [[npcs/1002-akram-kingdom-minister-warren|Akram Kingdom Minister Warren]].

Checks:

- your Level ≥ 10
- your Job = 0
- you have [[quests/801-junon-planet-identification|Junon Planet Identification]]
- you carry = 1 × [[items/quest/101-identification|Identification]]

Then:

- works on [[quests/801-junon-planet-identification|Junon Planet Identification]]
- 1 × [[items/quest/101-identification|Identification]] is taken
- the quest becomes [[quests/951-hawker-job-change-quest|Hawker Job Change Quest]]

### `1001-01`

Happens by talking to [[npcs/1002-akram-kingdom-minister-warren|Akram Kingdom Minister Warren]].

Checks:

- your Level ≥ 10
- your Job = 0
- you have [[quests/801-junon-planet-identification|Junon Planet Identification]]
- you carry = 1 × [[items/quest/101-identification|Identification]]

Then:

- works on [[quests/801-junon-planet-identification|Junon Planet Identification]]
- 1 × [[items/quest/101-identification|Identification]] is taken
- the quest becomes [[quests/1001-dealer-job-change-quest|Dealer Job Change Quest]]
