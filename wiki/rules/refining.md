---
kind: rule
id: refining
name: Refining, Disassembly and Gems
status: in-game
max_grade: 9
max_durability_after_refining: 120
npc_range_m: 15
world_production_rate: 100
refine_success_if: "success value < 1000"
refine_mp: "(grade + 4) x (Q + 20) x 0.9"
refine_zuly: "grade x (grade + 1) x Q x (Q + 20) x 0.2"
disassemble_mp: "Q + 30"
disassemble_zuly: "Q x 10 + 20"
gem_removal_mp: "Q / 2 + gem quality"
gem_removal_zuly: "Q x 5 + 50"
first_gem_number: 301
refine_materials_weapon:
  - { grade: "0 to 1", chemicals: 5, refining_material: "7 [[items/material/61-low-essence|Low Essence]]", hearts: "" }
  - { grade: "1 to 2", chemicals: 7, refining_material: "9 [[items/material/62-essence|Essence]]", hearts: "" }
  - { grade: "2 to 3", chemicals: 8, refining_material: "12 [[items/material/63-low-ether|Low Ether]]", hearts: "1 [[items/material/151-black-hearts|Black Hearts]]" }
  - { grade: "3 to 4", chemicals: 9, refining_material: "15 [[items/material/64-ether|Ether]]", hearts: "1 [[items/material/152-green-hearts|Green Hearts]]" }
  - { grade: "4 to 5", chemicals: 10, refining_material: "18 [[items/material/65-high-ether|High Ether]]", hearts: "2 [[items/material/153-blue-hearts|Blue Hearts]]" }
  - { grade: "5 to 6", chemicals: 12, refining_material: "22 [[items/material/66-iricer|Iricer]]", hearts: "2 [[items/material/154-pink-hearts|Pink Hearts]]" }
  - { grade: "6 to 7", chemicals: 14, refining_material: "25 [[items/material/67-hime|Hime]]", hearts: "2 [[items/material/155-red-hearts|Red Hearts]]" }
  - { grade: "7 to 8", chemicals: 16, refining_material: "28 [[items/material/68-low-enthiric|Low Enthiric]]", hearts: "3 [[items/material/156-golden-hearts|Golden Hearts]]" }
  - { grade: "8 to 9", chemicals: 18, refining_material: "30 [[items/material/69-enthiric|Enthiric]]", hearts: "3 [[items/material/157-white-hearts|White Hearts]]" }
refine_materials_other:
  - { grade: "0 to 1", chemicals: 3, refining_material: "5 [[items/material/61-low-essence|Low Essence]]", hearts: "" }
  - { grade: "1 to 2", chemicals: 4, refining_material: "7 [[items/material/62-essence|Essence]]", hearts: "" }
  - { grade: "2 to 3", chemicals: 5, refining_material: "9 [[items/material/63-low-ether|Low Ether]]", hearts: "1 [[items/material/151-black-hearts|Black Hearts]]" }
  - { grade: "3 to 4", chemicals: 6, refining_material: "11 [[items/material/64-ether|Ether]]", hearts: "1 [[items/material/152-green-hearts|Green Hearts]]" }
  - { grade: "4 to 5", chemicals: 7, refining_material: "13 [[items/material/65-high-ether|High Ether]]", hearts: "1 [[items/material/153-blue-hearts|Blue Hearts]]" }
  - { grade: "5 to 6", chemicals: 8, refining_material: "15 [[items/material/66-iricer|Iricer]]", hearts: "2 [[items/material/154-pink-hearts|Pink Hearts]]" }
  - { grade: "6 to 7", chemicals: 9, refining_material: "18 [[items/material/67-hime|Hime]]", hearts: "2 [[items/material/155-red-hearts|Red Hearts]]" }
  - { grade: "7 to 8", chemicals: 10, refining_material: "22 [[items/material/68-low-enthiric|Low Enthiric]]", hearts: "2 [[items/material/156-golden-hearts|Golden Hearts]]" }
  - { grade: "8 to 9", chemicals: 12, refining_material: "25 [[items/material/69-enthiric|Enthiric]]", hearts: "3 [[items/material/157-white-hearts|White Hearts]]" }
