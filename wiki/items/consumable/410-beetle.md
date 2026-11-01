---
kind: item
id: consumable/410
name: Beetle
status: in-game
icon: item/1696
class: Magic Item
effect: teaches skill 2810, uses skill 2810, fuel +2810
needs: Level 21
price: 270
weight: 22
dropped_by:
- monster: '[[monsters/81-beetle|Beetle]]'
  level: 20
  slots_of_30: 2
- monster: '[[monsters/82-beetle-fighter|Beetle Fighter]]'
  level: 22
  slots_of_30: 2
sold_by:
- '[[npcs/1016-livestock-farmer-lampa|Livestock Farmer Lampa]]'
source:
  data: godot/data/items/consumable/410-beetle.tres
  code: module/src/items.rs
---
# Beetle

A capsule containing a monster that will fight for you once summoned.

Using it summons the monster and costs **2000** [[rules/stamina|Stamina]]. If you have less, the
capsule isn't used.
