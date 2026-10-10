---
kind: item
id: weapon/203
name: Long Bow
status: in-game
icon: item/1159
class: Bow
level: 20
attack: 33
attack_speed: 10
range: 22
damage: physical
two_handed: true
durability: 40
quality: 37
needs: Dexterity 45
price: 2460
weight: 10
crafted_with: '[[skills/2471-bow-craft|Bow Craft]] level 1'
recipe:
- material: any Wooden Material
  quantity: 13
- material: '[[items/material/41-worn-out-leather|Worn-out Leather]]'
  quantity: 2
- material: '[[items/material/180-weed-stem|Weed Stem]]'
  quantity: 4
craft_difficulty: 20
dropped_by:
- monster: '[[monsters/33-hunter-dalping|Hunter Dalping]]'
  level: 24
  slots_of_30: 0.2
- monster: '[[monsters/34-dalping-leader|Dalping Leader]]'
  level: 26
  slots_of_30: 0.4
- monster: '[[monsters/67-queen-honeybee|Queen HoneyBee]]'
  level: 26
  slots_of_30: 0.2
- monster: '[[monsters/68-queen-bibi|Queen Bibi]]'
  level: 27
  slots_of_30: 0.2
- monster: '[[monsters/84-turtle|Turtle]]'
  level: 27
  slots_of_30: 0.2
- monster: '[[monsters/85-turtle-guard|Turtle Guard]]'
  level: 29
  slots_of_30: 0.4
- monster: '[[monsters/91-honey-rackie|Honey Rackie]]'
  level: 24
  slots_of_30: 0.2
- monster: '[[monsters/96-hunter-rackie|Hunter Rackie]]'
  level: 26
  slots_of_30: 0.6
- monster: '[[monsters/205-woopie-king|Woopie King]]'
  level: 25
  slots_of_30: 0.2
sold_by:
- '[[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]]'
source:
  data: LIST_WEAPON.STB row 203
  code: module/src/items.rs
---
# Long Bow

The most fundamental Bow for established Bow attack strategies.
