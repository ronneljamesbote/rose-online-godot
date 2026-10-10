---
kind: rule
id: shops
name: Shops
status: changed-from-irose
store_range_m: 60
store_window_closes_beyond_m: 15
store_tabs_max: 4
max_buy_quantity: 999
world_price_rate: 100
item_price_rate: 50
town_price_rate: 100
item_rate_classes: "Medicine, Food, Metal, Otherworldly Metal, Stone, Wood, Leather, Cloth, Refining Material, Chemicals"
examples:
  - { item: "[[items/weapon/2-short-sword|Short Sword]]", base_price: 420, buy: 319, buy_with_discount_10: 287, sell_new: 141 }
  - { item: "[[items/consumable/1-health-vial-s|Health Vial (S)]]", base_price: 100, buy: 100, buy_with_discount_10: 90, sell_new: 55 }
  - { item: "[[items/jewellery/1-shining-ring|Shining Ring]]", base_price: 2050, buy: 3075, buy_with_discount_10: 2768, sell_new: 1708 }
source:
  code:
    - module/src/npcs.rs (npc_store_transaction, STORE_RANGE_CM)
    - crates/rose-game-irose/src/data/ability_values.rs (npc_store_buy_price, npc_store_sell_price)
    - module/src/lib.rs (WorldRates, world_rates_row, set_price_rates)
    - godot/scripts/store_window.gd
    - godot/scripts/inventory_window.gd (selling with right-click)
    - godot/rust/src/net.rs (get_store, price_rates)
  data: "LIST_NPC.STB (each NPC's store tabs and union), LIST_SELL.STB (the items on each tab), item STBs (base price column 5, price rate column 6, quality column 8, class column 4)"
---
# Shops

Many town NPCs run a shop. Pick the shop answer in their dialog (see
[[rules/npc-dialogs|NPC dialogs]]), or click an NPC that has nothing to say but has a
shop. The store window opens next to your inventory with up to four tabs of items. Each
NPC page lists its shop under `shop` with each item's base price.

- **Buy**: right-click an item to buy one, Shift + right-click to buy ten (stackable
  items only).
- **Sell**: while the store is open, right-click an item in your bag to sell one,
  Shift + right-click to sell the whole stack, or drag the item onto the store.
- The item's tooltip in the store shows the price, and bag items show what the store
  pays.
- Walking more than 15 m from the NPC, or pressing Esc, closes the store.

Every buy and sell is one deal: the store first takes what you sell, then adds what you
buy, then pays or charges the difference. If anything fails (your bag is full, you don't
have the Zuly) nothing happens at all. You must be alive, in the same zone and within
60 m of the NPC. You can buy up to 999 of a stackable item at once; equipment is one at
a time.

Shops that belong to a union (faction) refuse everyone for now ("only union members can
trade here"), because unions are not in the game yet.

## Price rates

Three rates set prices for the whole server. An admin can change them; these are the
defaults:

| Rate | Default | Used for |
| --- | --- | --- |
| world price rate | 100 | what stores pay for everything |
| item price rate | 50 | consumables and materials of the classes listed in `item_rate_classes` (potions, food, ores, wood, leather, cloth and so on) |
| town price rate | 100 | other consumables, materials, gems, jewellery and quest items |

At an item rate of 50 the item's own price rate does nothing. Other rates move the price
up or down by the item's price rate (from the item data) per mille.

The Dealer skill [[skills/2051-discount|Discount]] lowers what you pay and
[[skills/2071-overcharge|Overcharge]] raises what stores pay you, each by its percentage.
Below, **D** is your Discount percent and **O** your Overcharge percent.

## Buying

Equipment (weapons, armour, shields, backs, faces, cart parts):

```math
\text{price} = \left\lfloor \frac{\text{base price} \times (\text{quality} + 50) \times (1 - D/100)}{100} + 0.5 \right\rfloor
```

Example: [[items/weapon/2-short-sword|Short Sword]], base price 420, quality 26:
420 × 76 ÷ 100 = 319.2, plus 0.5 = 319.7, so **319** Zuly. With Discount 10%: 287.28 + 0.5,
so **287** Zuly.

Consumables, materials, gems, jewellery and quest items, with **rate** the item or town
price rate from the table above:

```math
\text{price} = \left\lfloor \text{base price} \times \left(1 + \frac{(\text{rate} - 50) \times \text{price rate}}{1000}\right) \times (1 - D/100) + 0.5 \right\rfloor
```

Examples: [[items/consumable/1-health-vial-s|Health Vial (S)]] (Medicine, base 100, uses
the item rate 50): 100 × 1 + 0.5 = **100** Zuly. [[items/jewellery/1-shining-ring|Shining Ring]]
(base 2,050, price rate 10, uses the town rate 100): 2,050 × (1 + 50 × 10 ÷ 1000) = 3,075,
plus 0.5, so **3,075** Zuly.

## Selling

Stores pay for an item by its condition. Equipment:

```math
\text{price} = \left\lfloor \frac{\text{base} \times (40 + \text{grade}) \times (200 + \text{durability}) \times (200 - \text{world rate}) \times (1 + O/100) \times \frac{4000 + \text{life}}{14000}}{1000000} + 0.2 \times \text{gem base price} \right\rfloor
```

- **grade** is the item's refine grade (see [[rules/refining|Refining]]).
- **durability** is the item's durability stat and **life** its wear, 1000 when new (see
  [[rules/durability|Durability]]).
- The gem part counts only for an item that has a gem set in it.

Example: a new [[items/weapon/2-short-sword|Short Sword]] (base 420, grade 0, durability 35,
life 1000): 420 × 40 × 235 × 100 × 1 × (5000 ÷ 14000) ÷ 1,000,000 = **141** Zuly.

Consumables, materials, gems, jewellery and quest items:

```math
\text{price} = \left\lfloor \frac{\text{base} \times \left(1000 + (\text{rate} - 50) \times \text{price rate}\right) \times (1 + O/100) \times (200 - \text{world rate})}{180000} \right\rfloor
```

Examples: [[items/consumable/1-health-vial-s|Health Vial (S)]]: 100 × 1000 × 100 ÷ 180,000 =
55.6, so **55** Zuly (61 with Overcharge 10%). [[items/jewellery/1-shining-ring|Shining Ring]]:
2,050 × 1,500 × 100 ÷ 180,000 = **1,708** Zuly.

Prices are worked out in floating point and then cut to a whole number.

## Changed from iROSE

- iROSE moved the town price rate with each town's economy over time; here the rates are
  fixed server-wide values (rose-offline's defaults) until an admin changes them.

> Open question: the server lets you trade with a store up to 60 m away, but the store window closes beyond 15 m. Should the server range be 15 m too?
