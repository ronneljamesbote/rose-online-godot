---
kind: quest
id: 2203
name: Bones on the Beach
status: in-game
given_by:
- '[[npcs/1039-dockmaster-wade|Dockmaster Wade]]'
npcs:
- '[[npcs/1046-beachcomber-old-tam|Beachcomber Old Tam]]'
level: 1
steps: 3
source:
  data: 'godot/data/quests/2203-bones-on-the-beach.tres; quest triggers in godot/data/quest_triggers/: qa22-001/2203-01, qa22-001/2203-02, qa22-001/2203-11'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Bones on the Beach

Wade asked you to walk up the shore to the old whale bones, where Old Tam combs the beach, and bring back proof that you went.

Part of the [[zones/22-adventurer-s-plain|Adventurer's Plain]] story; see "Quests" on the zone page for the whole chain.

[[npcs/1039-dockmaster-wade|Dockmaster Wade]] offers it from level 1, after [[quests/2202-jelly-on-the-jetty|Jelly on the Jetty]].

- **Start:** Find Old Tam at the whale bones north along the beach.
- **End:** Wade has his proof.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2203-01`

Happens by talking to [[npcs/1039-dockmaster-wade|Dockmaster Wade]].

Checks:

- quest switch 624 is on (you finished [[quests/2202-jelly-on-the-jetty|Jelly on the Jetty]])
- quest switch 625 is off (you haven't done this quest yet)

Then:

- you get the quest [[quests/2203-bones-on-the-beach|Bones on the Beach]]

### `2203-02`

Happens by talking to [[npcs/1039-dockmaster-wade|Dockmaster Wade]].

Checks:

- you have [[quests/2203-bones-on-the-beach|Bones on the Beach]]
- you carry ≥ 1 × [[items/quest/713-carved-whale-tooth|Carved Whale Tooth]]

Then:

- works on [[quests/2203-bones-on-the-beach|Bones on the Beach]]
- 1 × [[items/quest/713-carved-whale-tooth|Carved Whale Tooth]] is taken
- experience, base 80 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 100 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get 1 × [[items/hands/655-riverford-gloves|Riverford Gloves]]
- quest switch 625 on
- the quest ends (removed from your list)

### `2203-11`

Happens by talking to [[npcs/1046-beachcomber-old-tam|Beachcomber Old Tam]].

Checks:

- you have [[quests/2203-bones-on-the-beach|Bones on the Beach]]
- you carry < 1 × [[items/quest/713-carved-whale-tooth|Carved Whale Tooth]]

Then:

- works on [[quests/2203-bones-on-the-beach|Bones on the Beach]]
- you get 1 × [[items/quest/713-carved-whale-tooth|Carved Whale Tooth]]

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Adventurer's Plain redesign.
