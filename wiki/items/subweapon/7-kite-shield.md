---
kind: item
id: subweapon/7
name: Kite Shield
status: in-game
icon: item/1501
class: Shield
defence: 16
resistance: 5
durability: 30
quality: 24
needs: Strength 100
bonus: Attack Speed -5
sockets: true
price: 10800
weight: 12
crafted_with: '[[skills/2411-subitem-craft|SubItem Craft]] level 3'
recipe:
- material: any Metal
  quantity: 28
- material: '[[items/material/43-sleek-leather|Sleek Leather]]'
  quantity: 8
- material: '[[items/material/172-thick-insect-shell|Thick Insect Shell]]'
  quantity: 6
craft_difficulty: 27
dropped_by:
- monster: '[[monsters/121-krawfy|Krawfy]]'
  level: 64
  slots_of_30: 0.2
- monster: '[[monsters/123-krawfy-captain|Krawfy Captain]]'
  level: 68
  slots_of_30: 0.4
- monster: '[[monsters/158-doonga-captain|Doonga Captain]]'
  level: 72
  slots_of_30: 0.4
- monster: '[[monsters/276-goblin-guard|Goblin Guard]]'
  level: 70
  slots_of_30: 0.2
- monster: '[[monsters/278-goblin-warrior|Goblin Warrior]]'
  level: 73
  slots_of_30: 0.2
- monster: '[[monsters/284-goblin-leader|Goblin Leader]]'
  level: 77
  slots_of_30: 0.2
- monster: '[[monsters/331-slag|Slag]]'
  level: 78
  slots_of_30: 0.2
- monster: '[[monsters/332-elder-slag|Elder Slag]]'
  level: 82
  slots_of_30: 0.2
- monster: '[[monsters/343-yeti-guard|Yeti Guard]]'
  level: 101
  slots_of_30: 0.4
sold_by:
- '[[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]]'
source:
  data: LIST_SUBWPN.STB row 7
  code: module/src/items.rs
---
# Kite Shield

A long, leather shield that offers protection to the entire body.
