---
kind: item
id: weapon/165
name: Bardiche
status: in-game
icon: item/1083
class: Spear
level: 40
attack: 58
attack_speed: 10
range: 3.5
damage: physical
two_handed: true
durability: 43
quality: 39
needs: Strength 77
sockets: true
price: 9840
weight: 25
crafted_with: '[[skills/2451-mace-craft|Mace Craft]] level 2'
recipe:
- material: any Metal
  quantity: 28
- material: '[[items/material/32-maple-wood|Maple Wood]]'
  quantity: 9
- material: '[[items/material/173-limpid-skin|Limpid Skin]]'
  quantity: 9
craft_difficulty: 26
dropped_by:
- monster: '[[monsters/136-grunter-warrior|Grunter Warrior]]'
  level: 45
  slots_of_30: 0.2
- monster: '[[monsters/139-grunter-captain|Grunter Captain]]'
  level: 55
  slots_of_30: 0.2
- monster: '[[monsters/161-jewel-golem|Jewel Golem]]'
  level: 47
  slots_of_30: 0.2
- monster: '[[monsters/183-fighter-clown|Fighter Clown]]'
  level: 52
  slots_of_30: 0.2
- monster: '[[monsters/185-ranger-clown|Ranger Clown]]'
  level: 48
  slots_of_30: 0.6
- monster: '[[monsters/256-needle-bat|Needle Bat]]'
  level: 46
  slots_of_30: 0.2
- monster: '[[monsters/257-bloodbat|BloodBat]]'
  level: 50
  slots_of_30: 0.2
- monster: '[[monsters/261-goblin-jar|Goblin Jar]]'
  level: 45
  slots_of_30: 0.2
sold_by:
- '[[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]]'
- '[[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]]'
source:
  data: LIST_WEAPON.STB row 165
  code: module/src/items.rs
---
# Bardiche

A unique Spear whose point is shaped like an axe blade.
