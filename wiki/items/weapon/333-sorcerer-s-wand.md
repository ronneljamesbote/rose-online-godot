---
kind: item
id: weapon/333
name: Sorcerer's Wand
status: in-game
icon: item/1328
class: Magic Tool
level: 30
attack: 32
attack_speed: 12
range: 18
damage: magic
two_handed: false
durability: 42
quality: 36
needs: Intelligence 65
bonus: MP Consumption +7
price: 5180
weight: 5
crafted_with: '[[skills/2491-magic-weapon-craft|Magic Weapon Craft]] level 2'
recipe:
- material: any Wooden Material
  quantity: 23
- material: '[[items/material/1-rusted-iron|Rusted Iron]]'
  quantity: 10
- material: '[[items/material/174-sticky-husk|Sticky Husk]]'
  quantity: 7
craft_difficulty: 22
dropped_by:
- monster: '[[monsters/111-moldie|Moldie]]'
  level: 37
  slots_of_30: 0.6
- monster: '[[monsters/113-gold-mine-moldie|Gold Mine Moldie]]'
  level: 39
  slots_of_30: 0.2
- monster: '[[monsters/135-grunter-fighter|Grunter Fighter]]'
  level: 44
  slots_of_30: 0.2
- monster: '[[monsters/176-aqua-hunter|Aqua Hunter]]'
  level: 33
  slots_of_30: 0.2
- monster: '[[monsters/181-small-clown|Small Clown]]'
  level: 47
  slots_of_30: 0.2
sold_by:
- '[[npcs/1006-arumic-merchant-tryteh|Arumic Merchant Tryteh]]'
quest_reward:
- '[[quests/907-solitary-orias|Solitary Orias]]'
source:
  data: LIST_WEAPON.STB row 333
  code: module/src/items.rs
---
# Sorcerer's Wand

A Wand, used by sorcerers, in which offensive magic sealed.
