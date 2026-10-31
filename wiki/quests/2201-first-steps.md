---
kind: quest
id: 2201
name: First Steps
status: in-game
given_by:
- '[[npcs/1031-visitor-guide-ronway|Visitor Guide Ronway]]'
npcs:
- '[[npcs/1030-visitor-guide-fairy-of-arua|Visitor Guide Fairy of Arua]]'
- '[[npcs/1034-smith-ronk|Smith Ronk]]'
- '[[npcs/1035-little-street-vendor-pony|Little Street Vendor Pony]]'
level: 1
steps: 5
source:
  data: 'godot/data/quests/2201-first-steps.tres; quest triggers in godot/data/quest_triggers/: qa22-001/2201-01, qa22-001/2201-02, qa22-001/2201-11, qa22-001/2201-12, qa22-001/2201-13'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# First Steps

Ronway asked you to meet the people of Arua's Landing: the Fairy of Arua, Smith Ronk and Pony the street vendor. Then go back to him.

Part of the [[zones/22-adventurer-s-plain|Adventurer's Plain]] story; see "Quests" on the zone page for the whole chain.

[[npcs/1031-visitor-guide-ronway|Visitor Guide Ronway]] offers it from level 1.

- **Start:** Meet the Fairy of Arua, Ronk and Pony, then come back to Ronway.
- **End:** Ronway welcomed you to the Adventurer's Plain.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2201-01`

Happens by talking to [[npcs/1031-visitor-guide-ronway|Visitor Guide Ronway]].

Checks:

- quest switch 620 is off (you haven't done this quest yet)

Then:

- you get the quest [[quests/2201-first-steps|First Steps]]

### `2201-02`

Happens by talking to [[npcs/1031-visitor-guide-ronway|Visitor Guide Ronway]].

Checks:

- you have [[quests/2201-first-steps|First Steps]]
- quest switch 621 is on (you met the Fairy of Arua)
- quest switch 622 is on (you met Ronk)
- quest switch 623 is on (you met Pony)

Then:

- works on [[quests/2201-first-steps|First Steps]]
- experience, base 40 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 50 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get 5 × [[items/consumable/1-health-vial-s|Health Vial (S)]]
- quest switch 620 on
- the quest ends (removed from your list)

### `2201-11`

Happens by talking to [[npcs/1030-visitor-guide-fairy-of-arua|Visitor Guide Fairy of Arua]].

Checks:

- you have [[quests/2201-first-steps|First Steps]]
- quest switch 621 is off (not met yet)

Then:

- works on [[quests/2201-first-steps|First Steps]]
- quest switch 621 on

### `2201-12`

Happens by talking to [[npcs/1034-smith-ronk|Smith Ronk]].

Checks:

- you have [[quests/2201-first-steps|First Steps]]
- quest switch 622 is off (not met yet)

Then:

- works on [[quests/2201-first-steps|First Steps]]
- quest switch 622 on

### `2201-13`

Happens by talking to [[npcs/1035-little-street-vendor-pony|Little Street Vendor Pony]].

Checks:

- you have [[quests/2201-first-steps|First Steps]]
- quest switch 623 is off (not met yet)

Then:

- works on [[quests/2201-first-steps|First Steps]]
- quest switch 623 on

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Adventurer's Plain redesign.
