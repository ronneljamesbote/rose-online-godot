---
kind: item
id: weapon/63
name: Quick Bow Gun
status: in-game
icon: item/990
class: Crossbow
level: 59
attack: 71
attack_speed: 8
range: 20
damage: physical
two_handed: false
durability: 45
quality: 52
needs: Strength 86
sockets: true
price: 23130
weight: 15
crafted_with: '[[skills/2471-bow-craft|Bow Craft]] level 3'
recipe:
- material: any Metal
  quantity: 45
- material: '[[items/material/43-sleek-leather|Sleek Leather]]'
  quantity: 15
- material: '[[items/material/174-sticky-husk|Sticky Husk]]'
  quantity: 4
craft_difficulty: 34
dropped_by:
- monster: '[[monsters/145-kaiman-hunter|Kaiman Hunter]]'
  level: 63
  slots_of_30: 0.6
- monster: '[[monsters/153-doonga-origin|Doonga Origin]]'
  level: 68
  slots_of_30: 0.2
- monster: '[[monsters/155-doonga-warrior|Doonga Warrior]]'
  level: 64
  slots_of_30: 0.4
- monster: '[[monsters/158-doonga-captain|Doonga Captain]]'
  level: 72
  slots_of_30: 0.2
- monster: '[[monsters/273-gold-mine-goblin-worker|Gold Mine Goblin Worker]]'
  level: 65
  slots_of_30: 0.4
- monster: '[[monsters/275-gold-mine-goblin-server|Gold Mine Goblin Server]]'
  level: 67
  slots_of_30: 0.6
- monster: '[[monsters/276-goblin-guard|Goblin Guard]]'
  level: 70
  slots_of_30: 0.2
sold_by:
- '[[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]]'
source:
  data: LIST_WEAPON.STB row 63
  code: module/src/items.rs
---
# Quick Bow Gun

A Crossbow that provides superior rapid firing power.
