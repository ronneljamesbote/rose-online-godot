---
kind: item
id: weapon/44
name: Battle Mace
status: in-game
icon: item/963
class: One-Handed Blunt Weapon
level: 110
attack: 170
attack_speed: 12
range: 2
damage: physical
two_handed: false
durability: 57
quality: 64
needs: Strength 205
sockets: true
price: 165850
weight: 60
crafted_with: '[[skills/2451-mace-craft|Mace Craft]] level 8'
recipe:
- material: any Metal
  quantity: 105
- material: '[[items/material/46-gorgeous-leather|Gorgeous Leather]]'
  quantity: 45
- material: '[[items/material/13-wolf-steel|Wolf Steel]]'
  quantity: 25
- material: '[[items/material/129-jilsian|Jilsian]]'
  quantity: 18
craft_difficulty: 61
dropped_by:
- monster: '[[monsters/368-winter-maul|Winter Maul]]'
  level: 114
  slots_of_30: 0.2
- monster: '[[monsters/371-basilisk|Basilisk]]'
  level: 121
  slots_of_30: 0.4
source:
  data: LIST_WEAPON.STB row 44
  code: module/src/items.rs
---
# Battle Mace

A Melee Weapon that has been optimized for use in battle.
