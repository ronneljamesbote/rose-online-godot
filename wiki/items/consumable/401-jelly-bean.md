---
kind: item
id: consumable/401
name: Jelly Bean
status: in-game
icon: item/1696
class: Magic Item
effect: teaches skill 2801, uses skill 2801, fuel +2801
needs: Level 3
price: 50
weight: 8
dropped_by:
- monster: '[[monsters/1-mini-jelly-bean|Mini-Jelly Bean]]'
  level: 2
  slots_of_30: 2
- monster: '[[monsters/2-jelly-bean|Jelly Bean]]'
  level: 4
  slots_of_30: 2
- monster: '[[monsters/5-red-jelly-bean|Red Jelly Bean]]'
  level: 6
  slots_of_30: 2
- monster: '[[monsters/8-butterfly|ButterFly]]'
  level: 1
  slots_of_30: 2
- monster: '[[monsters/9-butterfly|ButterFly]]'
  level: 1
  slots_of_30: 2
- monster: '[[monsters/10-butterfly|ButterFly]]'
  level: 2
  slots_of_30: 2
sold_by:
- '[[npcs/1016-livestock-farmer-lampa|Livestock Farmer Lampa]]'
quest_reward:
- '[[quests/2411-beasts-for-hire|Beasts for Hire]]'
source:
  data: godot/data/items/consumable/401-jelly-bean.tres
  code: module/src/items.rs
---
# Jelly Bean

A capsule containing a monster that will fight for you once summoned.
