---
kind: backlog
id: bag-weight
name: Bag weight
status: not-in-game-yet
summary: Everything you carry has a weight; carrying more than your max weight slows you down and stops pickups
source:
  data: item STBs column 7 (weight of one item)
  code:
    - crates/rose-game-irose/src/data/ability_values.rs (calculate_max_weight)
  reference: iROSE 129_129en client (weight shown in the inventory window)
---
# Bag weight

## How it works in iROSE 129

Every item has a weight (column 7 of its item STB); a stack weighs the item's weight times
the quantity. The inventory window shows how much you carry as a share of your max weight.
Your max weight comes from level, Strength, equipment and bag passives, as described in
[[rules/stats|Stats]]:

```math
\text{max weight} = 1100 + \text{level} \times 5 + \text{STR} \times 6 + \text{bonus}
```

When you carry too much:

- you can't pick up items or buy items that would take you over the limit;
- you move more slowly once you are over your max weight.

> Open question: the exact thresholds and penalties in iROSE 129 (for example walking
> only above 100% and not being able to attack or move above some higher share) are not
> checked against the original server. Settle them before building this.

## What our game does today

The server works out max weight, but the game doesn't show it and nothing checks it. You
can carry, pick up and buy anything as long as your bag has a free slot, and weight never
slows you down.

## Building it

- **Server**: add up the carried weight (bag and equipped items) and check it on pickup,
  purchase, trade, crafting rewards and quest rewards; apply the movement penalty in the
  move checks.
- **Client**: show the current weight against the max in the inventory window and an
  "overweight" message.
- **Data**: no change; item weights are already in the iROSE item STBs.
