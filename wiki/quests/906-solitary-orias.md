---
kind: quest
id: 906
name: Solitary Orias
status: in-game
npcs:
- '[[npcs/1073-ikaness-staff-orias|Ikaness Staff Orias]]'
steps: 8
source:
  data: LIST_QUEST.STB row 906; QSD triggers 905-02, 906-01, 906-31, 906-32, 906-33, 906-34, 906-35, 906-36
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Solitary Orias

Orias said that he needs Stones of Nature to fight this strange force. Bring Orias 5 Stones of Nature from each of the following elements: Fire, Water and Earth. Fight Dalpings and Ranger Dalpings to find these Stones of Nature.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `905-02`

Happens by talking to [[npcs/1073-ikaness-staff-orias|Ikaness Staff Orias]].

Checks:

- you have [[quests/905-solitary-orias|Solitary Orias]]

Then:

- works on [[quests/905-solitary-orias|Solitary Orias]]
- the quest becomes [[quests/906-solitary-orias|Solitary Orias]]

### `906-01`

Happens by talking to [[npcs/1073-ikaness-staff-orias|Ikaness Staff Orias]].

Checks:

- you have [[quests/906-solitary-orias|Solitary Orias]]
- you carry ≥ 5 × [[items/quest/119-stone-of-nature-fire|Stone of Nature (Fire)]]
- you carry ≥ 5 × [[items/quest/120-stone-of-nature-water|Stone of Nature (Water)]]
- you carry ≥ 5 × [[items/quest/121-stone-of-nature-earth|Stone of Nature (Earth)]]

Then:

- works on [[quests/906-solitary-orias|Solitary Orias]]
- 5 × [[items/quest/119-stone-of-nature-fire|Stone of Nature (Fire)]] is taken
- 5 × [[items/quest/120-stone-of-nature-water|Stone of Nature (Water)]] is taken
- 5 × [[items/quest/121-stone-of-nature-earth|Stone of Nature (Earth)]] is taken
- the quest becomes [[quests/907-solitary-orias|Solitary Orias]]

### `906-31`

Checks:

- you have [[quests/906-solitary-orias|Solitary Orias]]
- a random roll 0–99 lands in 0–30
- you carry < 5 × [[items/quest/119-stone-of-nature-fire|Stone of Nature (Fire)]]

Then:

- works on [[quests/906-solitary-orias|Solitary Orias]]
- you get 1 × [[items/quest/119-stone-of-nature-fire|Stone of Nature (Fire)]]

If the checks fail, step `906-32` is tried instead.

### `906-32`

Checks:

- you have [[quests/906-solitary-orias|Solitary Orias]]
- a random roll 0–99 lands in 0–30
- you carry < 5 × [[items/quest/120-stone-of-nature-water|Stone of Nature (Water)]]

Then:

- works on [[quests/906-solitary-orias|Solitary Orias]]
- you get 1 × [[items/quest/120-stone-of-nature-water|Stone of Nature (Water)]]

If the checks fail, step `906-33` is tried instead.

### `906-33`

Checks:

- you have [[quests/906-solitary-orias|Solitary Orias]]
- a random roll 0–99 lands in 0–30
- you carry < 5 × [[items/quest/121-stone-of-nature-earth|Stone of Nature (Earth)]]

Then:

- works on [[quests/906-solitary-orias|Solitary Orias]]
- you get 1 × [[items/quest/121-stone-of-nature-earth|Stone of Nature (Earth)]]

If the checks fail, step `856-31` is tried instead.

### `906-34`

Checks:

- you have [[quests/906-solitary-orias|Solitary Orias]]
- a random roll 0–99 lands in 0–30
- you carry < 5 × [[items/quest/121-stone-of-nature-earth|Stone of Nature (Earth)]]

Then:

- works on [[quests/906-solitary-orias|Solitary Orias]]
- you get 1 × [[items/quest/121-stone-of-nature-earth|Stone of Nature (Earth)]]

If the checks fail, step `906-35` is tried instead.

### `906-35`

Checks:

- you have [[quests/906-solitary-orias|Solitary Orias]]
- a random roll 0–99 lands in 0–30
- you carry < 5 × [[items/quest/120-stone-of-nature-water|Stone of Nature (Water)]]

Then:

- works on [[quests/906-solitary-orias|Solitary Orias]]
- you get 1 × [[items/quest/120-stone-of-nature-water|Stone of Nature (Water)]]

If the checks fail, step `906-36` is tried instead.

### `906-36`

Checks:

- you have [[quests/906-solitary-orias|Solitary Orias]]
- a random roll 0–99 lands in 0–30
- you carry < 5 × [[items/quest/119-stone-of-nature-fire|Stone of Nature (Fire)]]

Then:

- works on [[quests/906-solitary-orias|Solitary Orias]]
- you get 1 × [[items/quest/119-stone-of-nature-fire|Stone of Nature (Fire)]]

If the checks fail, step `856-32` is tried instead.
