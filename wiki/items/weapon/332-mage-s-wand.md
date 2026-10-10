---
kind: item
id: weapon/332
name: Mage's Wand
status: in-game
icon: item/1327
class: Magic Tool
level: 21
attack: 24
attack_speed: 12
range: 18
damage: magic
two_handed: false
durability: 40
quality: 32
needs: Intelligence 50
bonus: MP Consumption +6
price: 2580
weight: 3
crafted_with: '[[skills/2491-magic-weapon-craft|Magic Weapon Craft]] level 1'
recipe:
- material: any Wooden Material
  quantity: 18
- material: '[[items/material/1-rusted-iron|Rusted Iron]]'
  quantity: 5
- material: '[[items/material/176-insect-leg|Insect Leg]]'
  quantity: 5
craft_difficulty: 20
dropped_by:
- monster: '[[monsters/33-hunter-dalping|Hunter Dalping]]'
  level: 24
  slots_of_30: 0.2
- monster: '[[monsters/68-queen-bibi|Queen Bibi]]'
  level: 27
  slots_of_30: 0.4
- monster: '[[monsters/85-turtle-guard|Turtle Guard]]'
  level: 29
  slots_of_30: 0.2
- monster: '[[monsters/93-fighter-rackie|Fighter Rackie]]'
  level: 28
  slots_of_30: 0.2
- monster: '[[monsters/96-hunter-rackie|Hunter Rackie]]'
  level: 26
  slots_of_30: 0.2
- monster: '[[monsters/111-moldie|Moldie]]'
  level: 37
  slots_of_30: 0.2
- monster: '[[monsters/132-porkie|Porkie]]'
  level: 33
  slots_of_30: 0.4
- monster: '[[monsters/171-aqua-guard|Aqua Guard]]'
  level: 30
  slots_of_30: 0.6
- monster: '[[monsters/172-aqua-warrior|Aqua Warrior]]'
  level: 31
  slots_of_30: 0.4
- monster: '[[monsters/173-aqua-captain|Aqua Captain]]'
  level: 34
  slots_of_30: 0.4
- monster: '[[monsters/175-aqua-ranger|Aqua Ranger]]'
  level: 32
  slots_of_30: 0.2
- monster: '[[monsters/176-aqua-hunter|Aqua Hunter]]'
  level: 33
  slots_of_30: 0.2
- monster: '[[monsters/205-woopie-king|Woopie King]]'
  level: 25
  slots_of_30: 0.2
sold_by:
- '[[npcs/1006-arumic-merchant-tryteh|Arumic Merchant Tryteh]]'
source:
  data: LIST_WEAPON.STB row 332
  code: module/src/items.rs
---
# Mage's Wand

A basic Wand used by Mages to focus their mystic energies.
