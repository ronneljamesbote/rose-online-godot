---
kind: item
id: weapon/11
name: Viking Sword
status: in-game
icon: item/920
class: One-Handed Sword
level: 80
attack: 100
attack_speed: 11
range: 2
damage: physical
two_handed: false
durability: 51
quality: 56
needs: Strength 143
sockets: true
price: 55950
weight: 30
crafted_with: '[[skills/2431-sword-craft|Sword Craft]] level 5'
recipe:
- material: any Metal
  quantity: 60
- material: '[[items/material/44-tough-leather|Tough Leather]]'
  quantity: 17
- material: '[[items/material/99-broken-metal-fragment|Broken Metal Fragment]]'
  quantity: 15
- material: '[[items/material/126-lesian|Lesian]]'
  quantity: 12
craft_difficulty: 45
dropped_by:
- monster: '[[monsters/163-master-golem|Master Golem]]'
  level: 84
  slots_of_30: 0.2
- monster: '[[monsters/277-gem-goblin-guard|Gem Goblin Guard]]'
  level: 85
  slots_of_30: 0.2
- monster: '[[monsters/281-goblin-mage|Goblin Mage]]'
  level: 88
  slots_of_30: 0.2
- monster: '[[monsters/452-seal-stone|Seal Stone]]'
  level: 88
  slots_of_30: 1
- monster: '[[monsters/471-tirwin|Tirwin]]'
  level: 78
  slots_of_30: 0.2
- monster: '[[monsters/472-tirwin|Tirwin]]'
  level: 86
  slots_of_30: 0.2
- monster: '[[monsters/481-tirwin|Tirwin]]'
  level: 82
  slots_of_30: 0.2
- monster: '[[monsters/482-tirwin|Tirwin]]'
  level: 88
  slots_of_30: 0.2
used_to_craft:
- '[[items/weapon/437-viking-sword-axe|Viking Sword & Axe]]'
source:
  data: LIST_WEAPON.STB row 11
  code: module/src/items.rs
---
# Viking Sword

A sword with a wide blade that was popularized by the Vikings.
