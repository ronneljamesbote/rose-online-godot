---
kind: item
id: weapon/306
name: Wisp Staff
status: in-game
icon: item/1292
class: Magic Staff
level: 56
attack: 70
attack_speed: 10
range: 3
damage: magic
two_handed: true
durability: 46
quality: 51
needs: Intelligence 99
bonus: MP Consumption +19
sockets: true
price: 30850
weight: 20
crafted_with: '[[skills/2491-magic-weapon-craft|Magic Weapon Craft]] level 3'
recipe:
- material: any Wooden Material
  quantity: 65
- material: '[[items/material/2-tin|Tin]]'
  quantity: 25
- material: '[[items/material/189-predator-claw|Predator Claw]]'
  quantity: 30
craft_difficulty: 33
dropped_by:
- monster: '[[monsters/121-krawfy|Krawfy]]'
  level: 64
  slots_of_30: 0.2
- monster: '[[monsters/142-kaiman-guard|Kaiman Guard]]'
  level: 60
  slots_of_30: 0.2
- monster: '[[monsters/143-kaiman-warrior|Kaiman Warrior]]'
  level: 64
  slots_of_30: 0.2
- monster: '[[monsters/145-kaiman-hunter|Kaiman Hunter]]'
  level: 63
  slots_of_30: 0.4
- monster: '[[monsters/154-doonga-fighter|Doonga Fighter]]'
  level: 62
  slots_of_30: 0.4
- monster: '[[monsters/155-doonga-warrior|Doonga Warrior]]'
  level: 64
  slots_of_30: 0.2
- monster: '[[monsters/156-doonga-hunter|Doonga Hunter]]'
  level: 62
  slots_of_30: 0.2
- monster: '[[monsters/273-gold-mine-goblin-worker|Gold Mine Goblin Worker]]'
  level: 65
  slots_of_30: 0.2
sold_by:
- '[[npcs/1091-arumic-merchant-chester|Arumic Merchant Chester]]'
source:
  data: LIST_WEAPON.STB row 306
  code: module/src/items.rs
---
# Wisp Staff

A monster's Staff that is always following the light of spirits.
