---
kind: item
id: consumable/418
name: Grunter
status: in-game
icon: item/1697
class: Magic Item
effect: teaches skill 2818, uses skill 2818, fuel +2818
needs: Level 40
price: 700
weight: 36
dropped_by:
- monster: '[[monsters/134-grunter|Grunter]]'
  level: 42
  slots_of_30: 1
- monster: '[[monsters/135-grunter-fighter|Grunter Fighter]]'
  level: 44
  slots_of_30: 1
sold_by:
- '[[npcs/1016-livestock-farmer-lampa|Livestock Farmer Lampa]]'
source:
  data: godot/data/items/consumable/418-grunter.tres
  code: module/src/items.rs
---
# Grunter

A capsule containing a monster that will fight for you once summoned.

Using it summons the monster and costs **2000** [[rules/stamina|Stamina]]. If you have less, the
capsule isn't used.
