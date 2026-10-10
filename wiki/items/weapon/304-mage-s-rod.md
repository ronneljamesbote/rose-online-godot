---
kind: item
id: weapon/304
name: Mage's Rod
status: in-game
icon: item/1290
class: Magic Staff
level: 38
attack: 47
attack_speed: 10
range: 3
damage: magic
two_handed: true
durability: 43
quality: 43
needs: Intelligence 72
bonus: MP Consumption +17
sockets: true
price: 11080
weight: 10
crafted_with: '[[skills/2491-magic-weapon-craft|Magic Weapon Craft]] level 2'
recipe:
- material: any Wooden Material
  quantity: 35
- material: '[[items/material/1-rusted-iron|Rusted Iron]]'
  quantity: 15
- material: '[[items/material/177-insect-wing|Insect Wing]]'
  quantity: 20
craft_difficulty: 25
dropped_by:
- monster: '[[monsters/51-smouly|Smouly]]'
  level: 44
  slots_of_30: 0.4
- monster: '[[monsters/52-elder-smouly|Elder Smouly]]'
  level: 46
  slots_of_30: 0.4
- monster: '[[monsters/53-old-smouly|Old Smouly]]'
  level: 48
  slots_of_30: 0.2
- monster: '[[monsters/135-grunter-fighter|Grunter Fighter]]'
  level: 44
  slots_of_30: 0.2
- monster: '[[monsters/182-clown|Clown]]'
  level: 49
  slots_of_30: 0.6
- monster: '[[monsters/206-wild-gorilla|Wild Gorilla]]'
  level: 43
  slots_of_30: 0.6
- monster: '[[monsters/207-aqua-king|Aqua King]]'
  level: 40
  slots_of_30: 0.4
sold_by:
- '[[npcs/1006-arumic-merchant-tryteh|Arumic Merchant Tryteh]]'
- '[[npcs/1091-arumic-merchant-chester|Arumic Merchant Chester]]'
quest_reward:
- '[[quests/912-an-appropriate-compromise|An Appropriate Compromise]]'
source:
  data: LIST_WEAPON.STB row 304
  code: module/src/items.rs
---
# Mage's Rod

A Mage's Rod that contains just a little bit of magic.
