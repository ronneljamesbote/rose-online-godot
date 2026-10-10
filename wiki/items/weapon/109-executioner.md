---
kind: item
id: weapon/109
name: Executioner
status: in-game
icon: item/1048
class: Two-Handed Sword
level: 80
attack: 130
attack_speed: 12
range: 2.5
damage: physical
two_handed: true
durability: 51
quality: 62
needs: Strength 149
sockets: true
price: 67600
weight: 30
crafted_with: '[[skills/2431-sword-craft|Sword Craft]] level 5'
recipe:
- material: any Metal
  quantity: 78
- material: '[[items/material/53-linsey-woolsey|Linsey-woolsey]]'
  quantity: 10
- material: '[[items/material/99-broken-metal-fragment|Broken Metal Fragment]]'
  quantity: 9
craft_difficulty: 41
dropped_by:
- monster: '[[monsters/163-master-golem|Master Golem]]'
  level: 84
  slots_of_30: 0.2
- monster: '[[monsters/277-gem-goblin-guard|Gem Goblin Guard]]'
  level: 85
  slots_of_30: 0.4
- monster: '[[monsters/281-goblin-mage|Goblin Mage]]'
  level: 88
  slots_of_30: 0.2
- monster: '[[monsters/336-wolf|Wolf]]'
  level: 85
  slots_of_30: 0.2
- monster: '[[monsters/339-lunar-wolf|Lunar Wolf]]'
  level: 90
  slots_of_30: 0.2
- monster: '[[monsters/341-yeti|Yeti]]'
  level: 94
  slots_of_30: 0.4
- monster: '[[monsters/389-wolf-keeper|Wolf Keeper]]'
  level: 112
  slots_of_30: 0.2
- monster: '[[monsters/471-tirwin|Tirwin]]'
  level: 78
  slots_of_30: 0.4
- monster: '[[monsters/472-tirwin|Tirwin]]'
  level: 86
  slots_of_30: 0.4
- monster: '[[monsters/481-tirwin|Tirwin]]'
  level: 82
  slots_of_30: 0.4
- monster: '[[monsters/482-tirwin|Tirwin]]'
  level: 88
  slots_of_30: 0.4
source:
  data: LIST_WEAPON.STB row 109
  code: module/src/items.rs
---
# Executioner

A sword used in the gruesome task of decapitation.
