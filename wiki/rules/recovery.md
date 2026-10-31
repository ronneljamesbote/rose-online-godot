---
kind: rule
id: recovery
name: Recovery
status: in-game
passive_interval_s: 4
potion_tick_s: 1
natural_recovery:
  - { CON: 15, "HP standing": 1, "HP sitting": 12, "MP standing": 0, "MP sitting": 10 }
  - { CON: 50, "HP standing": 2, "HP sitting": 23, "MP standing": 0, "MP sitting": 20 }
  - { CON: 100, "HP standing": 3, "HP sitting": 37, "MP standing": 0, "MP sitting": 34 }
potions:
  - { item: "[[items/consumable/1-health-vial-s|Health Vial (S)]]", restores: "200 HP", per_second: 76, seconds: 3 }
  - { item: "[[items/consumable/2-health-vial-m|Health Vial (M)]]", restores: "400 HP", per_second: 76, seconds: 6 }
  - { item: "[[items/consumable/3-health-vial-l|Health Vial (L)]]", restores: "800 HP", per_second: 76, seconds: 11 }
  - { item: "[[items/consumable/4-health-bottle-s|Health Bottle (S)]]", restores: "300 HP", per_second: 140, seconds: 3 }
  - { item: "[[items/consumable/5-health-bottle-m|Health Bottle (M)]]", restores: "600 HP", per_second: 140, seconds: 5 }
  - { item: "[[items/consumable/6-health-bottle-l|Health Bottle (L)]]", restores: "1000 HP", per_second: 140, seconds: 8 }
  - { item: "[[items/consumable/7-herbal-medicine-s|Herbal Medicine (S)]]", restores: "500 HP", per_second: 30, seconds: 17 }
  - { item: "[[items/consumable/8-herbal-medicine-m|Herbal Medicine (M)]]", restores: "700 HP", per_second: 30, seconds: 24 }
  - { item: "[[items/consumable/9-herbal-medicine-l|Herbal Medicine (L)]]", restores: "1000 HP", per_second: 30, seconds: 34 }
  - { item: "[[items/consumable/10-vital-water-s|Vital Water (S)]]", restores: "250 HP", per_second: "all at once", seconds: 0 }
  - { item: "[[items/consumable/13-vital-water-xl|Vital Water (XL)]]", restores: "1000 HP", per_second: "all at once", seconds: 0 }
  - { item: "[[items/consumable/21-mana-vial-s|Mana Vial (S)]]", restores: "150 MP", per_second: 30, seconds: 5 }
  - { item: "[[items/consumable/22-mana-vial-m|Mana Vial (M)]]", restores: "300 MP", per_second: 30, seconds: 10 }
  - { item: "[[items/consumable/23-mana-vial-l|Mana Vial (L)]]", restores: "500 MP", per_second: 30, seconds: 17 }
  - { item: "[[items/consumable/24-mana-bottle-s|Mana Bottle (S)]]", restores: "200 MP", per_second: 70, seconds: 3 }
  - { item: "[[items/consumable/25-mana-bottle-m|Mana Bottle (M)]]", restores: "400 MP", per_second: 70, seconds: 6 }
  - { item: "[[items/consumable/26-mana-bottle-l|Mana Bottle (L)]]", restores: "700 MP", per_second: 70, seconds: 10 }
  - { item: "[[items/consumable/29-spiritual-water-s|Spiritual Water (S)]]", restores: "200 MP", per_second: "all at once", seconds: 0 }
  - { item: "[[items/consumable/101-apple|Apple]] and other food", restores: "100 HP (Apple)", per_second: 30, seconds: 4 }
  - { item: "[[items/consumable/120-milk|Milk]] and other drinks", restores: "60 MP (Milk)", per_second: 16, seconds: 4 }
cooldowns:
  - { group: HP food, items: "food that restores HP (Apple, Cooked Meat, ShavShav ...)", seconds: "its restoring time - 1" }
  - { group: MP food, items: "drinks and food that restore MP (Milk, juices, Sweet Honey, PiruKiru ...)", seconds: "its restoring time - 1" }
  - { group: HP potion, items: "Health Vial, Health Bottle, Herbal Medicine, Vital Water, Red Pill, Flanae Root, Tree Sap", seconds: 30 }
  - { group: MP potion, items: "Mana Vial, Mana Bottle, Pilules, Spiritual Water, Blue Pill, Royal Jelly", seconds: 15 }
source:
  code:
    - module/src/lib.rs (passive_recovery, RECOVERY_INTERVAL_US, sit, stand_up, spawn_tick, Sitting)
    - crates/rose-game-irose/src/data/ability_values.rs (calculate_passive_recover_hp, calculate_passive_recover_mp, calculate additional_health_recovery and additional_mana_recovery)
    - module/src/items.rs (use_item, apply_consumable, regen_tick, clear_regen, Regen, ItemCooldown, show_food_effect)
    - godot/rust/src/net.rs (set_item_cooldown, get_entities auras), godot/scripts/net_entity.gd (_show_auras)
    - module/src/ability.rs (add_value)
  data: the consumable resources in godot/data/items/consumable/ (amount status_effect_value, recovery effect status_effect_id; iROSE's LIST_USEITEM.STB columns 20 and 24), the status effect resources 1 to 6 in godot/data/status_effects/ (amount per second, apply_status_effect_1_value; iROSE's LIST_STATUS.STB rows 1-6)
---
# Recovery

HP comes back slowly on its own, and faster while you sit. **MP only comes back while you
sit.** Potions, food and drinks restore HP or MP over a few seconds, or all at once for
Vital Water and Spiritual Water. Nothing restores HP or MP while you are fallen (see
[[rules/death|Death]]), and natural recovery stops while you drive a cart (see
[[rules/carts|Carts]]); potions still work there.

