---
kind: item
id: weapon/64
name: Crossbow Gun
status: in-game
icon: item/991
class: Crossbow
level: 68
attack: 88
attack_speed: 9
range: 22
damage: physical
two_handed: false
durability: 47
quality: 56
needs: Strength 97
sockets: true
price: 39400
weight: 20
crafted_with: '[[skills/2471-bow-craft|Bow Craft]] level 4'
recipe:
- material: any Metal
  quantity: 55
- material: '[[items/material/44-tough-leather|Tough Leather]]'
  quantity: 20
- material: '[[items/material/175-insect-feeler|Insect Feeler]]'
  quantity: 5
craft_difficulty: 38
dropped_by:
- monster: '[[monsters/123-krawfy-captain|Krawfy Captain]]'
  level: 68
  slots_of_30: 0.2
- monster: '[[monsters/158-doonga-captain|Doonga Captain]]'
  level: 72
  slots_of_30: 0.2
- monster: '[[monsters/162-junon-s-golem|Junon''s Golem]]'
  level: 73
  slots_of_30: 0.6
- monster: '[[monsters/164-stone-golem|Stone Golem]]'
  level: 74
  slots_of_30: 0.4
- monster: '[[monsters/276-goblin-guard|Goblin Guard]]'
  level: 70
  slots_of_30: 0.2
- monster: '[[monsters/278-goblin-warrior|Goblin Warrior]]'
  level: 73
  slots_of_30: 0.4
- monster: '[[monsters/283-gold-mine-goblin-mage|Gold Mine Goblin Mage]]'
  level: 76
  slots_of_30: 0.2
- monster: '[[monsters/284-goblin-leader|Goblin Leader]]'
  level: 77
  slots_of_30: 0.2
- monster: '[[monsters/331-slag|Slag]]'
  level: 78
  slots_of_30: 0.2
- monster: '[[monsters/332-elder-slag|Elder Slag]]'
  level: 82
  slots_of_30: 0.2
sold_by:
- '[[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]]'
source:
  data: LIST_WEAPON.STB row 64
  code: module/src/items.rs
---
# Crossbow Gun

A Crossbow which provides great destructive power.
