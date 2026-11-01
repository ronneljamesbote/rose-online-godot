---
kind: quest
id: 1002
name: Dealer Job Change Quest
status: changed-from-irose
npcs:
- '[[npcs/1004-ferrell-guild-staff-crow|Ferrell Guild Staff Crow]]'
- '[[npcs/1016-livestock-farmer-lampa|Livestock Farmer Lampa]]'
- '[[npcs/1036-ferrell-guild-staff-seyon|Ferrell Guild Staff Seyon]]'
steps: 4
source:
  data: 'godot/data/quests/1002-dealer-job-change-quest.tres; quest triggers in godot/data/quest_triggers/: qj000-01/1001-02, qj000-01/1002-01, qj000-01/1002-02, qj000-01/1002-03'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Dealer Job Change Quest

**Can't be done any more.** First jobs come from the job mentors at the Sandwind Caravan Camp in
the [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]] (see
[[quests/801-junon-planet-identification|Junon Planet Identification]]), and the Zant city hub
redesign took this quest's steps out of the Zant NPCs' dialogs. The steps below are iROSE's and
are kept in the game data.

Crow asked you to bring him Fleece and Small Metal Boards, since their market prices have risen. For this, you need to meet Lampa in Zant Village, and Seyon in the Adventurer's Plains.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1001-02`

Happens by talking to [[npcs/1004-ferrell-guild-staff-crow|Ferrell Guild Staff Crow]].

Checks:

- you have [[quests/1001-dealer-job-change-quest|Dealer Job Change Quest]]

Then:

- works on [[quests/1001-dealer-job-change-quest|Dealer Job Change Quest]]
- the quest becomes [[quests/1002-dealer-job-change-quest|Dealer Job Change Quest]]

### `1002-01`

Happens by talking to [[npcs/1004-ferrell-guild-staff-crow|Ferrell Guild Staff Crow]].

Checks:

- you have [[quests/1002-dealer-job-change-quest|Dealer Job Change Quest]]
- you carry = 5 × [[items/quest/111-small-metal-board|Small Metal Board]]
- you carry = 5 × [[items/quest/112-fleece|Fleece]]

Then:

- works on [[quests/1002-dealer-job-change-quest|Dealer Job Change Quest]]
- 5 × [[items/quest/111-small-metal-board|Small Metal Board]] is taken
- 5 × [[items/quest/112-fleece|Fleece]] is taken
- the quest becomes [[quests/1003-dealer-job-change-quest|Dealer Job Change Quest]]
- you get [[items/consumable/1-health-vial-s|Health Vial (S)]], base count 5 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])

### `1002-02`

Happens by talking to [[npcs/1016-livestock-farmer-lampa|Livestock Farmer Lampa]].

Checks:

- you have [[quests/1002-dealer-job-change-quest|Dealer Job Change Quest]]
- you carry < 5 × [[items/quest/112-fleece|Fleece]]

Then:

- works on [[quests/1002-dealer-job-change-quest|Dealer Job Change Quest]]
- you get 5 × [[items/quest/112-fleece|Fleece]]

### `1002-03`

Happens by talking to [[npcs/1036-ferrell-guild-staff-seyon|Ferrell Guild Staff Seyon]].

Checks:

- you have [[quests/1002-dealer-job-change-quest|Dealer Job Change Quest]]
- you carry < 5 × [[items/quest/111-small-metal-board|Small Metal Board]]

Then:

- works on [[quests/1002-dealer-job-change-quest|Dealer Job Change Quest]]
- you get 5 × [[items/quest/111-small-metal-board|Small Metal Board]]

## Changed from iROSE

Not in the game any more: see the note at the top.