refine_chance_khukuri:
  - { grade: "0 to 1", gray_powder_q6: "100%", rainbow_powder_q35: "100%", lisent_fe_q40: "100%", lisent_u_q80: "100%", npc_fee: "0 Zuly", mp: 187 }
  - { grade: "1 to 2", gray_powder_q6: "26%", rainbow_powder_q35: "100%", lisent_fe_q40: "100%", lisent_u_q80: "100%", npc_fee: "665 Zuly", mp: 234 }
  - { grade: "2 to 3", gray_powder_q6: "0%", rainbow_powder_q35: "100%", lisent_fe_q40: "100%", lisent_u_q80: "100%", npc_fee: "1996 Zuly", mp: 280 }
  - { grade: "3 to 4", gray_powder_q6: "0%", rainbow_powder_q35: "100%", lisent_fe_q40: "100%", lisent_u_q80: "100%", npc_fee: "3993 Zuly", mp: 327 }
  - { grade: "4 to 5", gray_powder_q6: "0%", rainbow_powder_q35: "77%", lisent_fe_q40: "97%", lisent_u_q80: "100%", npc_fee: "6656 Zuly", mp: 374 }
  - { grade: "5 to 6", gray_powder_q6: "0%", rainbow_powder_q35: "41%", lisent_fe_q40: "56%", lisent_u_q80: "100%", npc_fee: "9984 Zuly", mp: 421 }
  - { grade: "6 to 7", gray_powder_q6: "0%", rainbow_powder_q35: "18%", lisent_fe_q40: "29%", lisent_u_q80: "100%", npc_fee: "13977 Zuly", mp: 468 }
  - { grade: "7 to 8", gray_powder_q6: "0%", rainbow_powder_q35: "1%", lisent_fe_q40: "10%", lisent_u_q80: "81%", npc_fee: "18636 Zuly", mp: 514 }
  - { grade: "8 to 9", gray_powder_q6: "0%", rainbow_powder_q35: "0%", lisent_fe_q40: "0%", lisent_u_q80: "53%", npc_fee: "23961 Zuly", mp: 561 }
gem_removal_risk:
  - { gem_grade: "[1] (quality 30)", bad_roll: "25%", result: "gem breaks" }
  - { gem_grade: "[2] (quality 40)", bad_roll: "24%", result: "one grade lower" }
  - { gem_grade: "[3] (quality 50)", bad_roll: "22%", result: "one grade lower" }
  - { gem_grade: "[4] (quality 60)", bad_roll: "20%", result: "one grade lower" }
  - { gem_grade: "[5] (quality 70)", bad_roll: "19%", result: "one grade lower" }
  - { gem_grade: "[6] (quality 80)", bad_roll: "17%", result: "one grade lower" }
  - { gem_grade: "[7] (quality 90)", bad_roll: "15%", result: "one grade lower" }
refining_npcs:
  - { npc: "[[npcs/1064-weapon-craftsman-mairath|Mairath]]", zone: "[[zones/23-breezy-hills|Breezy Hills]]" }
  - { npc: "[[npcs/1008-weapon-seller-raffle|Raffle]]", zone: "[[zones/1-canyon-city-of-zant|Zant]]" }
  - { npc: "[[npcs/1093-weapon-merchant-crune|Crune]]", zone: "[[zones/2-city-of-junon-polis|Junon Polis]]" }
  - { npc: "[[npcs/1181-smith-pavrick|Pavrick]]", zone: "[[zones/51-magic-city-of-the-eucar|Eucar]]" }
  - { npc: "[[npcs/1223-smith-nel-eldora|Nel Eldora]]", zone: "[[zones/61-refuge-xita|Refuge Xita]]" }
  - { npc: "[[npcs/1161-clan-base-camp-manager-kushard|Kushard]]", zone: "[[zones/15-zone-15|Clan hall]]" }
