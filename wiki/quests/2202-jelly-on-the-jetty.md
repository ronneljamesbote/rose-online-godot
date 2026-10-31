---
kind: quest
id: 2202
name: Jelly on the Jetty
status: in-game
given_by:
- '[[npcs/1039-dockmaster-wade|Dockmaster Wade]]'
level: 1
steps: 3
source:
  data: 'godot/data/quests/2202-jelly-on-the-jetty.tres; quest triggers in godot/data/quest_triggers/: qa22-001/2202-01, qa22-001/2202-02, qa22-001/2202-31'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Jelly on the Jetty

Dockmaster Wade needs 6 Jelly Goo from the Mini-Jelly Beans on Tidewater Beach to seal the jetty planks.

Part of the [[zones/22-adventurer-s-plain|Adventurer's Plain]] story; see "Quests" on the zone page for the whole chain.

[[npcs/1039-dockmaster-wade|Dockmaster Wade]] offers it from level 1, after [[quests/2201-first-steps|First Steps]].

- **Start:** Bring Wade 6 Jelly Goo from the Mini-Jelly Beans on the beach.
- **End:** The jetty is sealed.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2202-01`

Happens by talking to [[npcs/1039-dockmaster-wade|Dockmaster Wade]].

Checks:

- quest switch 620 is on (you finished [[quests/2201-first-steps|First Steps]])
- quest switch 624 is off (you haven't done this quest yet)

Then:

- you get the quest [[quests/2202-jelly-on-the-jetty|Jelly on the Jetty]]

### `2202-02`

Happens by talking to [[npcs/1039-dockmaster-wade|Dockmaster Wade]].

Checks:

- you have [[quests/2202-jelly-on-the-jetty|Jelly on the Jetty]]
- you carry ≥ 6 × [[items/quest/712-jelly-goo|Jelly Goo]]

Then:

- works on [[quests/2202-jelly-on-the-jetty|Jelly on the Jetty]]
- 6 × [[items/quest/712-jelly-goo|Jelly Goo]] is taken
- experience, base 60 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 80 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get 10 × [[items/consumable/103-grapes|Grapes]]
- quest switch 624 on
- the quest ends (removed from your list)

### `2202-31`

Happens by killing [[monsters/1-mini-jelly-bean|Mini-Jelly Bean]]: its kill step `5051-34` ([[quests/5051-hunting-practice-1|Hunting Practice (1)]]) runs it next, so a kill counts for both quests.

Checks:

- you have [[quests/2202-jelly-on-the-jetty|Jelly on the Jetty]]
- a random roll 0–99 lands in 0–70
- you carry < 6 × [[items/quest/712-jelly-goo|Jelly Goo]]

Then:

- works on [[quests/2202-jelly-on-the-jetty|Jelly on the Jetty]]
- you get 1 × [[items/quest/712-jelly-goo|Jelly Goo]]

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Adventurer's Plain redesign.
