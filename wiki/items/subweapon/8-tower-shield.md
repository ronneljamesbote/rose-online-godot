---
kind: item
id: subweapon/8
name: Tower Shield
status: in-game
icon: item/1502
class: Shield
defence: 18
resistance: 6
durability: 32
quality: 26
needs: Strength 114
bonus: Attack Speed -5
sockets: true
price: 14500
weight: 12
crafted_with: '[[skills/2411-subitem-craft|SubItem Craft]] level 4'
recipe:
- material: any Metal
  quantity: 30
- material: '[[items/material/44-tough-leather|Tough Leather]]'
  quantity: 9
- material: '[[items/material/172-thick-insect-shell|Thick Insect Shell]]'
  quantity: 7
craft_difficulty: 30
dropped_by:
- monster: '[[monsters/279-gem-goblin-warrior|Gem Goblin Warrior]]'
  level: 89
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
source:
  data: LIST_SUBWPN.STB row 8
  code: module/src/items.rs
---
# Tower Shield

A shield that looks incredibly durable at first glance.
