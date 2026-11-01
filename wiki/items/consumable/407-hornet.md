---
kind: item
id: consumable/407
name: Hornet
status: in-game
icon: item/1696
class: Magic Item
effect: teaches skill 2807, uses skill 2807, fuel +2807
needs: Level 17
price: 210
weight: 19
dropped_by:
- monster: '[[monsters/63-needle-hornet|Needle Hornet]]'
  level: 15
  slots_of_30: 2
- monster: '[[monsters/64-hornet|Hornet]]'
  level: 18
  slots_of_30: 2
sold_by:
- '[[npcs/1016-livestock-farmer-lampa|Livestock Farmer Lampa]]'
source:
  data: godot/data/items/consumable/407-hornet.tres
  code: module/src/items.rs
---
# Hornet

A capsule containing a monster that will fight for you once summoned.

Using it summons the monster and costs **2000** [[rules/stamina|Stamina]]. If you have less, the
capsule isn't used.
