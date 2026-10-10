---
kind: quest
id: 3607
name: Weak Mana Waves from Lunar
status: in-game
given_by:
- '[[npcs/1112-arumic-researcher-carasia|Arumic Researcher Carasia]]'
steps: 4
source:
  data: LIST_QUEST.STB row 3607; QSD triggers 3607-01, 3607-02, 3607-03, 3607-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Weak Mana Waves from Lunar

We've been unsuccessful in receiving Mana Wave messages from the planet of Luna, so we must construct a Mana Tower to receive these weak transmissions. Contribute to the Arumic Mana Tower construction by hunting Krawfy Warriors to collect 25 Krawfy Long Antenna.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3607-01`

Happens by talking to [[npcs/1112-arumic-researcher-carasia|Arumic Researcher Carasia]].

Checks:

- your Faction = 4
- your Level ≥ 55

Then:

- you get the quest [[quests/3607-weak-mana-waves-from-lunar|Weak Mana Waves from Lunar]]

### `3607-02`

Happens by talking to [[npcs/1112-arumic-researcher-carasia|Arumic Researcher Carasia]].

Checks:

- you have [[quests/3607-weak-mana-waves-from-lunar|Weak Mana Waves from Lunar]]
- your Faction = 4
- your Level ≤ 70
- you carry ≥ 25 × [[items/quest/316-krawfy-long-antenna|Krawfy Long Antenna]]

Then:

- works on [[quests/3607-weak-mana-waves-from-lunar|Weak Mana Waves from Lunar]]
- 25 × [[items/quest/316-krawfy-long-antenna|Krawfy Long Antenna]] is taken
- you get [[items/consumable/5-health-bottle-m|Health Bottle (M)]] (item count: 5)
- you get [[items/consumable/22-mana-vial-m|Mana Vial (M)]] (item count: 5)
- add 8 to your UnionPoint4
- the quest ends (removed from your list)

If the checks fail, step `3607-03` is tried instead.

### `3607-03`

Checks:

- you have [[quests/3607-weak-mana-waves-from-lunar|Weak Mana Waves from Lunar]]
- your Faction = 4
- your Level > 70
- you carry ≥ 25 × [[items/quest/316-krawfy-long-antenna|Krawfy Long Antenna]]

Then:

- works on [[quests/3607-weak-mana-waves-from-lunar|Weak Mana Waves from Lunar]]
- 25 × [[items/quest/316-krawfy-long-antenna|Krawfy Long Antenna]] is taken
- you get [[items/consumable/5-health-bottle-m|Health Bottle (M)]] (item count: 5)
- you get [[items/consumable/22-mana-vial-m|Mana Vial (M)]] (item count: 5)
- add 3 to your UnionPoint4
- the quest ends (removed from your list)

### `3607-31`

Checks:

- you have [[quests/3607-weak-mana-waves-from-lunar|Weak Mana Waves from Lunar]]
- a random roll 0–99 lands in 0–33
- you carry < 25 × [[items/quest/316-krawfy-long-antenna|Krawfy Long Antenna]]

Then:

- works on [[quests/3607-weak-mana-waves-from-lunar|Weak Mana Waves from Lunar]]
- you get 1 × [[items/quest/316-krawfy-long-antenna|Krawfy Long Antenna]]
