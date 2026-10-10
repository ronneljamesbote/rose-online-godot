---
kind: item
id: weapon/263
name: Hard Launcher
status: in-game
icon: item/1250
class: Launcher
level: 48
attack: 81
attack_speed: 12
range: 26
damage: physical
two_handed: true
durability: 41
quality: 52
needs: Strength 44
sockets: true
price: 35900
weight: 20
crafted_with: '[[skills/1211-damage-support|Damage Support]] level 3'
recipe:
- material: any Metal
  quantity: 70
- material: '[[items/material/53-linsey-woolsey|Linsey-woolsey]]'
  quantity: 30
- material: '[[items/material/100-gunpowder|Gunpowder]]'
  quantity: 4
craft_difficulty: 30
dropped_by:
- monster: '[[monsters/139-grunter-captain|Grunter Captain]]'
  level: 55
  slots_of_30: 0.2
- monster: '[[monsters/141-kaiman|Kaiman]]'
  level: 59
  slots_of_30: 0.2
- monster: '[[monsters/151-doonga|Doonga]]'
  level: 56
  slots_of_30: 0.2
- monster: '[[monsters/183-fighter-clown|Fighter Clown]]'
  level: 52
  slots_of_30: 0.2
- monster: '[[monsters/186-hunter-clown|Hunter Clown]]'
  level: 50
  slots_of_30: 0.2
- monster: '[[monsters/208-guardian-tree|Guardian Tree]]'
  level: 57
  slots_of_30: 0.4
- monster: '[[monsters/209-grunter-king|Grunter King]]'
  level: 60
  slots_of_30: 0.4
- monster: '[[monsters/271-goblin-worker|Goblin Worker]]'
  level: 51
  slots_of_30: 0.4
sold_by:
- '[[npcs/1096-ferrell-guild-merchant-mildun|Ferrell Guild Merchant Mildun]]'
source:
  data: LIST_WEAPON.STB row 263
  code: module/src/items.rs
---
# Hard Launcher

A tough Launcher that won't break easily.
