---
kind: item
id: weapon/8
name: Elven Sword
status: in-game
icon: item/917
class: One-Handed Sword
level: 56
attack: 62
attack_speed: 10
range: 2
damage: physical
two_handed: false
durability: 46
quality: 46
needs: Strength 105
sockets: true
price: 19750
weight: 15
crafted_with: '[[skills/2431-sword-craft|Sword Craft]] level 3'
recipe:
- material: any Metal
  quantity: 45
- material: '[[items/material/42-thin-leather|Thin Leather]]'
  quantity: 14
- material: '[[items/material/102-sharp-iron-piece|Sharp Iron Piece]]'
  quantity: 3
craft_difficulty: 34
dropped_by:
- monster: '[[monsters/121-krawfy|Krawfy]]'
  level: 64
  slots_of_30: 0.2
- monster: '[[monsters/123-krawfy-captain|Krawfy Captain]]'
  level: 68
  slots_of_30: 0.2
- monster: '[[monsters/142-kaiman-guard|Kaiman Guard]]'
  level: 60
  slots_of_30: 0.8
- monster: '[[monsters/151-doonga|Doonga]]'
  level: 56
  slots_of_30: 0.4
- monster: '[[monsters/152-elder-doonga|Elder Doonga]]'
  level: 58
  slots_of_30: 0.4
- monster: '[[monsters/154-doonga-fighter|Doonga Fighter]]'
  level: 62
  slots_of_30: 0.4
- monster: '[[monsters/258-elder-bloodbat|Elder BloodBat]]'
  level: 60
  slots_of_30: 0.4
sold_by:
- '[[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]]'
used_to_craft:
- '[[items/weapon/435-saber-elven-sword|Saber & Elven Sword]]'
source:
  data: LIST_WEAPON.STB row 8
  code: module/src/items.rs
---
# Elven Sword

An imitation of the legendary swords used by the Elves.
