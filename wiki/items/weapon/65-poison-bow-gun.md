---
kind: item
id: weapon/65
name: Poison Bow Gun
status: in-game
icon: item/992
class: Crossbow
level: 77
attack: 106
attack_speed: 10
range: 22
damage: physical
two_handed: false
durability: 48
quality: 60
needs: Strength 109
sockets: true
price: 57050
weight: 20
crafted_with: '[[skills/2471-bow-craft|Bow Craft]] level 5'
recipe:
- material: any Metal
  quantity: 65
- material: '[[items/material/45-tender-leather|Tender Leather]]'
  quantity: 25
- material: '[[items/material/185-white-animal-tail-fur|White Animal Tail Fur]]'
  quantity: 10
craft_difficulty: 42
dropped_by:
- monster: '[[monsters/163-master-golem|Master Golem]]'
  level: 84
  slots_of_30: 0.2
- monster: '[[monsters/166-master-stone-golem|Master Stone Golem]]'
  level: 80
  slots_of_30: 0.2
- monster: '[[monsters/451-seal-stone|Seal Stone]]'
  level: 80
  slots_of_30: 1
- monster: '[[monsters/452-seal-stone|Seal Stone]]'
  level: 88
  slots_of_30: 1
- monster: '[[monsters/471-tirwin|Tirwin]]'
  level: 78
  slots_of_30: 0.4
- monster: '[[monsters/472-tirwin|Tirwin]]'
  level: 86
  slots_of_30: 0.4
- monster: '[[monsters/481-tirwin|Tirwin]]'
  level: 82
  slots_of_30: 0.4
- monster: '[[monsters/482-tirwin|Tirwin]]'
  level: 88
  slots_of_30: 0.4
sold_by:
- '[[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]]'
source:
  data: LIST_WEAPON.STB row 65
  code: module/src/items.rs
---
# Poison Bow Gun

A Crossbow smothered with poison curse magic.
