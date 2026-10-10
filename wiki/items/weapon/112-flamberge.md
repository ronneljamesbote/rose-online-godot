---
kind: item
id: weapon/112
name: Flamberge
status: in-game
icon: item/1051
class: Two-Handed Sword
level: 102
attack: 169
attack_speed: 11
range: 2.5
damage: physical
two_handed: true
durability: 55
quality: 71
needs: Strength 185
sockets: true
price: 137650
weight: 50
crafted_with: '[[skills/2431-sword-craft|Sword Craft]] level 7'
recipe:
- material: any Metal
  quantity: 100
- material: '[[items/material/55-silk|Silk]]'
  quantity: 25
- material: '[[items/material/11-unprocessed-metal|Unprocessed Metal]]'
  quantity: 15
- material: '[[items/material/127-misian|Misian]]'
  quantity: 16
craft_difficulty: 53
dropped_by:
- monster: '[[monsters/348-ruper-captain|Ruper Captain]]'
  level: 106
  slots_of_30: 0.4
- monster: '[[monsters/353-rider-captain|Rider Captain]]'
  level: 108
  slots_of_30: 0.2
- monster: '[[monsters/357-tyrant|Tyrant]]'
  level: 112
  slots_of_30: 0.4
- monster: '[[monsters/361-frostworm|FrostWorm]]'
  level: 109
  slots_of_30: 0.4
- monster: '[[monsters/362-elder-frostworm|Elder FrostWorm]]'
  level: 115
  slots_of_30: 0.4
- monster: '[[monsters/368-winter-maul|Winter Maul]]'
  level: 114
  slots_of_30: 0.2
- monster: '[[monsters/454-seal-stone|Seal Stone]]'
  level: 108
  slots_of_30: 1
source:
  data: LIST_WEAPON.STB row 112
  code: module/src/items.rs
---
# Flamberge

A sword with a wavy edge.
