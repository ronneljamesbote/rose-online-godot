---
kind: item
id: consumable/404
name: Pumpkin
status: in-game
icon: item/1696
class: Magic Item
effect: teaches skill 2804, uses skill 2804, fuel +2804
needs: Level 8
price: 90
weight: 12
dropped_by:
- monster: '[[monsters/21-pumpkin|Pumpkin]]'
  level: 9
  slots_of_30: 2
- monster: '[[monsters/23-elder-pumpkin|Elder Pumpkin]]'
  level: 13
  slots_of_30: 2
- monster: '[[monsters/24-cowboy-pumpkin|Cowboy Pumpkin]]'
  level: 14
  slots_of_30: 2
sold_by:
- '[[npcs/1016-livestock-farmer-lampa|Livestock Farmer Lampa]]'
source:
  data: godot/data/items/consumable/404-pumpkin.tres
  code: module/src/items.rs
---
# Pumpkin

A capsule containing a monster that will fight for you once summoned.

Using it summons the monster and costs **2000** [[rules/stamina|Stamina]]. If you have less, the
capsule isn't used.