disassembly_npcs:
  - { npc: "[[npcs/1012-ferrell-guild-staff-ulysses|Ulysses]]", zone: "[[zones/1-canyon-city-of-zant|Zant]]" }
  - { npc: "[[npcs/1098-ferrell-guild-staff-kiroth|Kiroth]]", zone: "[[zones/2-city-of-junon-polis|Junon Polis]]" }
  - { npc: "[[npcs/1184-ferrell-trader-sergei|Sergei]]", zone: "[[zones/51-magic-city-of-the-eucar|Eucar]]" }
  - { npc: "[[npcs/1223-smith-nel-eldora|Nel Eldora]]", zone: "[[zones/61-refuge-xita|Refuge Xita]]" }
  - { npc: "[[npcs/1161-clan-base-camp-manager-kushard|Kushard]]", zone: "[[zones/15-zone-15|Clan hall]]" }
source:
  code:
    - module/src/craft.rs (refine_item, refine_recipe_index, disassemble_item, insert_gem, pay_for_work)
    - godot/scripts/item_work_window.gd
  data: LIST_PRODUCT.STB rows 1-10 (weapon refining) and 11-20 (other equipment), and each item's recipe row; LIST_JEMITEM.STB (gems from row 301, quality column 8)
---
# Refining, Disassembly and Gems

Three services work on finished equipment. Each can be done by an NPC for Zuly, or by
yourself with a Dealer skill for MP:

| Service | Skill | NPCs |
| --- | --- | --- |
| Refining (raise the grade) | [[skills/2631-item-refining\|Item Refining]] | weapon sellers and smiths, for example [[npcs/1064-weapon-craftsman-mairath\|Mairath]] in Breezy Hills |
| Disassembly (take apart, or take a gem out) | [[skills/2621-item-disassembly\|Item Disassembly]] | Ferrell Guild staff, for example [[npcs/1012-ferrell-guild-staff-ulysses\|Ulysses]] in Zant |
| Setting a gem | none needed | none needed |

With an NPC, you must be in the same zone and within 15 metres. With a skill, the MP cost
is the one worked out below, not the skill's own listed cost. Refining and disassembly work
on items in your bag, not on worn ones. You can't use them while dead.

Below, **Q** is the quality of the item being worked on (item page "quality") and
**grade** is its current grade (0 for a new item).

## Refining

Refining raises an item's grade by one, up to grade 9. The recipe depends on the grade you
start from: weapons use LIST_PRODUCT rows 1-10, all other equipment rows 11-20 (tables
above). The first step takes any **Chemicals** (the powders and Lisents); the quality of
that chemical matters a lot. The other steps take the exact items listed.

**Cost**, rounded down. At an NPC refining from grade 0 is free.

```math
\begin{aligned}
\text{MP} &= (\text{grade} + 4) \times (Q + 20) \times 0.9 \\
\text{Zuly} &= \text{grade} \times (\text{grade} + 1) \times Q \times (Q + 20) \times 0.2
\end{aligned}
```

All materials and the cost are used up whether it works or not.

**Success.** With **M** the chemical's quality, **dur** the item's durability and a roll r
from 0 to 99, the game works out a value with whole-number division. Refining works when
the value is below 1000:

```math
\text{value} = \left\lfloor \frac{(g + 2)(g + 3)(5g + 3Q + 250)(r + 61) \times 320}{M \times (\text{dur} + 180) \times 110} \right\rfloor + 200
```

Higher grades, higher item quality and a low-quality chemical make it harder. Higher
durability makes it easier.

**Durability change.** Every refine also works out a change, with a new roll r2 from 1 to
100 (whole-number division, rounded toward zero):

```math
\text{change} = \frac{200 + 10(M + 5) + 3\,r_2 - 80(g + 6)}{40}
```

- **Success**: the grade goes up by 1. If the change is positive, durability goes up by it
  (at most 120). A negative change does nothing.
- **Failure**: durability changes by the change, up or down (kept between 0 and 120). The
  item also loses grades, with a roll r3 from 0 to 99:

```math
\text{grades lost} = \left\lfloor \frac{(g + 1)(g + 10)}{r_3 + 41} \right\rfloor
```

  So failing from grade 0, 1 or 2 never loses a grade; from grade 8 you lose 1 to 3.

