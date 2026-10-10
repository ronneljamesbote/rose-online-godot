---
kind: item
id: subweapon/9
name: Mythril Shield
status: in-game
icon: item/1503
class: Shield
defence: 21
resistance: 7
durability: 33
quality: 27
needs: Strength 128
bonus: Attack Speed -5
sockets: true
price: 19950
weight: 12
crafted_with: '[[skills/2411-subitem-craft|SubItem Craft]] level 4'
recipe:
- material: any Metal
  quantity: 36
- material: '[[items/material/44-tough-leather|Tough Leather]]'
  quantity: 10
- material: '[[items/material/172-thick-insect-shell|Thick Insect Shell]]'
  quantity: 8
craft_difficulty: 33
dropped_by:
- monster: '[[monsters/282-gem-goblin-mage|Gem Goblin Mage]]'
  level: 92
  slots_of_30: 0.2
- monster: '[[monsters/343-yeti-guard|Yeti Guard]]'
  level: 101
  slots_of_30: 0.4
source:
  data: LIST_SUBWPN.STB row 9
  code: module/src/items.rs
---
# Mythril Shield

A nigh impenetrable shield constructed from Mythril.
