---
kind: item
id: weapon/62
name: Cutter Bow Gun
status: in-game
icon: item/989
class: Crossbow
level: 50
attack: 65
attack_speed: 10
range: 20
damage: physical
two_handed: false
durability: 44
quality: 48
needs: Strength 74
sockets: true
price: 15240
weight: 10
crafted_with: '[[skills/2471-bow-craft|Bow Craft]] level 3'
recipe:
- material: any Wooden Material
  quantity: 55
- material: '[[items/material/42-thin-leather|Thin Leather]]'
  quantity: 10
- material: '[[items/material/173-limpid-skin|Limpid Skin]]'
  quantity: 3
craft_difficulty: 30
dropped_by:
- monster: '[[monsters/122-krawfy-warrior|Krawfy Warrior]]'
  level: 65
  slots_of_30: 0.4
- monster: '[[monsters/142-kaiman-guard|Kaiman Guard]]'
  level: 60
  slots_of_30: 0.2
- monster: '[[monsters/145-kaiman-hunter|Kaiman Hunter]]'
  level: 63
  slots_of_30: 0.2
- monster: '[[monsters/152-elder-doonga|Elder Doonga]]'
  level: 58
  slots_of_30: 0.4
- monster: '[[monsters/258-elder-bloodbat|Elder BloodBat]]'
  level: 60
  slots_of_30: 0.2
- monster: '[[monsters/272-coal-mine-goblin-worker|Coal Mine Goblin Worker]]'
  level: 55
  slots_of_30: 0.4
sold_by:
- '[[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]]'
source:
  data: LIST_WEAPON.STB row 62
  code: module/src/items.rs
---
# Cutter Bow Gun

A Crossbow that fires bolts which can cut through anything.
