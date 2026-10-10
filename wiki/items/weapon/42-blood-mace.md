---
kind: item
id: weapon/42
name: Blood Mace
status: in-game
icon: item/960
class: One-Handed Blunt Weapon
level: 96
attack: 142
attack_speed: 12
range: 2
damage: physical
two_handed: false
durability: 54
quality: 59
needs: Strength 181
sockets: true
price: 103600
weight: 60
crafted_with: '[[skills/2451-mace-craft|Mace Craft]] level 7'
recipe:
- material: any Metal
  quantity: 95
- material: '[[items/material/45-tender-leather|Tender Leather]]'
  quantity: 35
- material: '[[items/material/11-unprocessed-metal|Unprocessed Metal]]'
  quantity: 15
- material: '[[items/material/127-misian|Misian]]'
  quantity: 15
craft_difficulty: 53
dropped_by:
- monster: '[[monsters/351-yeti-rider|Yeti Rider]]'
  level: 100
  slots_of_30: 0.2
- monster: '[[monsters/383-behemoth-king|Behemoth King]]'
  level: 142
  slots_of_30: 1
- monster: '[[monsters/453-seal-stone|Seal Stone]]'
  level: 100
  slots_of_30: 1
source:
  data: LIST_WEAPON.STB row 42
  code: module/src/items.rs
---
# Blood Mace

A cursed mace that thirsts for blood.
