---
kind: item
id: weapon/34
name: Ogre Mace
status: in-game
icon: item/952
class: One-Handed Blunt Weapon
level: 30
attack: 39
attack_speed: 11
range: 2
damage: physical
two_handed: false
durability: 42
quality: 34
needs: Strength 69
price: 4445
weight: 20
crafted_with: '[[skills/2451-mace-craft|Mace Craft]] level 2'
recipe:
- material: any Metal
  quantity: 14
- material: '[[items/material/41-worn-out-leather|Worn-out Leather]]'
  quantity: 4
- material: '[[items/material/189-predator-claw|Predator Claw]]'
  quantity: 4
craft_difficulty: 22
dropped_by:
- monster: '[[monsters/111-moldie|Moldie]]'
  level: 37
  slots_of_30: 0.4
- monster: '[[monsters/113-gold-mine-moldie|Gold Mine Moldie]]'
  level: 39
  slots_of_30: 0.2
- monster: '[[monsters/133-porkie-hooligan|Porkie Hooligan]]'
  level: 35
  slots_of_30: 0.4
- monster: '[[monsters/176-aqua-hunter|Aqua Hunter]]'
  level: 33
  slots_of_30: 0.2
sold_by:
- '[[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]]'
source:
  data: LIST_WEAPON.STB row 34
  code: module/src/items.rs
---
# Ogre Mace

A solid weapon made from the skeleton of an Ogre.
