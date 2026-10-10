---
kind: item
id: weapon/204
name: Orc Bow
status: in-game
icon: item/1160
class: Bow
level: 29
attack: 43
attack_speed: 10
range: 22
damage: physical
two_handed: true
durability: 41
quality: 41
needs: Dexterity 60
price: 4480
weight: 10
crafted_with: '[[skills/2471-bow-craft|Bow Craft]] level 2'
recipe:
- material: any Wooden Material
  quantity: 19
- material: '[[items/material/41-worn-out-leather|Worn-out Leather]]'
  quantity: 5
- material: '[[items/material/185-white-animal-tail-fur|White Animal Tail Fur]]'
  quantity: 5
craft_difficulty: 22
dropped_by:
- monster: '[[monsters/69-master-queen-bibi|Master Queen Bibi]]'
  level: 33
  slots_of_30: 0.4
- monster: '[[monsters/112-coal-mine-moldie|Coal Mine Moldie]]'
  level: 38
  slots_of_30: 0.2
- monster: '[[monsters/133-porkie-hooligan|Porkie Hooligan]]'
  level: 35
  slots_of_30: 0.4
- monster: '[[monsters/135-grunter-fighter|Grunter Fighter]]'
  level: 44
  slots_of_30: 0.2
- monster: '[[monsters/173-aqua-captain|Aqua Captain]]'
  level: 34
  slots_of_30: 0.4
- monster: '[[monsters/176-aqua-hunter|Aqua Hunter]]'
  level: 33
  slots_of_30: 0.2
sold_by:
- '[[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]]'
quest_reward:
- '[[quests/956-why-did-toddy-steal-the-jewels|Why Did Toddy Steal the Jewels?]]'
source:
  data: LIST_WEAPON.STB row 204
  code: module/src/items.rs
---
# Orc Bow

The basic Bow for Orc Archers.
