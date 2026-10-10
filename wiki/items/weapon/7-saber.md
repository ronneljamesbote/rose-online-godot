---
kind: item
id: weapon/7
name: Saber
status: in-game
icon: item/916
class: One-Handed Sword
level: 47
attack: 51
attack_speed: 10
range: 2
damage: physical
two_handed: false
durability: 45
quality: 43
needs: Strength 91
sockets: true
price: 11880
weight: 15
crafted_with: '[[skills/2431-sword-craft|Sword Craft]] level 3'
recipe:
- material: any Metal
  quantity: 30
- material: '[[items/material/42-thin-leather|Thin Leather]]'
  quantity: 13
- material: '[[items/material/180-weed-stem|Weed Stem]]'
  quantity: 4
craft_difficulty: 30
dropped_by:
- monster: '[[monsters/138-grunter-leader|Grunter Leader]]'
  level: 53
  slots_of_30: 0.6
- monster: '[[monsters/141-kaiman|Kaiman]]'
  level: 59
  slots_of_30: 0.4
- monster: '[[monsters/152-elder-doonga|Elder Doonga]]'
  level: 58
  slots_of_30: 0.2
- monster: '[[monsters/161-jewel-golem|Jewel Golem]]'
  level: 47
  slots_of_30: 0.4
- monster: '[[monsters/183-fighter-clown|Fighter Clown]]'
  level: 52
  slots_of_30: 0.2
- monster: '[[monsters/186-hunter-clown|Hunter Clown]]'
  level: 50
  slots_of_30: 0.6
- monster: '[[monsters/257-bloodbat|BloodBat]]'
  level: 50
  slots_of_30: 0.4
- monster: '[[monsters/261-goblin-jar|Goblin Jar]]'
  level: 45
  slots_of_30: 0.2
- monster: '[[monsters/274-goblin-server|Goblin Server]]'
  level: 52
  slots_of_30: 0.2
sold_by:
- '[[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]]'
used_to_craft:
- '[[items/weapon/435-saber-elven-sword|Saber & Elven Sword]]'
source:
  data: LIST_WEAPON.STB row 7
  code: module/src/items.rs
---
# Saber

A dashing sword which cuts through the wind.
