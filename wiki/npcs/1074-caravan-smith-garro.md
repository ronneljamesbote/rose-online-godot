---
kind: npc
id: 1074
name: '[Caravan Smith] Garro'
status: in-game
zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
services: shop
locations:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5189
  y: 5070
shop:
- tab: One-Handed Weapons
  item: '[[items/weapon/2-short-sword|Short Sword]]'
  base_price: 420
- tab: One-Handed Weapons
  item: '[[items/weapon/3-rapier|Rapier]]'
  base_price: 850
- tab: One-Handed Weapons
  item: '[[items/weapon/4-khukuri|Khukuri]]'
  base_price: 1890
- tab: One-Handed Weapons
  item: '[[items/weapon/5-long-sword|Long Sword]]'
  base_price: 3570
- tab: One-Handed Weapons
  item: '[[items/weapon/6-bushido|Bushido]]'
  base_price: 6680
- tab: One-Handed Weapons
  item: '[[items/weapon/31-wooden-bat|Wooden Bat]]'
  base_price: 580
- tab: One-Handed Weapons
  item: '[[items/weapon/32-monkey-wrench|Monkey Wrench]]'
  base_price: 1025
- tab: One-Handed Weapons
  item: '[[items/weapon/33-buffoon-mace| Buffoon Mace]]'
  base_price: 2220
- tab: One-Handed Weapons
  item: '[[items/weapon/34-ogre-mace|Ogre Mace]]'
  base_price: 4445
- tab: One-Handed Weapons
  item: '[[items/weapon/35-onion-mace|Onion Mace]]'
  base_price: 7920
- tab: One-Handed Weapons
  item: '[[items/subweapon/1-wooden-shield|Wooden Shield]]'
  base_price: 200
- tab: One-Handed Weapons
  item: '[[items/subweapon/2-tarz|Tarz]]'
  base_price: 750
- tab: One-Handed Weapons
  item: '[[items/subweapon/3-round-shield|Round Shield]]'
  base_price: 1350
- tab: One-Handed Weapons
  item: '[[items/subweapon/4-buckler|Buckler]]'
  base_price: 2400
- tab: Two-Handed Weapons
  item: '[[items/weapon/101-haedong-sword|Haedong Sword]]'
  base_price: 1275
- tab: Two-Handed Weapons
  item: '[[items/weapon/102-nameless-sword|Nameless Sword]]'
  base_price: 2820
- tab: Two-Handed Weapons
  item: '[[items/weapon/103-sword-of-hardship|Sword of Hardship]]'
  base_price: 5075
- tab: Two-Handed Weapons
  item: '[[items/weapon/104-cutter-edge|Cutter Edge]]'
  base_price: 8640
- tab: Two-Handed Weapons
  item: '[[items/weapon/131-woodman-axe|Woodman Axe]]'
  base_price: 3150
- tab: Two-Handed Weapons
  item: '[[items/weapon/132-small-axe|Small Axe]]'
  base_price: 5840
- tab: Two-Handed Weapons
  item: '[[items/weapon/133-battle-axe|Battle Axe]]'
  base_price: 10485
- tab: Two-Handed Weapons
  item: '[[items/weapon/161-bamboo-spear|Bamboo Spear]]'
  base_price: 380
- tab: Two-Handed Weapons
  item: '[[items/weapon/162-javelin|Javelin]]'
  base_price: 1425
- tab: Two-Handed Weapons
  item: '[[items/weapon/163-round-spear|Round Spear]]'
  base_price: 2910
- tab: Two-Handed Weapons
  item: '[[items/weapon/164-scimitar|Scimitar]]'
  base_price: 5600
- tab: Two-Handed Weapons
  item: '[[items/weapon/165-bardiche|Bardiche]]'
  base_price: 9840
- tab: Bows
  item: '[[items/weapon/201-toy-bow|Toy Bow]]'
  base_price: 460
- tab: Bows
  item: '[[items/weapon/202-short-bow|Short Bow]]'
  base_price: 1175
- tab: Bows
  item: '[[items/weapon/203-long-bow|Long Bow]]'
  base_price: 2460
- tab: Bows
  item: '[[items/weapon/204-orc-bow|Orc Bow]]'
  base_price: 4480
- tab: Bows
  item: '[[items/weapon/205-white-wing-bow|White Wing Bow]]'
  base_price: 8040
- tab: Bows
  item: '[[items/weapon/61-simple-bow-gun|Simple Bow Gun]]'
  base_price: 8155
- tab: Bows
  item: '[[items/material/301-wooden-arrow|Wooden Arrow]]'
  base_price: 1
- tab: Bows
  item: '[[items/material/302-bone-arrow|Bone Arrow]]'
  base_price: 2
- tab: Bows
  item: '[[items/material/303-stone-arrow|Stone Arrow]]'
  base_price: 3
- tab: Bows
  item: '[[items/material/304-claw-arrow|Claw Arrow]]'
  base_price: 4
- tab: Bows
  item: '[[items/material/311-fire-arrow|Fire Arrow]]'
  base_price: 3
- tab: Bows
  item: '[[items/material/312-white-night-arrow|White Night Arrow]]'
  base_price: 6
- tab: Combat Weapons
  item: '[[items/weapon/401-knuckle|Knuckle]]'
  base_price: 1150
- tab: Combat Weapons
  item: '[[items/weapon/402-sword-knuckle| Sword Knuckle]]'
  base_price: 2280
- tab: Combat Weapons
  item: '[[items/weapon/403-katar|Katar]]'
  base_price: 4480
- tab: Combat Weapons
  item: '[[items/weapon/404-rake-hand|Rake Hand]]'
  base_price: 7640
- tab: Combat Weapons
  item: '[[items/weapon/431-khukuri-long-sword|Khukuri & Long Sword]]'
  base_price: 5845
- tab: Combat Weapons
  item: '[[items/weapon/432-long-sword-mace|Long Sword & Mace]]'
  base_price: 10240
source:
  data: godot/data/npcs/1074-caravan-smith-garro.tres; dialog godot/data/dialogs/em21-017-caravan-smith-garro.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Caravan Smith] Garro

Stands at the Sandwind Caravan Camp in [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]. Sells one-handed and two-handed weapons, bows and combat weapons (see [[rules/shops|Shops]]).

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Every wagon needs a blade or two on board. Have a look, they're all sharp.
  - **You:** Let me see your goods.  → opens the shop
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Valley of Luxem Tower redesign, using an existing iROSE model.
