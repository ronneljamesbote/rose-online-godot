---
kind: item
id: weapon/13
name: Katana
status: in-game
icon: item/923
class: One-Handed Sword
level: 95
attack: 120
attack_speed: 10
range: 2
damage: physical
two_handed: false
durability: 53
quality: 62
needs: Strength 167
sockets: true
price: 93250
weight: 40
crafted_with: '[[skills/2431-sword-craft|Sword Craft]] level 7'
recipe:
- material: any Metal
  quantity: 80
- material: '[[items/material/45-tender-leather|Tender Leather]]'
  quantity: 30
- material: '[[items/material/11-unprocessed-metal|Unprocessed Metal]]'
  quantity: 10
- material: '[[items/material/128-hersian|Hersian]]'
  quantity: 16
craft_difficulty: 53
dropped_by:
- monster: '[[monsters/343-yeti-guard|Yeti Guard]]'
  level: 101
  slots_of_30: 0.2
- monster: '[[monsters/347-ruper-wizard|Ruper Wizard]]'
  level: 102
  slots_of_30: 0.2
- monster: '[[monsters/348-ruper-captain|Ruper Captain]]'
  level: 106
  slots_of_30: 0.2
- monster: '[[monsters/352-rider-wizard|Rider Wizard]]'
  level: 104
  slots_of_30: 0.2
- monster: '[[monsters/357-tyrant|Tyrant]]'
  level: 112
  slots_of_30: 0.2
- monster: '[[monsters/453-seal-stone|Seal Stone]]'
  level: 100
  slots_of_30: 1
used_to_craft:
- '[[items/weapon/439-dual-katana|Dual Katana]]'
source:
  data: LIST_WEAPON.STB row 13
  code: module/src/items.rs
---
# Katana

The sword of choice for veteran samurai.
