---
kind: rule
id: crafting
name: Crafting
status: in-game
max_steps: 4
world_production_rate: 100
crafted_durability_range: "0-100"
socket_rare_type_always: 2
socket_rare_type_maybe: 1
socket_chance_rare_type_1: "(quality + 60) in 400"
craft_skills:
  - { skill: "[[skills/2431-sword-craft|Sword Craft]]", makes: "one-handed swords, two-handed swords, dual swords (make number 11)", mp: "50 + 5 per level" }
  - { skill: "[[skills/2451-mace-craft|Mace Craft]]", makes: "one-handed blunt weapons, spears, two-handed axes, katars (12)", mp: "50 + 5 per level" }
  - { skill: "[[skills/2471-bow-craft|Bow Craft]]", makes: "bows and crossbows (13)", mp: "50 + 5 per level" }
  - { skill: "[[skills/2511-gun-craft|Gun Craft]]", makes: "guns and launchers (14)", mp: "50 + 5 per level" }
  - { skill: "[[skills/2491-magic-weapon-craft|Magic Weapon Craft]]", makes: "staves, wands and some shields (15)", mp: "50 + 5 per level" }
  - { skill: "[[skills/2411-subitem-craft|Subitem Craft]]", makes: "shields and back items (16)", mp: "50 + 5 per level" }
  - { skill: "[[skills/2551-armor-craft|Armor Craft]]", makes: "Soldier and Hawker armour (17)", mp: "60 + 5 per level" }
  - { skill: "[[skills/2571-robe-craft|Robe Craft]]", makes: "Muse and Dealer armour (18)", mp: "60 + 5 per level" }
  - { skill: "[[skills/2531-clothing-craft|Clothing Craft]]", makes: "everyday clothes and face items (19)", mp: "60 + 5 per level" }
  - { skill: "[[skills/2591-cart-craft|Cart Craft]]", makes: "cart parts (21)", mp: "100 + 20 per level" }
  - { skill: "[[skills/2601-castlegear-craft|CastleGear Craft]]", makes: "castle gear parts (22)", mp: "100 + 20 per level" }
  - { skill: "[[skills/2611-gem-cutting|Gem Cutting]]", makes: "gems (35)", mp: "100 + 20 per level" }
  - { skill: "[[skills/2641-accessory-craft|Accessory Craft]]", makes: "nothing in the iROSE data (31)", mp: "50 + 5 per level" }
  - { skill: "[[skills/2401-item-craft|Item Craft]]", makes: "nothing in the iROSE data (41)", mp: "50 + 5 per level" }
step_formulas:
  - { step: 1, needed: "(D + 35) x (Q + 15) / 16", plus_factor: 30 }
  - { step: 2, needed: "(D + 15) x (Q + 140) / (N + 3) / 4", plus_factor: 20 }
  - { step: 3, needed: "(D + 90) x (Q + 30) / (N + 3) / 4", plus_factor: 10 }
  - { step: 4, needed: "(D + 40) x (Q + 60) / (N + 2) / 6", plus_factor: 10 }
source:
  code:
    - module/src/craft.rs (craft_item)
    - module/src/skills.rs (check_can_use, pay_costs, use_cost)
    - crates/rose-game-data/src/lib.rs (CraftRecipe, load_craft_recipes)
    - module/src/character.rs (reward_xp)
  data: LIST_PRODUCT.STB (recipes); item STBs columns 8 (quality), 12 (craft skill), 13 (craft skill level), 14 (recipe row), 15 (difficulty), 29 (durability), 30 (rare type); LIST_SKILL.STB column 9 (make number)
---
# Crafting

Dealers (and anyone else with a craft skill) can make weapons, armour, vehicle parts and
gems from materials. You pick the item and put one bag slot of material in for each step
of its recipe. Each material is a **step** with its own success roll. If a step fails,
crafting stops: the materials of that step and the steps before it are used up, the later
ones stay in your bag. You get experience either way.

A crafted item can come out better or worse than a dropped one: its durability depends on
how well the steps went, and it may get a bonus option or a gem socket.

## What you need

- A craft skill whose make number matches the item (see the table above). The item page
  shows it as "crafted with", with the skill level it needs. Your skill level must be at
  least that.
- The recipe's materials (LIST_PRODUCT.STB, the item page shows it as "recipe"). A step
  either names one exact item, or says "any Metal", "any Leather" and so on: then any item
  of that class works, and its quality matters.
- Enough MP for the skill (the skill's MP cost, lowered by Save Mana like any skill). Crafting
  starts the global skill cooldown. You can't craft while dead.
- Each material in its own bag slot. Stackable materials need the recipe's quantity in that
  stack; equipment used as a material counts as 1.

## The steps

Letters used below:

- **D**: the item's craft difficulty (item page "craft_difficulty").
- **Q**: the item's quality. **M**: the quality of the material in the **first** step (it is
  used for all steps).
