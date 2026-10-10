---
kind: item
id: weapon/237
name: Edge Gun
status: in-game
icon: item/1215
class: Gun
level: 64
attack: 94
attack_speed: 11
range: 23
damage: physical
two_handed: true
durability: 48
quality: 57
needs: Concentration 115
sockets: true
price: 39000
weight: 25
crafted_with: '[[skills/1211-damage-support|Damage Support]] level 4'
recipe:
- material: any Wooden Material
  quantity: 75
- material: '[[items/material/6-silver-iron|Silver Iron]]'
  quantity: 20
- material: '[[items/material/100-gunpowder|Gunpowder]]'
  quantity: 4
craft_difficulty: 38
dropped_by:
- monster: '[[monsters/123-krawfy-captain|Krawfy Captain]]'
  level: 68
  slots_of_30: 0.4
- monster: '[[monsters/158-doonga-captain|Doonga Captain]]'
  level: 72
  slots_of_30: 0.2
sold_by:
- '[[npcs/1096-ferrell-guild-merchant-mildun|Ferrell Guild Merchant Mildun]]'
source:
  data: LIST_WEAPON.STB row 237
  code: module/src/items.rs
---
# Edge Gun

A pistol whose accuracy can be compared to that of ninja assassins.
