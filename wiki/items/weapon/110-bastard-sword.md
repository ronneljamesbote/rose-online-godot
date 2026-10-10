---
kind: item
id: weapon/110
name: Bastard Sword
status: in-game
icon: item/1049
class: Two-Handed Sword
level: 88
attack: 140
attack_speed: 11
range: 2.5
damage: physical
two_handed: true
durability: 53
quality: 65
needs: Strength 162
sockets: true
price: 85700
weight: 40
crafted_with: '[[skills/2431-sword-craft|Sword Craft]] level 6'
recipe:
- material: any Metal
  quantity: 90
- material: '[[items/material/54-high-durability-cloth|High Durability Cloth]]'
  quantity: 15
- material: '[[items/material/99-broken-metal-fragment|Broken Metal Fragment]]'
  quantity: 10
- material: '[[items/material/126-lesian|Lesian]]'
  quantity: 12
craft_difficulty: 45
dropped_by:
- monster: '[[monsters/282-gem-goblin-mage|Gem Goblin Mage]]'
  level: 92
  slots_of_30: 0.2
- monster: '[[monsters/343-yeti-guard|Yeti Guard]]'
  level: 101
  slots_of_30: 0.2
- monster: '[[monsters/452-seal-stone|Seal Stone]]'
  level: 88
  slots_of_30: 1
source:
  data: LIST_WEAPON.STB row 110
  code: module/src/items.rs
---
# Bastard Sword

A sword that seems to be intended for two-handed use, but beginners might confuse it for one-handed use.
