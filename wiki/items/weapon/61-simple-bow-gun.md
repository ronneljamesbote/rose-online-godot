---
kind: item
id: weapon/61
name: Simple Bow Gun
status: in-game
icon: item/988
class: Crossbow
level: 40
attack: 52
attack_speed: 10
range: 20
damage: physical
two_handed: false
durability: 42
quality: 44
needs: Strength 62
sockets: true
price: 8155
weight: 10
crafted_with: '[[skills/2471-bow-craft|Bow Craft]] level 2'
recipe:
- material: any Wooden Material
  quantity: 35
- material: '[[items/material/41-worn-out-leather|Worn-out Leather]]'
  quantity: 5
- material: '[[items/material/172-thick-insect-shell|Thick Insect Shell]]'
  quantity: 2
craft_difficulty: 26
dropped_by:
- monster: '[[monsters/138-grunter-leader|Grunter Leader]]'
  level: 53
  slots_of_30: 0.4
- monster: '[[monsters/161-jewel-golem|Jewel Golem]]'
  level: 47
  slots_of_30: 0.2
- monster: '[[monsters/183-fighter-clown|Fighter Clown]]'
  level: 52
  slots_of_30: 0.4
- monster: '[[monsters/185-ranger-clown|Ranger Clown]]'
  level: 48
  slots_of_30: 0.4
- monster: '[[monsters/256-needle-bat|Needle Bat]]'
  level: 46
  slots_of_30: 0.2
- monster: '[[monsters/257-bloodbat|BloodBat]]'
  level: 50
  slots_of_30: 0.2
- monster: '[[monsters/261-goblin-jar|Goblin Jar]]'
  level: 45
  slots_of_30: 0.4
sold_by:
- '[[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]]'
- '[[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]]'
source:
  data: LIST_WEAPON.STB row 61
  code: module/src/items.rs
---
# Simple Bow Gun

A simple Crossbow that can be used with one hand.
