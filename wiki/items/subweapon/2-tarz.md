---
kind: item
id: subweapon/2
name: Tarz
status: in-game
icon: item/1496
class: Shield
defence: 5
resistance: 1
durability: 20
quality: 14
needs: Strength 20
bonus: Attack Speed -5
price: 750
weight: 12
crafted_with: '[[skills/2411-subitem-craft|SubItem Craft]] level 1'
recipe:
- material: any Wooden Material
  quantity: 6
- material: '[[items/material/41-worn-out-leather|Worn-out Leather]]'
  quantity: 3
- material: '[[items/material/172-thick-insect-shell|Thick Insect Shell]]'
  quantity: 1
craft_difficulty: 12
dropped_by:
- monster: '[[monsters/34-dalping-leader|Dalping Leader]]'
  level: 26
  slots_of_30: 0.4
- monster: '[[monsters/63-needle-hornet|Needle Hornet]]'
  level: 15
  slots_of_30: 0.2
- monster: '[[monsters/64-hornet|Hornet]]'
  level: 18
  slots_of_30: 0.2
- monster: '[[monsters/74-pomic-soldier|Pomic Soldier]]'
  level: 17
  slots_of_30: 0.2
- monster: '[[monsters/81-beetle|Beetle]]'
  level: 20
  slots_of_30: 0.2
- monster: '[[monsters/84-turtle|Turtle]]'
  level: 27
  slots_of_30: 0.2
- monster: '[[monsters/91-honey-rackie|Honey Rackie]]'
  level: 24
  slots_of_30: 0.2
- monster: '[[monsters/204-wandering-rackie|Wandering Rackie]]'
  level: 20
  slots_of_30: 0.2
sold_by:
- '[[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]]'
source:
  data: LIST_SUBWPN.STB row 2
  code: module/src/items.rs
---
# Tarz

A strong shield that is made by plaiting pieces of leather.
