---
kind: item
id: consumable/432
name: Lunar Wolf
status: in-game
icon: item/1697
class: Magic Item
effect: teaches skill 2832, uses skill 2832, fuel +2832
needs: Planet Requirements 2
price: 1990
weight: 29
dropped_by:
- monster: '[[monsters/336-wolf|Wolf]]'
  level: 85
  slots_of_30: 2
- monster: '[[monsters/339-lunar-wolf|Lunar Wolf]]'
  level: 90
  slots_of_30: 2
- monster: '[[monsters/389-wolf-keeper|Wolf Keeper]]'
  level: 112
  slots_of_30: 2
source:
  data: godot/data/items/consumable/432-lunar-wolf.tres
  code: module/src/items.rs
---
# Lunar Wolf

A capsule containing a monster that will fight for you once summoned.

Using it summons the monster and costs **2000** [[rules/stamina|Stamina]]. If you have less, the
capsule isn't used.
