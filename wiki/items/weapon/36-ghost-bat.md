---
kind: item
id: weapon/36
name: Ghost Bat
status: in-game
icon: item/954
class: One-Handed Blunt Weapon
level: 48
attack: 63
attack_speed: 12
range: 2
damage: physical
two_handed: false
durability: 45
quality: 41
needs: Strength 100
sockets: true
price: 13905
weight: 30
crafted_with: '[[skills/2451-mace-craft|Mace Craft]] level 3'
recipe:
- material: any Metal
  quantity: 26
- material: '[[items/material/42-thin-leather|Thin Leather]]'
  quantity: 6
- material: '[[items/material/99-broken-metal-fragment|Broken Metal Fragment]]'
  quantity: 3
craft_difficulty: 30
dropped_by:
- monster: '[[monsters/138-grunter-leader|Grunter Leader]]'
  level: 53
  slots_of_30: 0.2
- monster: '[[monsters/139-grunter-captain|Grunter Captain]]'
  level: 55
  slots_of_30: 0.6
- monster: '[[monsters/183-fighter-clown|Fighter Clown]]'
  level: 52
  slots_of_30: 0.2
- monster: '[[monsters/271-goblin-worker|Goblin Worker]]'
  level: 51
  slots_of_30: 0.4
sold_by:
- '[[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]]'
used_to_craft:
- '[[items/weapon/434-twin-ghost-bat|Twin Ghost Bat]]'
source:
  data: LIST_WEAPON.STB row 36
  code: module/src/items.rs
---
# Ghost Bat

An enchanted bat readily used by demons.
