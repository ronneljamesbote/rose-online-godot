---
kind: quest
id: 1981
name: Zeppastone Wind Gem
status: in-game
npcs:
- '[[npcs/1062-smith-punwell|Smith Punwell]]'
monsters:
- '[[monsters/261-goblin-jar|Goblin Jar]]'
steps: 3
source:
  data: LIST_QUEST.STB row 1981; QSD triggers 1981-01, 1981-02, 1981-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Zeppastone Wind Gem

Punwell says that the Zeppastone Wind Gem is necessary for the prosperity of the Windmill Village. All you know is that the Zeppastone should be in a dark, enclosed place.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1981-01`

Checks:

- your Level ≥ 41
- quest switch 128 is off

Then:

- you get the quest [[quests/1981-zeppastone-wind-gem|Zeppastone Wind Gem]]

### `1981-02`

Happens by talking to [[npcs/1062-smith-punwell|Smith Punwell]].

Checks:

- you have [[quests/1981-zeppastone-wind-gem|Zeppastone Wind Gem]]
- you carry = 1 × [[items/quest/509-zeppastone-wind-gem|Zeppastone Wind Gem]]

Then:

- works on [[quests/1981-zeppastone-wind-gem|Zeppastone Wind Gem]]
- 1 × [[items/quest/509-zeppastone-wind-gem|Zeppastone Wind Gem]] is taken
- experience, base 5000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/2-health-vial-m|Health Vial (M)]], base count 10 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/22-mana-vial-m|Mana Vial (M)]], base count 10 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/1982-the-prince-of-akram|The Prince of Akram]]
- quest switch 128 on

### `1981-31`

Happens by killing [[monsters/261-goblin-jar|Goblin Jar]].

Checks:

- you have [[quests/1981-zeppastone-wind-gem|Zeppastone Wind Gem]]
- a random roll 0–99 lands in 0–70
- you carry < 1 × [[items/quest/509-zeppastone-wind-gem|Zeppastone Wind Gem]]

Then:

- works on [[quests/1981-zeppastone-wind-gem|Zeppastone Wind Gem]]
- you get 1 × [[items/quest/509-zeppastone-wind-gem|Zeppastone Wind Gem]]

If the checks fail, step `140-31` is tried instead.
