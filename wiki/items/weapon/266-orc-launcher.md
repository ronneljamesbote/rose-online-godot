---
kind: item
id: weapon/266
name: Orc Launcher
status: in-game
icon: item/1253
class: Launcher
level: 73
attack: 125
attack_speed: 12
range: 27
damage: physical
two_handed: true
durability: 45
quality: 63
needs: Strength 67
sockets: true
price: 99850
weight: 20
crafted_with: '[[skills/1211-damage-support|Damage Support]] level 5'
recipe:
- material: any Metal
  quantity: 120
- material: '[[items/material/41-worn-out-leather|Worn-out Leather]]'
  quantity: 15
- material: '[[items/material/100-gunpowder|Gunpowder]]'
  quantity: 10
craft_difficulty: 41
dropped_by:
- monster: '[[monsters/166-master-stone-golem|Master Stone Golem]]'
  level: 80
  slots_of_30: 0.4
- monster: '[[monsters/277-gem-goblin-guard|Gem Goblin Guard]]'
  level: 85
  slots_of_30: 0.4
- monster: '[[monsters/279-gem-goblin-warrior|Gem Goblin Warrior]]'
  level: 89
  slots_of_30: 0.4
- monster: '[[monsters/284-goblin-leader|Goblin Leader]]'
  level: 77
  slots_of_30: 0.2
sold_by:
- '[[npcs/1096-ferrell-guild-merchant-mildun|Ferrell Guild Merchant Mildun]]'
source:
  data: LIST_WEAPON.STB row 266
  code: module/src/items.rs
---
# Orc Launcher

A heavy Launcher used by the Orc Warriors.
