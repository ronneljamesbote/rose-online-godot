---
kind: item
id: weapon/35
name: Onion Mace
status: in-game
icon: item/953
class: One-Handed Blunt Weapon
level: 39
attack: 52
attack_speed: 12
range: 2
damage: physical
two_handed: false
durability: 44
quality: 37
needs: Strength 84
sockets: true
price: 7920
weight: 25
crafted_with: '[[skills/2451-mace-craft|Mace Craft]] level 2'
recipe:
- material: any Metal
  quantity: 19
- material: '[[items/material/41-worn-out-leather|Worn-out Leather]]'
  quantity: 5
- material: '[[items/material/192-animal-backbone|Animal Backbone]]'
  quantity: 5
craft_difficulty: 26
dropped_by:
- monster: '[[monsters/52-elder-smouly|Elder Smouly]]'
  level: 46
  slots_of_30: 0.2
- monster: '[[monsters/53-old-smouly|Old Smouly]]'
  level: 48
  slots_of_30: 0.2
- monster: '[[monsters/135-grunter-fighter|Grunter Fighter]]'
  level: 44
  slots_of_30: 0.2
- monster: '[[monsters/182-clown|Clown]]'
  level: 49
  slots_of_30: 0.4
- monster: '[[monsters/183-fighter-clown|Fighter Clown]]'
  level: 52
  slots_of_30: 0.2
- monster: '[[monsters/208-guardian-tree|Guardian Tree]]'
  level: 57
  slots_of_30: 0.4
- monster: '[[monsters/257-bloodbat|BloodBat]]'
  level: 50
  slots_of_30: 0.2
- monster: '[[monsters/261-goblin-jar|Goblin Jar]]'
  level: 45
  slots_of_30: 0.2
sold_by:
- '[[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]]'
- '[[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]]'
quest_reward:
- '[[quests/860-hovert-s-mementos|Hovert''s Mementos]]'
source:
  data: LIST_WEAPON.STB row 35
  code: module/src/items.rs
---
# Onion Mace

A simple Melee Weapon topped with an onion.