## Natural recovery

Every **4 seconds** the server gives every living player some HP, and MP if they are
sitting. The 4 second clock is shared by everyone, so the first gift after you sit down can
come at any time within 4 seconds. Whether you count as sitting is checked at that moment.

Standing or moving:

```math
\text{HP} = \left\lfloor \frac{\text{extra HP} + \frac{\text{CON} + 40}{6}}{6} \right\rfloor \qquad \text{MP} = 0
```

Sitting:

```math
\text{HP} = \left\lfloor \text{extra HP} + \frac{\text{CON} + 30}{8} \times 2.3 \right\rfloor \qquad \text{MP} = \left\lfloor \text{extra MP} + \frac{\text{CON} + 20}{10} \times \frac{20}{7} \right\rfloor
```

"extra HP" and "extra MP" come from passive skills: a skill's fixed recovery bonus, plus
the recovery bonus on your equipment multiplied by the skill's recovery percent.

**Worked example:** a new character with 15 CON gets
(55 / 6) / 6 = 1.5, so **1 HP** every 4 seconds while standing. Sitting it gets
(45 / 8) × 2.3 = 12.9, so **12 HP**, and (35 / 10) × 20 / 7 = **10 MP** every 4 seconds.
The table `natural_recovery` above shows more CON values.

> Open question: recovery bonuses on equipment only count when a passive skill gives a
> recovery percent (they are multiplied by it), so without such a skill they do nothing.
> Check whether that is what iROSE does.

Monsters do not recover HP over time. A monster that gives up a chase heals fully (see
[[rules/monster-behaviour|Monster behaviour]]).

## Sitting

Press X (or use the Sit action) to sit down, and again to stand up. Sitting stops you
and cancels an attack or a skill being cast. You can't sit while driving, while fallen,
or while stunned or asleep.

You stand up again when you move, attack, use a skill or get on a cart. Being hit does
**not** stand you up. A personal shop is open only while you sit (see
[[rules/personal-shops|Personal shops]]).

## Potions, food and drinks

Using one takes it from your bag and starts restoring. Most restore a fixed amount each
second until the total is given; Vital Water and Spiritual Water give it all at once.
Restoring never goes above your max HP or MP, and what doesn't fit is lost. The table
`potions` above lists the common ones.

| Kind | Restores | Per second |
| --- | --- | --- |
| Health Vial | HP | 76 |
| Health Bottle | HP | 140 |
| Herbal Medicine, food | HP | 30 |
| Mana Vial | MP | 30 |
| Mana Bottle | MP | 70 |
| Drinks (milk, juice) | MP | 16 |
| Vital Water | HP | all at once |
| Spiritual Water | MP | all at once |

**Worked example:** a [[items/consumable/1-health-vial-s|Health Vial (S)]] restores 200 HP:
76 after 1 second, 76 after 2, the last 48 after 3.

## Cooldowns

Potions and food have a cooldown, shared by everything of the same group. There are four
groups, each with its own cooldown (the table `cooldowns` above):

| Group | Cooldown |
| --- | --- |
| HP food | the food's restoring time minus 1 second |
| MP food (drinks) | the food's restoring time minus 1 second |
| HP potion | 30 seconds |
| MP potion | 15 seconds |

A food's restoring time is its total divided by what it gives per second, rounded up, so
an [[items/consumable/101-apple|Apple]] (100 HP at 30 a second) restores for 4 seconds and
has a 3 second cooldown; [[items/consumable/136-shavshav|ShavShav]] (700 HP) restores for
24 and waits 23. Each item's page shows its group and seconds.

- Using any item of a group starts that group's cooldown for its own length; another
  item of the same group can't be used until it ends. The other three groups are not held
  up: you can drink an HP potion right after eating an Apple.
- The cooldown belongs to the character. Logging out, changing zone, falling, dropping
  and picking up items, trading, banking or buying more don't reset it; the time keeps
  running while you are offline.
- Using an item during its cooldown does nothing and tells you how many seconds are left.
  The item keeps its place in the bag.
- The hotbar and the inventory show the cooldown on every item of the group as a dark
  sweep with the seconds left.

## Food buffs

While food restores HP or MP it shows as a buff (HP Restoration or MP Restoration) in your
buff row, with the time left, and its effect plays on the character for everyone to see
(the healing glow for HP food, the blue one for MP food). Both go when the food has given
all it has, when another food of the same kind replaces it, or when you fall or log out.
Potions show no buff.

## How they stack

- One HP effect and one MP effect from potions, and one of each from food, can run at the
  same time: an HP potion and HP food both restore side by side.
- A second HP potion **replaces** the HP potion effect still running, and what the first
  one had not given yet is lost; a second HP food replaces the HP food. The same goes for
  MP. With the cooldowns, a potion can only replace one that has run for 30 (HP) or 15
  (MP) seconds, and food the last second of the food before it.
- Potions and natural recovery add up: both keep working at the same time, sitting or not.
- Falling ends all running potion and food effects, and logging out ends them too.

**Worked example:** you drink a Health Vial (L) (800 HP, 11 seconds) and eat an Apple 2
seconds later. The vial keeps giving 76 HP a second and the Apple adds 30 a second for 4
seconds. Another Health Vial waits until 30 seconds after the first; another Apple until 3
seconds after the Apple.

## Changed from iROSE

- iROSE has no waiting time between potions or food; here they have the four cooldown
  groups above (Ronnel's design).
- In iROSE (and rose-offline) food and a potion of the same kind replace each other; here
  they restore side by side, and food shows a buff and its effect while it lasts.