- **N**: how many steps the recipe has (1 to 4). **S**: your craft skill level.
- **CON**, **SEN**: your Concentration and Sense. **L**: your level.
- **r**: a fresh roll from 0 to 99 for each step.

Each step works out your **progress** and passes when it is at least what the step
**needs** (table above). The world production rate is 100.

```math
\begin{aligned}
P_1 &= M \times (r + 71) \times (0.5\,\text{CON} + D/2 + 530) \times 100 / 800000 \\
P_2 &= (M + D/2) \times (r + 96) \times (0.5\,\text{CON} + 6S + 2M + 770) / (N + 7) / 1600 \\
P_3 &= (P_1/6 + Q) \times (r + 81) \times (0.3\,\text{CON} + 5S + 2M + 600) / (N + 7) / 2000 \\
P_4 &= (P_1 + P_2 + 40) \times (r + 51) / 200
\end{aligned}
```

Every step also adds to a **plus** score, which decides the item's durability and bonus
option. Each step adds its result rounded toward zero (so a weak pass can add 0 and a
close call can't go below 0, since the step had to pass):

```math
\text{plus} \mathrel{+}= \left\lfloor \frac{(P_k - \text{needed}_k) \times \text{factor}_k}{P_k} \right\rfloor
```

Step 1 is the exception: it divides by P1 + Q instead of P1. The factors are 30, 20, 10
and 10.

## The made item

**Durability** (0-100, rounded down), with a fresh roll r:

```math
\text{durability} = (\text{base durability} + 15) \times \frac{1.3\,\text{plus} + 2S + 120}{r + 81} \times 0.6
```

**Socket or bonus option.**

- Items of rare type 2 always come with an empty gem socket. Items of rare type 1 get one
  with a chance of (Q + 60) in 400. An item with a socket gets no bonus option.
- Otherwise the item may get a bonus option. With a roll R from 1 to 100:

```math
\text{option} = \left\lfloor \frac{(\text{SEN} + 220 - L/2) \times (\text{plus} + 20) \times 0.4 + 35D - 1600 - R}{R + 17} - 85 \right\rfloor
```

  If it is above 0, the item gets bonus option number option mod ((Q + 12) x 3.2, at most
  300). Bonus options are the rows below 300 of LIST_JEMITEM.STB; they add stats like a gem.

The item is marked as crafted. If your bag is full, it drops at your feet (only you can
pick it up at first).

**Experience** (rounded down):

```math
\begin{aligned}
\text{success: } & 1 + (L + 35) \times (P_1 + L) \times (N + 4) \times (D + 20) / 23000 \\
\text{failure: } & 1 + (L + 50) \times P_1 \times (N + 4) / 1300
\end{aligned}
```

## Worked example

A level 30 Dealer with Concentration 40, Sense 40 and
[[skills/2431-sword-craft|Sword Craft]] level 1 makes a
[[items/weapon/4-khukuri|Khukuri]] (difficulty 20, quality 32, base durability 40, rare
type 0). Its recipe has 3 steps: 13 of any Metal, 2
[[items/material/41-worn-out-leather|Worn-out Leather]], 1
[[items/material/176-insect-leg|Insect Leg]]. She uses
[[items/material/1-rusted-iron|Rusted Iron]] (quality 30) as the metal. Every roll is 50.

| Step | Progress | Needed | Plus |
| --- | --- | --- | --- |
| 1 | 30 x 121 x 560 x 100 / 800000 = 254.1 | 55 x 47 / 16 = 161.56 | (254.1 - 161.56) x 30 / 286.1 = 9.7, so 9 |
| 2 | 40 x 146 x 856 / 10 / 1600 = 312.44 | 35 x 172 / 6 / 4 = 250.83 | 9 + 3.9, so 12 |
| 3 | 74.35 x 131 x 677 / 10 / 2000 = 329.69 | 110 x 62 / 6 / 4 = 284.17 | 12 + 1.4, so 13 |

- Durability: 55 x (16.9 + 2 + 120) / 131 x 0.6 = 34.99, so **34**.
- Bonus option: (245 x 33 x 0.4 + 700 - 1600 - 51) / 68 - 85 is below 0, so none. Only rolls
  R of 1 to 10 give an option here (10%).
- Experience: 1 + 65 x 284.1 x 7 x 40 / 23000 = **225**. Had a step failed, 1 + 80 x 254.1 x
  7 / 1300 = 110.

Over all rolls, this crafter succeeds about 49% of the time with Rusted Iron, 92% with
[[items/material/5-iron|Iron]] (quality 46) and every time with
[[items/material/10-damascus|Damascus]] (quality 75): the first material's quality matters
most.

## Related

- Refining, disassembly and gems: [[rules/refining|Refining]].
- How durability wears down: [[rules/durability|Durability]].

> Open question: only the first material's quality counts for every step; the quality of the later materials is read but never used. iROSE may have used each step's own material.
> Open question: Item Craft (make number 41) shares its make number with Item Disassembly, so Item Craft can also disassemble items, and Item Disassembly could craft an item marked 41 (none exist in the iROSE data).
