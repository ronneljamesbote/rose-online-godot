---
kind: item
id: weapon/337
name: Flame Wand
status: in-game
icon: item/1332
class: Magic Tool
level: 65
attack: 72
attack_speed: 12
range: 20
damage: magic
two_handed: false
durability: 48
quality: 50
needs: Intelligence 122
bonus: MP Consumption +10
sockets: true
price: 43900
weight: 5
crafted_with: '[[skills/2491-magic-weapon-craft|Magic Weapon Craft]] level 4'
recipe:
- material: any Wooden Material
  quantity: 85
- material: '[[items/material/3-copper|Copper]]'
  quantity: 40
- material: '[[items/material/190-bark|Bark]]'
  quantity: 15
craft_difficulty: 38
dropped_by:
- monster: '[[monsters/158-doonga-captain|Doonga Captain]]'
  level: 72
  slots_of_30: 0.2
- monster: '[[monsters/165-elder-stone-golem|Elder Stone Golem]]'
  level: 76
  slots_of_30: 0.2
- monster: '[[monsters/276-goblin-guard|Goblin Guard]]'
  level: 70
  slots_of_30: 0.2
- monster: '[[monsters/278-goblin-warrior|Goblin Warrior]]'
  level: 73
  slots_of_30: 0.2
- monster: '[[monsters/283-gold-mine-goblin-mage|Gold Mine Goblin Mage]]'
  level: 76
  slots_of_30: 0.4
sold_by:
- '[[npcs/1091-arumic-merchant-chester|Arumic Merchant Chester]]'
source:
  data: LIST_WEAPON.STB row 337
  code: module/src/items.rs
---
# Flame Wand

A Mage's Wand that can shoot magical flames.
