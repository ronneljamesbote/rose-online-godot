---
kind: rule
id: drops
name: Drops
status: in-game
drops_per_kill: at most 1
world_drop_rate: 300%
world_money_rate: 300%
no_drop_level_difference: more than 10 levels above the monster
drop_table_slots: 30
group_items: 5 per group (each 1/5 of a slot)
owner_time_s: 60
ground_time_s: 120
drop_scatter_m: 2
pickup_range_m: 4
party_share_range_m: 50
max_stack_from_drop: 10
source:
  code:
    - crates/rose-game-irose/src/data/drop_table.rs (get_drop, get_drop_table)
    - module/src/items.rs (monster_drop, drop_on_ground, expire_drops, pickup_item)
    - module/src/lib.rs (kill, world_rates_row, set_world_rates)
    - module/src/party.rs (pickup_receiver, split_money, nearby)
    - module/src/skills.rs (pickup_nearest)
    - crates/rose-game-irose/src/data/ability_values.rs (calculate_drop_rate)
    - tools/wiki-gen/src/main.rs (drop_row, the slots shown on monster pages)
  data: ITEM_DROP.STB, LIST_NPC.STB (drop item rate, drop money rate, drop table row, level)
---
# Drops

When you kill a monster it may drop Zuly or one item on the ground. Each kill rolls once: it
gives money, one item or nothing. The item comes either from the **monster's own drop
table** or from the **zone's drop table**, which is shared by every monster in that zone.
Quest items that drop for a quest come from the quest instead (see
[[rules/quests|Quests]]).

## Who the drop is for

- The drop is rolled for the player who landed the killing blow. If their summon killed
  it, the drop is for the summon's owner. That player's level, Drop Rate stat and Charm
  are used.
- A player's summon that dies drops nothing.

## Will it drop, and what?

The monster's data (on each monster page) gives its `drop_item_rate`, its
`drop_money_rate` and its level. The server rates are 300% for items and for money. The
level difference is your level minus the monster's level; a monster above your level counts
as 0.

1. **Too strong.** If you are **more than 10 levels above** the monster, it drops nothing.
2. **The drop roll.** With a random number from 1 to 100:

```math
\text{drop} = \left\lfloor \left( \text{world rate} + \text{monster item rate} - \text{roll} - 3.5 \times (\text{level difference} + 16) - 10 + \text{your Drop Rate} \right) \times 0.38 \right\rfloor
```

   If this is 0 or less, nothing drops.
3. **Money?** With a chance of the monster's `drop_money_rate` in percent, it drops Zuly:

```math
\text{Zuly} = \left\lfloor \frac{(\text{monster level} + 20) \times (\text{monster level} + \text{drop} + 40) \times \text{world money rate}}{3200} \right\rfloor
```

4. **Which table?** Otherwise it drops an item. With a chance of the monster's
   `drop_item_rate` in percent it uses the monster's own table (the row named on its
   page); otherwise the zone's table (the row with the zone's number).
5. **Which slot?** A drop table row has **30 slots**. One is picked at random from the
   first $\min(\text{drop}, 30)$ slots. At the normal 300% rate "drop" is always 30 or
   more, so all 30 slots are equally likely.
6. **Groups.** A slot holds either one item, nothing, or a **group** number from 1 to 4.
   A group is a list of 5 more items on the same row; one of the 5 is picked at random.
7. An empty slot (or an empty place in a group) drops nothing.

### What "slots of 30" means on monster pages

The `drops` table of each monster page lists its own drop table, with `slots_of_30`: how
many of the 30 slots give that item. An item that is directly in a slot counts 1; an item
in a group counts 1/5 for every slot pointing at that group. So with 5 slots of 30, an
item is picked in $5/30$ of the drops that use the monster's table. Items from the zone's
table are not listed on the monster page.

### Worked example

