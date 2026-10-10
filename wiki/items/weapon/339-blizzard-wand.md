---
kind: item
id: weapon/339
name: Blizzard Wand
status: in-game
icon: item/1334
class: Magic Tool
level: 81
attack: 94
attack_speed: 12
range: 20
damage: magic
two_handed: false
durability: 51
quality: 56
needs: Intelligence 147
bonus: MP Consumption +12
sockets: true
price: 84450
weight: 10
crafted_with: '[[skills/2491-magic-weapon-craft|Magic Weapon Craft]] level 5'
recipe:
- material: any Wooden Material
  quantity: 120
- material: '[[items/material/4-bronze|Bronze]]'
  quantity: 60
- material: '[[items/material/61-low-essence|Low Essence]]'
  quantity: 15
- material: '[[items/material/126-lesian|Lesian]]'
  quantity: 15
craft_difficulty: 45
dropped_by:
- monster: '[[monsters/169-grandmaster-golem|Grandmaster Golem]]'
  level: 90
  slots_of_30: 0.4
- monster: '[[monsters/281-goblin-mage|Goblin Mage]]'
  level: 88
  slots_of_30: 0.2
- monster: '[[monsters/282-gem-goblin-mage|Gem Goblin Mage]]'
  level: 92
  slots_of_30: 0.2
- monster: '[[monsters/286-master-goblin|Master Goblin]]'
  level: 100
  slots_of_30: 0.4
- monster: '[[monsters/451-seal-stone|Seal Stone]]'
  level: 80
  slots_of_30: 1
- monster: '[[monsters/452-seal-stone|Seal Stone]]'
  level: 88
  slots_of_30: 1
source:
  data: LIST_WEAPON.STB row 339
  code: module/src/items.rs
---
# Blizzard Wand

A Mage's Wand that can magically shoot ice fragments.
