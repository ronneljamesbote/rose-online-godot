---
kind: item
id: consumable/101
name: Apple
status: in-game
icon: item/1625
class: Food
effect: HP +100, HP Restoration (100), teaches skill 100, uses skill 100, fuel +100
price: 30
weight: 1
dropped_by:
- monster: '[[monsters/71-needle-pomic|Needle Pomic]]'
  level: 12
  slots_of_30: 1
- monster: '[[monsters/74-pomic-soldier|Pomic Soldier]]'
  level: 17
  slots_of_30: 1
- monster: '[[monsters/85-turtle-guard|Turtle Guard]]'
  level: 29
  slots_of_30: 1
dropped_in_zones:
- '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
- '[[zones/20-birth-island|Birth Island]]'
sold_by:
- '[[npcs/1013-tavern-owner-sharlin|Tavern Owner Sharlin]]'
- '[[npcs/1030-visitor-guide-fairy-of-arua|Visitor Guide Fairy of Arua]]'
- '[[npcs/1035-little-street-vendor-pony|Little Street Vendor Pony]]'
- '[[npcs/1063-little-street-vendor-mile|Little Street Vendor Mile]]'
source:
  data: LIST_USEITEM.STB row 101
  code: module/src/items.rs
---
# Apple

A ripe apple.