A level 5 player with no Drop Rate bonus kills a
[[monsters/1-mini-jelly-bean|Mini-Jelly Bean]] (level 2, drop item rate 70, drop money rate
6) on [[zones/20-birth-island|Birth Island]]. The level difference is 3.

- Drop roll, with a roll of 50:
  $\lfloor (300 + 70 - 50 - 3.5 \times 19 - 10) \times 0.38 \rfloor = \lfloor 92.53 \rfloor = 92$.
  With any roll it is between 73 and 111, so the kill always drops something unless it
  lands on an empty slot.
- Money (6%): $\lfloor 22 \times (2 + 92 + 40) \times 300 / 3200 \rfloor = \lfloor 276.4 \rfloor = 276$ Zuly.
- Otherwise (94%) an item: 70% from the Mini-Jelly Bean's own table, 30% from Birth
  Island's table. [[items/material/171-thin-insect-shell|Thin Insect Shell]] has 5 slots of
  30, so its chance per kill is $0.94 \times 0.70 \times 5/30 = 11.0\%$ (plus any chance
  from the zone's table). The [[items/weapon/2-short-sword|Short Sword]] (1.2 slots) has
  $0.94 \times 0.70 \times 1.2/30 = 2.6\%$.
- At level 13 the same player is 11 levels above the Mini-Jelly Bean and gets no drops
  from it.

## What the dropped item is like

- **Gems, materials and quest items** drop as a stack of

```math
1 + \left\lfloor \frac{\lfloor (\text{monster level} + 10)/9 \rfloor + \text{1..20} + \text{your Drop Rate}}{\text{drop} + 4} \right\rfloor
```

  at most **10**. At normal rates this is almost always 1.
- **Consumables** drop one at a time.
- **Equipment** gets random properties, with a random number from the range shown:
  - durability: $\lfloor \text{item durability} \times (0.3\,\text{monster level} + 2 \times \text{item rate} + 320) \times 0.5 / \text{201..300} \rfloor$, from 0 to 100,
  - lifespan: $\lfloor (\text{item rate} + 200) \times 80 / \text{31..130} \rfloor$, from 0 to 1000 (100.0%),
  - grade, for weapons, shields and armour:
    $\lfloor ((\text{item rate} - 5) \times 3 + 150 - \lfloor 1.5 \times \text{monster level} \rfloor - g + \text{Charm}) \times 0.4 / (g + 30) \rfloor - 1$
    with $g$ from 1 to 100, kept between 0 and 3,
  - sockets and options by the item's rare type: rare type 3 always comes with a gem
    option (100 to 140); rare type 2 always has a socket; rare type 1 has a socket with a
    chance of $(\text{quality} + 60) / 400$; other non-jewellery items may get a random
    option, more likely from high-level monsters, with a high item rate and with more Charm.
  "Item rate" here is the monster's `drop_item_rate`. See [[rules/durability|Durability]]
  and [[rules/refining|Refining]].

## On the ground

- The drop lands up to **2 m** from where the monster died (in each direction).
- For **60 seconds** only the killer and their party members can pick it up. After that
  anyone can.
- It disappears after **120 seconds** if nobody picks it up.
- You pick an item up from up to **4 m** away, by clicking it or with the
  [[skills/12-pick-up|Pick Up]] action (which takes the nearest one you may pick up). You
  can't pick up while dead, stunned or asleep, or while riding as a passenger.
- Some consumables are used as soon as they are picked up instead of going into your bag.
- If your bag is full (or you can't carry more Zuly) the item stays on the ground.

## In a party

The party leader chooses how drops are shared ([[rules/party|Party]]). "Nearby" means party
members who are online, in the same zone and within **50 m** of the drop.

- **Picker, Zuly split** (the default): items go to whoever picks them up. Zuly is split
  evenly between the nearby members; the picker also gets what is left over from the
  division.
- **In turn**: each item, and each pile of Zuly, goes to the next nearby member in turn
  (items and Zuly keep separate turns). If that member's bag is full, the picker keeps it.

With only the picker nearby, everything goes to the picker.
