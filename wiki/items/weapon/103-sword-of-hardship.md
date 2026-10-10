---
kind: item
id: weapon/103
name: Sword of Hardship
status: in-game
icon: item/1042
class: Two-Handed Sword
level: 29
attack: 46
attack_speed: 11
range: 2.5
damage: physical
two_handed: true
durability: 42
quality: 40
needs: Strength 65
price: 5075
weight: 20
crafted_with: '[[skills/2431-sword-craft|Sword Craft]] level 2'
recipe:
- material: any Metal
  quantity: 18
- material: '[[items/material/51-tough-cloth|Tough Cloth]]'
  quantity: 4
- material: '[[items/material/176-insect-leg|Insect Leg]]'
  quantity: 3
craft_difficulty: 20
dropped_by:
- monster: '[[monsters/69-master-queen-bibi|Master Queen Bibi]]'
  level: 33
  slots_of_30: 0.6
- monster: '[[monsters/112-coal-mine-moldie|Coal Mine Moldie]]'
  level: 38
  slots_of_30: 0.2
- monster: '[[monsters/132-porkie|Porkie]]'
  level: 33
  slots_of_30: 0.2
- monster: '[[monsters/134-grunter|Grunter]]'
  level: 42
  slots_of_30: 0.2
- monster: '[[monsters/173-aqua-captain|Aqua Captain]]'
  level: 34
  slots_of_30: 0.4
- monster: '[[monsters/207-aqua-king|Aqua King]]'
  level: 40
  slots_of_30: 0.4
sold_by:
- '[[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]]'
source:
  data: LIST_WEAPON.STB row 103
  code: module/src/items.rs
---
# Sword of Hardship

The more hardship you experience with this sword, the more you gain experience.
