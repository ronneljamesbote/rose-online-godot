---
kind: item
id: weapon/269
name: Burning Launcher
status: in-game
icon: item/1256
class: Launcher
level: 96
attack: 173
attack_speed: 12
range: 28
damage: physical
two_handed: true
durability: 48
quality: 73
needs: Strength 87
sockets: true
price: 222950
weight: 20
crafted_with: '[[skills/1211-damage-support|Damage Support]] level 7'
recipe:
- material: any Metal
  quantity: 180
- material: '[[items/material/44-tough-leather|Tough Leather]]'
  quantity: 60
- material: '[[items/material/101-thick-iron-piece|Thick Iron Piece]]'
  quantity: 25
- material: '[[items/material/128-hersian|Hersian]]'
  quantity: 15
craft_difficulty: 53
dropped_by:
- monster: '[[monsters/287-grandmaster-goblin|Grandmaster Goblin]]'
  level: 105
  slots_of_30: 0.4
- monster: '[[monsters/348-ruper-captain|Ruper Captain]]'
  level: 106
  slots_of_30: 0.2
- monster: '[[monsters/351-yeti-rider|Yeti Rider]]'
  level: 100
  slots_of_30: 0.2
- monster: '[[monsters/453-seal-stone|Seal Stone]]'
  level: 100
  slots_of_30: 1
- monster: '[[monsters/473-tirwin|Tirwin]]'
  level: 98
  slots_of_30: 0.4
- monster: '[[monsters/474-tirwin|Tirwin]]'
  level: 106
  slots_of_30: 0.4
- monster: '[[monsters/483-tirwin|Tirwin]]'
  level: 102
  slots_of_30: 0.4
- monster: '[[monsters/484-tirwin|Tirwin]]'
  level: 108
  slots_of_30: 0.4
source:
  data: LIST_WEAPON.STB row 269
  code: module/src/items.rs
---
# Burning Launcher

Launcher with explosive power that shoots flaming shells.
