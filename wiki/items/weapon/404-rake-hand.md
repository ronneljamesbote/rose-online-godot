---
kind: item
id: weapon/404
name: Rake Hand
status: in-game
icon: item/1381
class: Katar
level: 38
attack: 45
attack_speed: 7
range: 2.5
damage: physical
two_handed: true
durability: 43
quality: 37
needs: Dexterity 72
sockets: true
price: 7640
weight: 20
crafted_with: '[[skills/2451-mace-craft|Mace Craft]] level 2'
recipe:
- material: any Metal
  quantity: 25
- material: '[[items/material/34-oak-wood|Oak Wood]]'
  quantity: 8
- material: '[[items/material/177-insect-wing|Insect Wing]]'
  quantity: 3
craft_difficulty: 26
dropped_by:
- monster: '[[monsters/52-elder-smouly|Elder Smouly]]'
  level: 46
  slots_of_30: 0.6
- monster: '[[monsters/135-grunter-fighter|Grunter Fighter]]'
  level: 44
  slots_of_30: 0.2
- monster: '[[monsters/182-clown|Clown]]'
  level: 49
  slots_of_30: 0.6
- monster: '[[monsters/257-bloodbat|BloodBat]]'
  level: 50
  slots_of_30: 0.2
sold_by:
- '[[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]]'
- '[[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]]'
quest_reward:
- '[[quests/957-the-essence-of-speed|The Essence of Speed]]'
- '[[quests/958-the-essence-of-speed|The Essence of Speed]]'
- '[[quests/959-the-essence-of-speed|The Essence of Speed]]'
source:
  data: LIST_WEAPON.STB row 404
  code: module/src/items.rs
---
# Rake Hand

A sharp, rake-like weapon that can gather fallen leaves.
