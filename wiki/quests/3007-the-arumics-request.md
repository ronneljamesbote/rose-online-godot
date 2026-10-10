---
kind: quest
id: 3007
name: The Arumics' Request
status: in-game
given_by:
- '[[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]]'
npcs:
- '[[npcs/1099-ferrell-guild-staff-hayen|Ferrell Guild Staff Hayen]]'
monsters:
- '[[monsters/121-krawfy|Krawfy]]'
steps: 5
source:
  data: LIST_QUEST.STB row 3007; QSD triggers 3007-01, 3007-02, 3007-03, 3007-04, 3007-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Arumics' Request

In order to return to Luna, the Arumics have secretly been constructing a Flying Ship. As their ally, the Arumics have asked the Junon Order to help gather construction materials. Go to Kenji's Beach and hunt Krawfy so that you can bring them 20 Krawfy Hard Shells.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3007-01`

Happens by talking to [[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]].

Checks:

- your Faction = 1
- your Level ≥ 55

Then:

- you get the quest [[quests/3007-the-arumics-request|The Arumics' Request]]

### `3007-02`

Happens by talking to [[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]].

Checks:

- you have [[quests/3007-the-arumics-request|The Arumics' Request]]
- your Faction = 1
- your Level ≤ 70
- you carry ≥ 20 × [[items/quest/315-krawfy-hard-shell|Krawfy Hard Shell]]

Then:

- works on [[quests/3007-the-arumics-request|The Arumics' Request]]
- 20 × [[items/quest/315-krawfy-hard-shell|Krawfy Hard Shell]] is taken
- you get [[items/consumable/3-health-vial-l|Health Vial (L)]], base count 5 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/22-mana-vial-m|Mana Vial (M)]], base count 5 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- add 8 to your UnionPoint1
- the quest ends (removed from your list)

If the checks fail, step `3007-03` is tried instead.

### `3007-03`

Checks:

- you have [[quests/3007-the-arumics-request|The Arumics' Request]]
- your Faction = 1
- your Level > 70
- you carry ≥ 20 × [[items/quest/315-krawfy-hard-shell|Krawfy Hard Shell]]

Then:

- works on [[quests/3007-the-arumics-request|The Arumics' Request]]
- 20 × [[items/quest/315-krawfy-hard-shell|Krawfy Hard Shell]] is taken
- you get [[items/consumable/3-health-vial-l|Health Vial (L)]], base count 5 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/22-mana-vial-m|Mana Vial (M)]], base count 5 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- add 3 to your UnionPoint1
- the quest ends (removed from your list)

### `3007-04`

Happens by talking to [[npcs/1099-ferrell-guild-staff-hayen|Ferrell Guild Staff Hayen]].

Checks:

- you have [[quests/3007-the-arumics-request|The Arumics' Request]]
- your Faction = 1

Then:

- you are moved to (5242, 5022) in [[zones/27-kenji-beach|Kenji Beach]]

### `3007-31`

Happens by killing [[monsters/121-krawfy|Krawfy]].

Checks:

- you have [[quests/3007-the-arumics-request|The Arumics' Request]]
- a random roll 0–99 lands in 0–35
- you carry < 20 × [[items/quest/315-krawfy-hard-shell|Krawfy Hard Shell]]

Then:

- works on [[quests/3007-the-arumics-request|The Arumics' Request]]
- you get 1 × [[items/quest/315-krawfy-hard-shell|Krawfy Hard Shell]]
