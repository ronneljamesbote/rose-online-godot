---
kind: item
id: weapon/303
name: Animal Rod
status: in-game
icon: item/1289
class: Magic Staff
level: 29
attack: 37
attack_speed: 10
range: 3
damage: magic
two_handed: true
durability: 41
quality: 39
needs: Intelligence 58
bonus: MP Consumption +16
price: 5915
weight: 10
crafted_with: '[[skills/2491-magic-weapon-craft|Magic Weapon Craft]] level 2'
recipe:
- material: any Wooden Material
  quantity: 25
- material: '[[items/material/1-rusted-iron|Rusted Iron]]'
  quantity: 10
- material: '[[items/material/174-sticky-husk|Sticky Husk]]'
  quantity: 10
craft_difficulty: 21
dropped_by:
- monster: '[[monsters/69-master-queen-bibi|Master Queen Bibi]]'
  level: 33
  slots_of_30: 0.2
- monster: '[[monsters/112-coal-mine-moldie|Coal Mine Moldie]]'
  level: 38
  slots_of_30: 0.4
- monster: '[[monsters/113-gold-mine-moldie|Gold Mine Moldie]]'
  level: 39
  slots_of_30: 0.2
- monster: '[[monsters/133-porkie-hooligan|Porkie Hooligan]]'
  level: 35
  slots_of_30: 0.4
sold_by:
- '[[npcs/1006-arumic-merchant-tryteh|Arumic Merchant Tryteh]]'
quest_reward:
- '[[quests/907-solitary-orias|Solitary Orias]]'
source:
  data: LIST_WEAPON.STB row 303
  code: module/src/items.rs
---
# Animal Rod

A shepherd's Rod that provides suitable damage.
