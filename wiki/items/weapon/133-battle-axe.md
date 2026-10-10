---
kind: item
id: weapon/133
name: Battle Axe
status: in-game
icon: item/1119
class: Two-Handed Axe
level: 39
attack: 62
attack_speed: 12
range: 2
damage: physical
two_handed: true
durability: 44
quality: 46
needs: Strength 90
sockets: true
price: 10485
weight: 20
crafted_with: '[[skills/2451-mace-craft|Mace Craft]] level 2'
recipe:
- material: any Metal
  quantity: 28
- material: '[[items/material/31-twig|Twig]]'
  quantity: 15
- material: '[[items/material/180-weed-stem|Weed Stem]]'
  quantity: 6
craft_difficulty: 22
dropped_by:
- monster: '[[monsters/53-old-smouly|Old Smouly]]'
  level: 48
  slots_of_30: 0.4
- monster: '[[monsters/135-grunter-fighter|Grunter Fighter]]'
  level: 44
  slots_of_30: 0.2
- monster: '[[monsters/136-grunter-warrior|Grunter Warrior]]'
  level: 45
  slots_of_30: 0.2
- monster: '[[monsters/181-small-clown|Small Clown]]'
  level: 47
  slots_of_30: 0.2
- monster: '[[monsters/182-clown|Clown]]'
  level: 49
  slots_of_30: 0.4
- monster: '[[monsters/206-wild-gorilla|Wild Gorilla]]'
  level: 43
  slots_of_30: 0.2
- monster: '[[monsters/256-needle-bat|Needle Bat]]'
  level: 46
  slots_of_30: 0.2
- monster: '[[monsters/261-goblin-jar|Goblin Jar]]'
  level: 45
  slots_of_30: 0.4
- monster: '[[monsters/271-goblin-worker|Goblin Worker]]'
  level: 51
  slots_of_30: 0.2
sold_by:
- '[[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]]'
- '[[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]]'
quest_reward:
- '[[quests/860-hovert-s-mementos|Hovert''s Mementos]]'
source:
  data: LIST_WEAPON.STB row 133
  code: module/src/items.rs
---
# Battle Axe

A specially crafted weapon that can be used as both an axe and a hammer.
