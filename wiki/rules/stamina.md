---
kind: rule
id: stamina
name: Stamina
status: in-game
min_stamina: 0
max_stamina: 5000
starting_stamina: 0
world_stamina_rate: 100
kill_gain: "(XP + 100) / (level + 6) x 100 / 80, rounded down"
stamina_items:
  - { item: "[[items/consumable/56-vital-jam-1|Vital Jam (+1)]]", adds: 100 }
  - { item: "[[items/consumable/57-vital-jam-2|Vital Jam (+2)]]", adds: 200 }
  - { item: "[[items/consumable/58-vital-jam-5|Vital Jam (+5)]]", adds: 500 }
  - { item: "[[items/consumable/59-vital-jam-10|Vital Jam (+10)]]", adds: 1000 }
  - { item: "[[items/consumable/60-vital-jam-20|Vital Jam (+20)]]", adds: 2000 }
  - { item: "[[items/consumable/171-stamina-50|Stamina (+50)]] to [[items/consumable/180-stamina-1000|Stamina (+1000)]]", adds: "50 to 1000, as named" }
stamina_costs:
  - { item: "[[items/consumable/301-purify-scroll-solo|Purify Scroll]], [[items/consumable/302-detect-scroll-solo|Detect Scroll]]", stamina: 100 }
  - { item: "[[items/consumable/303-stealth-scroll-solo|Stealth Scroll]]", stamina: 200 }
  - { item: "HP, MP, Dexterity, Strength, Defense and Accuracy Scrolls (e.g. [[items/consumable/306-hp-scroll-solo|HP Scroll]])", stamina: 500 }
  - { item: "[[items/consumable/312-damage-scroll-solo|Damage Scroll]]", stamina: 700 }
  - { item: "Advanced HP, MP, Dexterity, Strength, Defense and Accuracy Scrolls (e.g. [[items/consumable/313-advanced-hp-scroll-solo|Advanced HP Scroll]])", stamina: 1000 }
  - { item: "[[items/consumable/319-advanced-damage-scroll-solo|Advanced Damage Scroll]]", stamina: 1300 }
  - { item: "[[items/consumable/321-blue-icicle|Blue Icicle]], [[items/consumable/322-green-icicle|Green Icicle]], [[items/consumable/323-white-icicle|White Icicle]]", stamina: 50 }
  - { item: "[[items/consumable/381-ice-charm|Ice Charm]], [[items/consumable/382-spark-charm|Spark Charm]], [[items/consumable/383-blood-charm|Blood Charm]]", stamina: 5000 }
  - { item: "Monster summon scrolls (e.g. [[items/consumable/401-jelly-bean|Jelly Bean]])", stamina: 100 }
source:
  code:
    - module/src/stamina.rs (add, reward, set_stamina)
    - crates/rose-game-irose/src/data/ability_values.rs (calculate_give_stamina)
    - module/src/party.rs (reward_kill_xp)
    - module/src/skills.rs (check_can_use, pay_costs)
    - module/src/ability.rs (get_value, add_value)
    - module/src/quests.rs
    - crates/rose-quest/src/lib.rs
  data: LIST_USEITEM.STB columns 19-20 (ability 76 = Stamina and amount); LIST_SKILL.STB columns 16-19 (Stamina cost)
---
# Stamina

Stamina is a pool from 0 to 5000 that some scrolls and charms spend instead of MP. Every
character starts with 0. The character window shows it as "Stamina (of 5000)".

## Getting stamina

**Killing monsters.** Every time you get experience from a kill, you also get stamina:

```math
\text{stamina} = \left\lfloor \frac{\text{XP} + 100}{\text{level} + 6} \times \frac{100}{80} \right\rfloor
```

*XP* is the experience you got from that kill (your share, in a party) and *level* is your
level before it. Low levels earn a lot per kill, high levels very little. Experience from
quests or crafting gives no stamina.

**Worked example.** A level 1 character gets 9 XP from a kill: (9 + 100) / 7 x 1.25 =
19.46, so **19** stamina. A level 90 character who gets 15 XP from a kill earns
115 / 96 x 1.25 = 1.5, so **1**.

**Items.** Vital Jam and the Stamina items add the amount in the table above. Note that a
Vital Jam's name counts in hundreds: Vital Jam (+1) adds 100.

**Quests** that reward stamina add it too.

Stamina never goes above 5000; anything over is lost.

## Spending stamina

Using one of the items in the cost table above runs a skill that costs stamina. If you have
less than it costs, the item isn't used and you see "Needs 500 Stamina (you have N)".

> Open question: quest conditions that check Stamina always read it as 0, and quest actions that set Stamina (rather than add to it) do nothing.
