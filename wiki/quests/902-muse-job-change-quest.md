---
kind: quest
id: 902
name: Muse Job Change Quest
status: in-game
npcs:
- '[[npcs/1006-arumic-merchant-tryteh|Arumic Merchant Tryteh]]'
monsters:
- '[[monsters/41-flanae|Flanae]]'
- '[[monsters/42-elder-flanae|Elder Flanae]]'
steps: 5
source:
  data: 'godot/data/quests/902-muse-job-change-quest.tres; quest triggers in godot/data/quest_triggers/: qj000-01/901-02, qj000-01/902-01, qj000-01/902-02, qn-041/902-31, qn-042/902-32'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Muse Job Change Quest

Tryteh told you about a recipe for creating a simple sleeping potion. First, bring 20 Flanae Seeds to Tryteh.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `901-02`

Happens by talking to [[npcs/1006-arumic-merchant-tryteh|Arumic Merchant Tryteh]].

Checks:

- you have [[quests/901-muse-job-change-quest|Muse Job Change Quest]]

Then:

- works on [[quests/901-muse-job-change-quest|Muse Job Change Quest]]
- the quest becomes [[quests/902-muse-job-change-quest|Muse Job Change Quest]]

### `902-01`

Happens by talking to [[npcs/1006-arumic-merchant-tryteh|Arumic Merchant Tryteh]].

Checks:

- you have [[quests/902-muse-job-change-quest|Muse Job Change Quest]]
- you carry ≥ 20 × [[items/quest/106-flanae-seed|Flanae Seed]]

Then:

- works on [[quests/902-muse-job-change-quest|Muse Job Change Quest]]
- 20 × [[items/quest/106-flanae-seed|Flanae Seed]] is taken
- the quest becomes [[quests/903-muse-job-change-quest|Muse Job Change Quest]]
- you get [[items/consumable/1-health-vial-s|Health Vial (S)]], base count 5 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])

### `902-02`

Happens by talking to [[npcs/1006-arumic-merchant-tryteh|Arumic Merchant Tryteh]].

Checks:

- you have [[quests/902-muse-job-change-quest|Muse Job Change Quest]]
- you carry ≥ 20 × [[items/quest/106-flanae-seed|Flanae Seed]]

Then:

- works on [[quests/902-muse-job-change-quest|Muse Job Change Quest]]
- set quest switch 4 to 1

### `902-31`

Happens by killing [[monsters/41-flanae|Flanae]].

Checks:

- you have [[quests/902-muse-job-change-quest|Muse Job Change Quest]]
- a random roll 0–99 lands in 0–90
- you carry < 20 × [[items/quest/106-flanae-seed|Flanae Seed]]

Then:

- works on [[quests/902-muse-job-change-quest|Muse Job Change Quest]]
- you get 1 × [[items/quest/106-flanae-seed|Flanae Seed]]
- then runs step `2206-31` ([[quests/2206-tibbs-farm|Tibbs' Farm]])

If the checks fail, step `5035-31` is tried instead.

### `902-32`

Happens by killing [[monsters/42-elder-flanae|Elder Flanae]].

Checks:

- you have [[quests/902-muse-job-change-quest|Muse Job Change Quest]]
- a random roll 0–99 lands in 0–90
- you carry < 20 × [[items/quest/106-flanae-seed|Flanae Seed]]

Then:

- works on [[quests/902-muse-job-change-quest|Muse Job Change Quest]]
- you get 1 × [[items/quest/106-flanae-seed|Flanae Seed]]
- then runs step `2206-31` ([[quests/2206-tibbs-farm|Tibbs' Farm]])

If the checks fail, step `5035-32` is tried instead.

## Changed from iROSE

The Adventurer's Plain redesign made this quest's kill steps also run the matching steps of the plain's new quests, so one kill counts for both.
