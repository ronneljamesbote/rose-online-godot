---
kind: item
id: subweapon/4
name: Buckler
status: in-game
icon: item/1498
class: Shield
defence: 8
resistance: 2
durability: 24
quality: 18
needs: Strength 52
bonus: Attack Speed -5
sockets: true
price: 2400
weight: 12
crafted_with: '[[skills/2411-subitem-craft|SubItem Craft]] level 1'
recipe:
- material: any Metal
  quantity: 12
- material: '[[items/material/42-thin-leather|Thin Leather]]'
  quantity: 5
- material: '[[items/material/172-thick-insect-shell|Thick Insect Shell]]'
  quantity: 3
craft_difficulty: 18
dropped_by:
- monster: '[[monsters/111-moldie|Moldie]]'
  level: 37
  slots_of_30: 0.4
- monster: '[[monsters/113-gold-mine-moldie|Gold Mine Moldie]]'
  level: 39
  slots_of_30: 0.4
- monster: '[[monsters/134-grunter|Grunter]]'
  level: 42
  slots_of_30: 0.2
- monster: '[[monsters/181-small-clown|Small Clown]]'
  level: 47
  slots_of_30: 0.2
- monster: '[[monsters/206-wild-gorilla|Wild Gorilla]]'
  level: 43
  slots_of_30: 0.2
- monster: '[[monsters/256-needle-bat|Needle Bat]]'
  level: 46
  slots_of_30: 0.2
sold_by:
- '[[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]]'
- '[[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]]'
source:
  data: LIST_SUBWPN.STB row 4
  code: module/src/items.rs
---
# Buckler

A charmingly decorated shield.
