---
kind: quest
id: 1986
name: Information on Lunar
status: in-game
npcs:
- '[[npcs/1051-arumic-resercher-lutis|Arumic Resercher Lutis]]'
- '[[npcs/1117-soldier-winters|Soldier Winters]]'
steps: 2
source:
  data: LIST_QUEST.STB row 1986; QSD triggers 1985-01, 1986-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Information on Lunar

Lutis started babbling about Luna, but maybe you should visit that planet someday. Now you need to speak to Winters, a Guard in Junon Polis, to continue your search for information on the Golden Dagger.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1985-01`

Happens by talking to [[npcs/1051-arumic-resercher-lutis|Arumic Resercher Lutis]].

Checks:

- you have [[quests/1985-the-truth-of-the-golden-dagger|The Truth of the Golden Dagger]]
- quest switch 131 is on
- quest switch 132 is off

Then:

- works on [[quests/1985-the-truth-of-the-golden-dagger|The Truth of the Golden Dagger]]
- experience, base 1000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/2-health-vial-m|Health Vial (M)]], base count 10 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/22-mana-vial-m|Mana Vial (M)]], base count 10 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/1986-information-on-lunar|Information on Lunar]] (progress kept)
- quest switch 132 on

### `1986-01`

Happens by talking to [[npcs/1117-soldier-winters|Soldier Winters]].

Checks:

- you have [[quests/1986-information-on-lunar|Information on Lunar]]
- quest switch 133 is off

Then:

- works on [[quests/1986-information-on-lunar|Information on Lunar]]
- experience, base 5000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/1988-pavrick-the-craftsman-on-lunar|Pavrick, the Craftsman on Lunar]] (progress kept)
- quest switch 133 on

If the checks fail, step `1986-02` is tried instead.
