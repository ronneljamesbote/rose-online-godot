---
kind: item
id: consumable/416
name: Krawfy
status: in-game
icon: item/1697
class: Magic Item
effect: teaches skill 2816, uses skill 2816, fuel +2816
needs: Level 62
price: 1440
weight: 52
dropped_by:
- monster: '[[monsters/121-krawfy|Krawfy]]'
  level: 64
  slots_of_30: 1
- monster: '[[monsters/123-krawfy-captain|Krawfy Captain]]'
  level: 68
  slots_of_30: 1
sold_by:
- '[[npcs/1016-livestock-farmer-lampa|Livestock Farmer Lampa]]'
source:
  data: godot/data/items/consumable/416-krawfy.tres
  code: module/src/items.rs
---
# Krawfy

A capsule containing a monster that will fight for you once summoned.

Using it summons the monster and costs **2000** [[rules/stamina|Stamina]]. If you have less, the
capsule isn't used.
