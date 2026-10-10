---
kind: item
id: subweapon/5
name: Aspis
status: in-game
icon: item/1499
class: Shield
defence: 11
resistance: 3
durability: 26
quality: 20
needs: Strength 68
bonus: Attack Speed -5
sockets: true
price: 4600
weight: 12
crafted_with: '[[skills/2411-subitem-craft|SubItem Craft]] level 2'
recipe:
- material: any Metal
  quantity: 18
- material: '[[items/material/42-thin-leather|Thin Leather]]'
  quantity: 6
- material: '[[items/material/172-thick-insect-shell|Thick Insect Shell]]'
  quantity: 4
craft_difficulty: 21
dropped_by:
- monster: '[[monsters/138-grunter-leader|Grunter Leader]]'
  level: 53
  slots_of_30: 0.2
- monster: '[[monsters/139-grunter-captain|Grunter Captain]]'
  level: 55
  slots_of_30: 0.4
- monster: '[[monsters/183-fighter-clown|Fighter Clown]]'
  level: 52
  slots_of_30: 0.2
- monster: '[[monsters/257-bloodbat|BloodBat]]'
  level: 50
  slots_of_30: 0.4
- monster: '[[monsters/271-goblin-worker|Goblin Worker]]'
  level: 51
  slots_of_30: 0.4
sold_by:
- '[[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]]'
source:
  data: LIST_SUBWPN.STB row 5
  code: module/src/items.rs
---
# Aspis

A shield with pointy spikes intended to menace enemies.
