---
kind: item
id: consumable/409
name: Pomic Soldier
status: in-game
icon: item/1696
class: Magic Item
effect: teaches skill 2809, uses skill 2809, fuel +2809
needs: Level 15
price: 180
weight: 18
dropped_by:
- monster: '[[monsters/74-pomic-soldier|Pomic Soldier]]'
  level: 17
  slots_of_30: 2
- monster: '[[monsters/75-pomic-fighter|Pomic Fighter]]'
  level: 19
  slots_of_30: 2
sold_by:
- '[[npcs/1016-livestock-farmer-lampa|Livestock Farmer Lampa]]'
source:
  data: godot/data/items/consumable/409-pomic-soldier.tres
  code: module/src/items.rs
---
# Pomic Soldier

A capsule containing a monster that will fight for you once summoned.

Using it summons the monster and costs **2000** [[rules/stamina|Stamina]]. If you have less, the
capsule isn't used.
