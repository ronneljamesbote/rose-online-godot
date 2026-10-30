---
kind: quest
id: 801
name: Junon Planet Identification
status: changed-from-irose
given_by:
- '[[npcs/1032-akram-minister-mairard|Akram Minister Mairard]]'
npcs:
- '[[npcs/1002-akram-kingdom-minister-warren|Akram Kingdom Minister Warren]]'
- '[[npcs/1080-caravan-sergeant-brakka|Caravan Sergeant Brakka]]'
- '[[npcs/1119-oasis-sage-orin|Oasis Sage Orin]]'
- '[[npcs/1120-outrider-kesh|Outrider Kesh]]'
- '[[npcs/1124-caravan-broker-harlan|Caravan Broker Harlan]]'
steps: 6
source:
  data: 'godot/data/quests/801-junon-planet-identification.tres; quest triggers in godot/data/quest_triggers/: qj000-01/801-01, qj000-01/851-01, qj000-01/901-01, qj000-01/951-01, qj000-01/1001-01, ql21-001/801-mentor'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Junon Planet Identification

Every new character starts with this quest and its papers (see [[rules/accounts|Accounts]]).

You can get a job after reaching character Level 10. Bring your identification papers to the job mentors at the Sandwind Caravan Camp in the Valley of Luxem Tower: Sergeant Brakka for Soldiers, Sage Orin for Muses, Outrider Kesh for Hawkers and Broker Harlan for Dealers.

The mentors don't ask for the papers: their trials
([[quests/2113-the-sergeant-s-trial|The Sergeant's Trial]],
[[quests/2114-the-sage-s-trial|The Sage's Trial]],
[[quests/2115-the-outrider-s-trial|The Outrider's Trial]],
[[quests/2116-the-broker-s-trial|The Broker's Trial]]) run step `801-mentor` when they change
your job, which takes the papers and ends this quest. Warren in Zant no longer takes the
papers: his first-job choice sends you to the mentors, so his steps below (`851-01`, `901-01`,
`951-01`, `1001-01`) are iROSE's and can't be reached.

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

### `801-mentor`

Happens when a mentor's trial in the Valley of Luxem Tower changes your job.

Checks:

- you have [[quests/801-junon-planet-identification|Junon Planet Identification]]

Then:

- works on [[quests/801-junon-planet-identification|Junon Planet Identification]]
- the quest ends (removed from your list)
- 1 × [[items/quest/101-identification|Identification]] is taken

## Changed from iROSE

- iROSE sends you to Warren in Zant, who turns this quest into his job change quest. Ours
  sends you to the Luxem Valley job mentors, whose trials end it; Warren doesn't take the
  papers any more.
- New characters start with it instead of getting it on leaving Birth Island.
