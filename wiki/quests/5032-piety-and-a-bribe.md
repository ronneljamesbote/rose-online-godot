---
kind: quest
id: 5032
name: Piety and a Bribe?
status: changed-from-irose
given_by:
- '[[npcs/1038-village-chief-gray|Village Chief Gray]]'
npcs:
- '[[npcs/1003-co-founder-of-the-junon-order-francis|Co-Founder of the Junon Order Francis]]'
steps: 3
source:
  data: 'godot/data/quests/5032-piety-and-a-bribe.tres; quest triggers in godot/data/quest_triggers/: qs10-001/5032-01, qs10-001/5032-02, qs10-001/5032-03'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Piety and a Bribe?

**Can't be done any more.** The Zant city hub redesign took Zant's odd jobs (fetch quests,
deliveries and stockpiling) out of the game: nobody offers this quest now. The steps below are
iROSE's and are kept in the game data.

Bring Gray's Pearl Oysters to Ulysses who is at the temple in Zant. Afterwards, return to Gray with the Elder Appointment Letter from Ulysses.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5032-01`

Happens by talking to [[npcs/1038-village-chief-gray|Village Chief Gray]].

Checks:

- your Level ≥ 7
- quest switch 16 is off

Then:

- you get the quest [[quests/5032-piety-and-a-bribe|Piety and a Bribe?]]
- works on [[quests/5032-piety-and-a-bribe|Piety and a Bribe?]]
- you get 1 × [[items/quest/506-pearl-oysters|Pearl Oysters]]

### `5032-02`

Happens by talking to [[npcs/1003-co-founder-of-the-junon-order-francis|Co-Founder of the Junon Order Francis]].

Checks:

- you have [[quests/5032-piety-and-a-bribe|Piety and a Bribe?]]
- you carry = 1 × [[items/quest/506-pearl-oysters|Pearl Oysters]]

Then:

- works on [[quests/5032-piety-and-a-bribe|Piety and a Bribe?]]
- 1 × [[items/quest/506-pearl-oysters|Pearl Oysters]] is taken
- you get 1 × [[items/quest/507-elder-appointment-letter|Elder Appointment Letter]]
- you get [[items/feet/3-waterproof-shoes|Waterproof Shoes]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- quest switch 16 on

### `5032-03`

Happens by talking to [[npcs/1038-village-chief-gray|Village Chief Gray]].

Checks:

- you have [[quests/5032-piety-and-a-bribe|Piety and a Bribe?]]
- you carry ≥ 1 × [[items/quest/507-elder-appointment-letter|Elder Appointment Letter]]

Then:

- works on [[quests/5032-piety-and-a-bribe|Piety and a Bribe?]]
- 1 × [[items/quest/507-elder-appointment-letter|Elder Appointment Letter]] is taken
- Zuly, base 700 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/56-vital-jam-1|Vital Jam (+1)]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

## Changed from iROSE

Not in the game any more: see the note at the top.
