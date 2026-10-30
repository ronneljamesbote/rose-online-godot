---
kind: quest
id: 2110
name: 'Words in Stone'
status: in-game
given_by:
- '[[npcs/1059-archaeologist-pell|Archaeologist Pell]]'
level: 16
steps: 3
source:
  data: 'godot/data/quests/2110-words-in-stone.tres; quest triggers in godot/data/quest_triggers/: ql21-001/2110-01, ql21-001/2110-02, ql21-001/2110-31'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Words in Stone

The Pomic Soldiers guarding the ruins carry pieces of the old inscriptions. Bring Pell 4 Inscribed Fragments.

Part of the [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]] story; see "Quests" on the zone page for the whole chain.

[[npcs/1059-archaeologist-pell|Archaeologist Pell]] offers it from level 16, after [[quests/2109-lost-diggers|Lost Diggers]].

- **Start:** Bring Pell 4 Inscribed Fragments from the Pomic Soldiers in the ruins.
- **End:** Pell read the fragments.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2110-01`

Happens by talking to [[npcs/1059-archaeologist-pell|Archaeologist Pell]].

Checks:

- your Level ≥ 16
- quest switch 608 = 1 (you finished [[quests/2109-lost-diggers|Lost Diggers]])
- quest switch 609 = 0 (you haven't done this quest yet)

Then:

- you get the quest [[quests/2110-words-in-stone|Words in Stone]]

### `2110-02`

Happens by talking to [[npcs/1059-archaeologist-pell|Archaeologist Pell]].

Checks:

- you have [[quests/2110-words-in-stone|Words in Stone]]
- you carry ≥ 4 × [[items/quest/706-inscribed-fragment|Inscribed Fragment]]

Then:

- works on [[quests/2110-words-in-stone|Words in Stone]]
- 4 × [[items/quest/706-inscribed-fragment|Inscribed Fragment]] is taken
- experience, base 3000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 2500 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set quest switch 609 to 1
- the quest ends (removed from your list)

### `2110-31`

Happens by killing [[monsters/74-pomic-soldier|Pomic Soldier]].

Checks:

- you have [[quests/2110-words-in-stone|Words in Stone]]
- a random roll 0–99 lands in 0–39
- you carry < 4 × [[items/quest/706-inscribed-fragment|Inscribed Fragment]]

Then:

- works on [[quests/2110-words-in-stone|Words in Stone]]
- you get 1 × [[items/quest/706-inscribed-fragment|Inscribed Fragment]]

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Valley of Luxem Tower redesign.
