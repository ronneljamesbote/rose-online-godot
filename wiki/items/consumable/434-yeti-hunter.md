---
kind: item
id: consumable/434
name: Yeti Hunter
status: in-game
icon: item/1697
class: Magic Item
effect: teaches skill 2834, uses skill 2834, fuel +2834
needs: Planet Requirements 2
price: 2860
weight: 32
dropped_by:
- monster: '[[monsters/342-yeti-hunter|Yeti Hunter]]'
  level: 95
  slots_of_30: 2
source:
  data: godot/data/items/consumable/434-yeti-hunter.tres
  code: module/src/items.rs
---
# Yeti Hunter

A capsule containing a monster that will fight for you once summoned.

Using it summons the monster and costs **2000** [[rules/stamina|Stamina]]. If you have less, the
capsule isn't used.
