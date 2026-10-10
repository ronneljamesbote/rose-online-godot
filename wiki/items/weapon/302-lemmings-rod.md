---
kind: item
id: weapon/302
name: Lemmings Rod
status: in-game
icon: item/1288
class: Magic Staff
level: 20
attack: 28
attack_speed: 10
range: 2.5
damage: magic
two_handed: true
durability: 39
quality: 35
needs: Intelligence 44
bonus: MP Consumption +16
price: 3030
weight: 5
crafted_with: '[[skills/2491-magic-weapon-craft|Magic Weapon Craft]] level 1'
recipe:
- material: any Wooden Material
  quantity: 15
- material: '[[items/material/1-rusted-iron|Rusted Iron]]'
  quantity: 5
- material: '[[items/material/176-insect-leg|Insect Leg]]'
  quantity: 10
craft_difficulty: 19
dropped_by:
- monster: '[[monsters/34-dalping-leader|Dalping Leader]]'
  level: 26
  slots_of_30: 0.4
- monster: '[[monsters/45-big-flanae|Big Flanae]]'
  level: 21
  slots_of_30: 0.2
- monster: '[[monsters/67-queen-honeybee|Queen HoneyBee]]'
  level: 26
  slots_of_30: 0.4
- monster: '[[monsters/68-queen-bibi|Queen Bibi]]'
  level: 27
  slots_of_30: 0.4
- monster: '[[monsters/85-turtle-guard|Turtle Guard]]'
  level: 29
  slots_of_30: 0.4
- monster: '[[monsters/92-rackie-hooligan|Rackie Hooligan]]'
  level: 26
  slots_of_30: 0.2
- monster: '[[monsters/93-fighter-rackie|Fighter Rackie]]'
  level: 28
  slots_of_30: 0.6
- monster: '[[monsters/172-aqua-warrior|Aqua Warrior]]'
  level: 31
  slots_of_30: 0.2
- monster: '[[monsters/205-woopie-king|Woopie King]]'
  level: 25
  slots_of_30: 0.2
sold_by:
- '[[npcs/1006-arumic-merchant-tryteh|Arumic Merchant Tryteh]]'
source:
  data: LIST_WEAPON.STB row 302
  code: module/src/items.rs
---
# Lemmings Rod

A cute Rod that's good for handling mice.
