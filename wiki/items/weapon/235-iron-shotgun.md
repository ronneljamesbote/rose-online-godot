---
kind: item
id: weapon/235
name: Iron Shotgun
status: in-game
icon: item/1213
class: Gun
level: 47
attack: 68
attack_speed: 11
range: 23
damage: physical
two_handed: true
durability: 45
quality: 50
needs: Concentration 89
sockets: true
price: 17280
weight: 20
crafted_with: '[[skills/1211-damage-support|Damage Support]] level 3'
recipe:
- material: any Wooden Material
  quantity: 48
- material: '[[items/material/4-bronze|Bronze]]'
  quantity: 12
- material: '[[items/material/179-weed-root|Weed Root]]'
  quantity: 4
craft_difficulty: 30
dropped_by:
- monster: '[[monsters/138-grunter-leader|Grunter Leader]]'
  level: 53
  slots_of_30: 0.4
- monster: '[[monsters/151-doonga|Doonga]]'
  level: 56
  slots_of_30: 0.2
- monster: '[[monsters/161-jewel-golem|Jewel Golem]]'
  level: 47
  slots_of_30: 0.2
- monster: '[[monsters/183-fighter-clown|Fighter Clown]]'
  level: 52
  slots_of_30: 0.2
- monster: '[[monsters/257-bloodbat|BloodBat]]'
  level: 50
  slots_of_30: 0.4
- monster: '[[monsters/272-coal-mine-goblin-worker|Coal Mine Goblin Worker]]'
  level: 55
  slots_of_30: 0.2
sold_by:
- '[[npcs/1096-ferrell-guild-merchant-mildun|Ferrell Guild Merchant Mildun]]'
source:
  data: LIST_WEAPON.STB row 235
  code: module/src/items.rs
---
# Iron Shotgun

A shotgun that is especially effective at point blank range. Don't get too close to it!
