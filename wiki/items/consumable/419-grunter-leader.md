---
kind: item
id: consumable/419
name: Grunter Leader
status: in-game
icon: item/1697
class: Magic Item
effect: teaches skill 2819, uses skill 2819, fuel +2819
needs: Level 50
price: 1020
weight: 43
dropped_by:
- monster: '[[monsters/138-grunter-leader|Grunter Leader]]'
  level: 53
  slots_of_30: 1
- monster: '[[monsters/139-grunter-captain|Grunter Captain]]'
  level: 55
  slots_of_30: 1
sold_by:
- '[[npcs/1016-livestock-farmer-lampa|Livestock Farmer Lampa]]'
source:
  data: godot/data/items/consumable/419-grunter-leader.tres
  code: module/src/items.rs
---
# Grunter Leader

A capsule containing a monster that will fight for you once summoned.

Using it summons the monster and costs **2000** [[rules/stamina|Stamina]]. If you have less, the
capsule isn't used.
