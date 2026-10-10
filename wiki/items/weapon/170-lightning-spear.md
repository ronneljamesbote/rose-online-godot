---
kind: item
id: weapon/170
name: Lightning Spear
status: in-game
icon: item/1088
class: Spear
level: 82
attack: 123
attack_speed: 10
range: 4
damage: physical
two_handed: true
durability: 51
quality: 55
needs: Strength 142
sockets: true
price: 67850
weight: 30
crafted_with: '[[skills/2451-mace-craft|Mace Craft]] level 5'
recipe:
- material: any Metal
  quantity: 88
- material: '[[items/material/34-oak-wood|Oak Wood]]'
  quantity: 24
- material: '[[items/material/102-sharp-iron-piece|Sharp Iron Piece]]'
  quantity: 7
- material: '[[items/material/126-lesian|Lesian]]'
  quantity: 2
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
  slots_of_30: 0.4
- monster: '[[monsters/340-shadow-wolf|Shadow Wolf]]'
  level: 97
  slots_of_30: 0.2
- monster: '[[monsters/342-yeti-hunter|Yeti Hunter]]'
  level: 95
  slots_of_30: 0.4
- monster: '[[monsters/390-wolf-keeper|Wolf Keeper]]'
  level: 117
  slots_of_30: 0.2
- monster: '[[monsters/452-seal-stone|Seal Stone]]'
  level: 88
  slots_of_30: 1
source:
  data: LIST_WEAPON.STB row 170
  code: module/src/items.rs
---
# Lightning Spear

An electrical Spear that can summon thunder. Be careful lest it shock you.
