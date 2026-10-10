---
kind: item
id: weapon/265
name: Marble Launcher
status: in-game
icon: item/1252
class: Launcher
level: 65
attack: 110
attack_speed: 12
range: 27
damage: physical
two_handed: true
durability: 44
quality: 59
needs: Strength 59
sockets: true
price: 73700
weight: 20
crafted_with: '[[skills/1211-damage-support|Damage Support]] level 4'
recipe:
- material: any Metal
  quantity: 100
- material: '[[items/material/55-silk|Silk]]'
  quantity: 40
- material: '[[items/material/100-gunpowder|Gunpowder]]'
  quantity: 8
craft_difficulty: 38
dropped_by:
- monster: '[[monsters/157-doonga-leader|Doonga Leader]]'
  level: 70
  slots_of_30: 0.2
- monster: '[[monsters/211-junon-s-kingkong|Junon''s KingKong]]'
  level: 70
  slots_of_30: 0.4
- monster: '[[monsters/278-goblin-warrior|Goblin Warrior]]'
  level: 73
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
- '[[npcs/1096-ferrell-guild-merchant-mildun|Ferrell Guild Merchant Mildun]]'
source:
  data: LIST_WEAPON.STB row 265
  code: module/src/items.rs
---
# Marble Launcher

A fancy launcher made of out high quality marble.
