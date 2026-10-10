---
kind: quest
id: 1984
name: The Ominous Kenji Stone
status: in-game
npcs:
- '[[npcs/1141-righteous-crusader-gallahad|Righteous Crusader Gallahad]]'
steps: 4
source:
  data: LIST_QUEST.STB row 1984; QSD triggers 1983-01, 1984-01, 1984-02, 1984-03
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Ominous Kenji Stone

Gallahad suggests that you investigate the area around the Kenji Stone. Find out what you can over there, and return to Gallahad.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1983-01`

Happens by talking to [[npcs/1141-righteous-crusader-gallahad|Righteous Crusader Gallahad]].

Checks:

- you have [[quests/1983-the-search-for-the-prince|The Search for the Prince]]
- quest switch 129 is on
- quest switch 130 is off

Then:

- works on [[quests/1983-the-search-for-the-prince|The Search for the Prince]]
- experience, base 1000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/1984-the-ominous-kenji-stone|The Ominous Kenji Stone]]
- quest switch 130 on

### `1984-01`

Happens by talking to [[npcs/1141-righteous-crusader-gallahad|Righteous Crusader Gallahad]].

Checks:

- you have [[quests/1984-the-ominous-kenji-stone|The Ominous Kenji Stone]]
- quest switch 0 = 1
- quest switch 130 is on
- quest switch 131 is off

Then:

- works on [[quests/1984-the-ominous-kenji-stone|The Ominous Kenji Stone]]
- experience, base 5000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/2-health-vial-m|Health Vial (M)]], base count 10 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/22-mana-vial-m|Mana Vial (M)]], base count 10 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get 1 × [[items/quest/510-golden-dagger|Golden Dagger]]
- the quest becomes [[quests/1985-the-truth-of-the-golden-dagger|The Truth of the Golden Dagger]] (progress kept)
- quest switch 131 on

### `1984-02`

Checks:

- you have [[quests/1984-the-ominous-kenji-stone|The Ominous Kenji Stone]]
- quest switch 0 = 0

Then:

- set quest switch 0 to 1

If the checks fail, step `143-03` is tried instead.

### `1984-03`

Checks:

- quest switch 130 is on
- quest switch 131 is off

Then:

- you get the quest [[quests/1984-the-ominous-kenji-stone|The Ominous Kenji Stone]]