**Worked example.** A [[items/weapon/4-khukuri|Khukuri]] (quality 32, durability 40) at
grade 0, refined with [[items/material/71-gray-powder|Gray Powder]] (quality 6) and a roll
of 50: 2 x 3 x 346 x 111 x 320 = 73739520, divided by 6 x 220 x 110 = 145200 gives 507, plus
200 is **707**, below 1000, so it works. It costs 187 MP with the skill, or nothing at an
NPC. The table above gives the chance for each grade and chemical for this Khukuri. Gray
Powder can't take it past grade 2; a Lisent (U) gets it to grade 9 about half the time.

With [[items/material/80-rainbow-powder|Rainbow Powder]] (quality 35) the durability change
from grade 0 is (200 + 400 + 3 r2 - 480) / 40: 3 to 10, so a successful refine adds that
much durability. From grade 3 the same powder gives -2 to 4.

## Disassembly

Disassembly breaks a bag item into some of the materials of its crafting recipe. Items
with no recipe can't be taken apart. The item is used up.

**Cost:** MP = Q + 30 with the skill, or Zuly = Q x 10 + 20 at an NPC.

For each step of the recipe, in order, you get:

- the exact material of that step, or for an "any Metal" (or any other class) step, that
  class's material of a grade close to the item's quality:
  grade = (Q - 20) / 12 rounded toward zero, kept between 1 and 10. For Metal, grade 1 is
  [[items/material/1-rusted-iron|Rusted Iron]] and grade 10 is
  [[items/material/10-damascus|Damascus]].
- a quantity, with a roll r from 0 to 39 for each step (rounded down):

```math
\text{quantity} = \frac{\text{recipe quantity} \times (111 + r) \times (\text{dur}/2 + \text{life}/10 + 100)}{60000}
```

  where life is 0-1000. For a stackable item (a gem) dur counts as 0 and life/10 as 100.
  A quantity of 0 gives nothing for that step.

**Worked example.** A [[items/weapon/4-khukuri|Khukuri]] (quality 32, durability 40, full
life) costs 62 MP or 340 Zuly. The metal step gives grade (32 - 20) / 12 = 1, Rusted Iron:
13 x (111 to 150) x 220 / 60000 = **5 to 7**. The 2
[[items/material/41-worn-out-leather|Worn-out Leather]] give 1 when r is 26 or more (35%),
otherwise 0. The 1 [[items/material/176-insect-leg|Insect Leg]] never comes back
(at most 1 x 150 x 220 / 60000 = 0.55).

If your bag is full, the materials drop at your feet.

## Gems and sockets

Some items have an empty gem **socket**: crafted items of rare type 2 always, of rare type 1
sometimes (see [[rules/crafting|Crafting]]), and dropped items that came with one.

**Setting a gem** is free and needs no skill or NPC. Use a gem (number 301 or higher, see
[[items/gem|Gems]]) on an item you are **wearing** that has an empty socket. The gem is
used up and its stats count while you wear the item. A socket holds one gem.

**Taking a gem out** is done with disassembly on an item with a set gem. The item itself
stays; only the gem comes out.

- Cost: MP = Q / 2 + the gem's quality (rounded down), or Zuly = Q x 5 + 50.
- A roll r from 0 to 99 goes wrong when r + 1 + (gem quality / 6, rounded down) is 30 or
  less. Then a gem of quality 35 or less ([1] gems) breaks, and a better gem comes out one
  grade lower. Otherwise the gem comes out whole. The table above gives the risk.

**Worked example.** A [[items/weapon/10-shark-blade|Shark Blade]] (quality 53) with a
[[items/gem/302-garnet-2|Garnet 2]] (quality 40) costs 53 / 2 + 40 = 66 MP or 315 Zuly.
The roll goes wrong when r + 1 + 6 is 30 or less: r from 0 to 23, 24%. Then you get a
[[items/gem/301-garnet-1|Garnet 1]] back; otherwise the Garnet 2.

> Open question: the server accepts refining, disassembly and repair at any NPC within 15 metres; only the client limits these services to NPCs whose dialog offers them.
> Open question: LIST_PRODUCT rows 10 and 20 (grade 9 to 10, with Hermes) exist, but refining stops at grade 9.
