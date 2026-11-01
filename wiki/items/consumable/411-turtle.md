---
kind: item
id: consumable/411
name: Turtle
status: in-game
icon: item/1696
class: Magic Item
effect: teaches skill 2811, uses skill 2811, fuel +2811
needs: Level 27
price: 380
weight: 26
dropped_by:
- monster: '[[monsters/84-turtle|Turtle]]'
  level: 27
  slots_of_30: 2
- monster: '[[monsters/85-turtle-guard|Turtle Guard]]'
  level: 29
  slots_of_30: 2
sold_by:
- '[[npcs/1016-livestock-farmer-lampa|Livestock Farmer Lampa]]'
source:
  data: godot/data/items/consumable/411-turtle.tres
  code: module/src/items.rs
---
# Turtle

A capsule containing a monster that will fight for you once summoned.

Using it summons the monster and costs **2000** [[rules/stamina|Stamina]]. If you have less, the
capsule isn't used.
