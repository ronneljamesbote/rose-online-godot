---
kind: quest
id: 1989
name: The Royal Golden Dagger
status: in-game
npcs:
- '[[npcs/1141-righteous-crusader-gallahad|Righteous Crusader Gallahad]]'
- '[[npcs/1181-smith-pavrick|Smith Pavrick]]'
steps: 3
source:
  data: LIST_QUEST.STB row 1989; QSD triggers 1988-03, 1988-04, 1989-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Royal Golden Dagger

Pavrick confirmed that the Golden Dagger really did belong to the royal family. Hence, the warrior who saved Kenji's Beach was the Prince. But where is he now? First, you need to return the Golden Dagger to Gallahad.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1988-03`

Happens by talking to [[npcs/1181-smith-pavrick|Smith Pavrick]].

Checks:

- you have [[quests/1988-pavrick-the-craftsman-on-lunar|Pavrick, the Craftsman on Lunar]]
- quest variable 1 = 20

Then:

- works on [[quests/1988-pavrick-the-craftsman-on-lunar|Pavrick, the Craftsman on Lunar]]
- 1 × [[items/quest/511-luna-whiskey|Luna Whiskey]] is taken
- experience, base 8000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 15000 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/2-health-vial-m|Health Vial (M)]], base count 5 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/22-mana-vial-m|Mana Vial (M)]], base count 5 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- quest switch 134 on
- the quest becomes [[quests/1989-the-royal-golden-dagger|The Royal Golden Dagger]] (progress kept)

### `1988-04`

Checks:

- quest switch 134 is on
- quest switch 135 is off

Then:

- you get the quest [[quests/1989-the-royal-golden-dagger|The Royal Golden Dagger]]
- works on [[quests/1989-the-royal-golden-dagger|The Royal Golden Dagger]]
- you get 1 × [[items/quest/510-golden-dagger|Golden Dagger]]

### `1989-01`

Happens by talking to [[npcs/1141-righteous-crusader-gallahad|Righteous Crusader Gallahad]].

Checks:

- you have [[quests/1989-the-royal-golden-dagger|The Royal Golden Dagger]]
- quest switch 135 is off

Then:

- works on [[quests/1989-the-royal-golden-dagger|The Royal Golden Dagger]]
- 1 × [[items/quest/510-golden-dagger|Golden Dagger]] is taken
- experience, base 12000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/gem/351-peridot-1|Peridot 1]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/gem/331-topaz-1|Topaz 1]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)
- quest switch 135 on
