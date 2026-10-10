---
kind: item
id: weapon/5
name: Long Sword
status: in-game
icon: item/914
class: One-Handed Sword
level: 29
attack: 31
attack_speed: 10
range: 2
damage: physical
two_handed: false
durability: 41
quality: 36
needs: Strength 62
price: 3570
weight: 10
crafted_with: '[[skills/2431-sword-craft|Sword Craft]] level 2'
recipe:
- material: any Metal
  quantity: 17
- material: '[[items/material/41-worn-out-leather|Worn-out Leather]]'
  quantity: 3
- material: '[[items/material/176-insect-leg|Insect Leg]]'
  quantity: 2
craft_difficulty: 22
dropped_by:
- monster: '[[monsters/68-queen-bibi|Queen Bibi]]'
  level: 27
  slots_of_30: 0.2
- monster: '[[monsters/69-master-queen-bibi|Master Queen Bibi]]'
  level: 33
  slots_of_30: 0.4
- monster: '[[monsters/132-porkie|Porkie]]'
  level: 33
  slots_of_30: 0.2
- monster: '[[monsters/173-aqua-captain|Aqua Captain]]'
  level: 34
  slots_of_30: 0.4
- monster: '[[monsters/175-aqua-ranger|Aqua Ranger]]'
  level: 32
  slots_of_30: 0.2
- monster: '[[monsters/176-aqua-hunter|Aqua Hunter]]'
  level: 33
  slots_of_30: 0.6
sold_by:
- '[[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]]'
quest_reward:
- '[[quests/857-living-as-a-true-soldier|Living as a True Soldier]]'
- '[[quests/1007-an-eye-for-the-marketplace|An Eye for the Marketplace]]'
used_to_craft:
- '[[items/weapon/431-khukuri-long-sword|Khukuri & Long Sword]]'
- '[[items/weapon/432-long-sword-mace|Long Sword & Mace]]'
source:
  data: LIST_WEAPON.STB row 5
  code: module/src/items.rs
---
# Long Sword

A general sword that is favored by many.
