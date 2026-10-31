---
kind: item
id: consumable/422
name: Doonga Warrior
status: in-game
icon: item/1697
class: Magic Item
effect: teaches skill 2822, uses skill 2822, fuel +2822
needs: Level 60
price: 1370
weight: 50
dropped_by:
- monster: '[[monsters/154-doonga-fighter|Doonga Fighter]]'
  level: 62
  slots_of_30: 1
- monster: '[[monsters/155-doonga-warrior|Doonga Warrior]]'
  level: 64
  slots_of_30: 1
source:
  data: godot/data/items/consumable/422-doonga-warrior.tres
  code: module/src/items.rs
---
# Doonga Warrior

A capsule containing a monster that will fight for you once summoned.

Using it summons the monster and costs **2000** [[rules/stamina|Stamina]]. If you have less, the
capsule isn't used.
