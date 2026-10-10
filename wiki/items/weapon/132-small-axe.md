---
kind: item
id: weapon/132
name: Small Axe
status: in-game
icon: item/1120
class: Two-Handed Axe
level: 30
attack: 47
attack_speed: 11
range: 1.5
damage: physical
two_handed: true
durability: 42
quality: 42
needs: Strength 73
price: 5840
weight: 20
crafted_with: '[[skills/2451-mace-craft|Mace Craft]] level 2'
recipe:
- material: any Metal
  quantity: 18
- material: '[[items/material/31-twig|Twig]]'
  quantity: 10
- material: '[[items/material/179-weed-root|Weed Root]]'
  quantity: 5
craft_difficulty: 20
dropped_by:
- monster: '[[monsters/111-moldie|Moldie]]'
  level: 37
  slots_of_30: 0.4
- monster: '[[monsters/113-gold-mine-moldie|Gold Mine Moldie]]'
  level: 39
  slots_of_30: 0.2
- monster: '[[monsters/176-aqua-hunter|Aqua Hunter]]'
  level: 33
  slots_of_30: 0.2
- monster: '[[monsters/181-small-clown|Small Clown]]'
  level: 47
  slots_of_30: 0.2
- monster: '[[monsters/207-aqua-king|Aqua King]]'
  level: 40
  slots_of_30: 0.4
sold_by:
- '[[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]]'
quest_reward:
- '[[quests/857-living-as-a-true-soldier|Living as a True Soldier]]'
source:
  data: LIST_WEAPON.STB row 132
  code: module/src/items.rs
---
# Small Axe

An Axe commonly used by ruffians and trouble makers.
