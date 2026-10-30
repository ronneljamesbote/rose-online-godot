---
kind: npc
id: 1065
name: '[Quartermaster] Hask'
status: in-game
zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
services: shop
locations:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5453
  y: 5300
shop:
- tab: Potion/Return Scroll
  item: '[[items/consumable/1-health-vial-s|Health Vial (S)]]'
  base_price: 100
- tab: Potion/Return Scroll
  item: '[[items/consumable/2-health-vial-m|Health Vial (M)]]'
  base_price: 220
- tab: Potion/Return Scroll
  item: '[[items/consumable/3-health-vial-l|Health Vial (L)]]'
  base_price: 480
- tab: Potion/Return Scroll
  item: '[[items/consumable/4-health-bottle-s|Health Bottle (S)]]'
  base_price: 270
- tab: Potion/Return Scroll
  item: '[[items/consumable/5-health-bottle-m|Health Bottle (M)]]'
  base_price: 570
- tab: Potion/Return Scroll
  item: '[[items/consumable/6-health-bottle-l|Health Bottle (L)]]'
  base_price: 1000
- tab: Potion/Return Scroll
  item: '[[items/consumable/7-herbal-medicine-s|Herbal Medicine (S)]]'
  base_price: 175
- tab: Potion/Return Scroll
  item: '[[items/consumable/8-herbal-medicine-m|Herbal Medicine (M)]]'
  base_price: 263
- tab: Potion/Return Scroll
  item: '[[items/consumable/21-mana-vial-s|Mana Vial (S)]]'
  base_price: 150
- tab: Potion/Return Scroll
  item: '[[items/consumable/22-mana-vial-m|Mana Vial (M)]]'
  base_price: 330
- tab: Potion/Return Scroll
  item: '[[items/consumable/23-mana-vial-l|Mana Vial (L)]]'
  base_price: 600
- tab: Potion/Return Scroll
  item: '[[items/consumable/24-mana-bottle-s|Mana Bottle (S)]]'
  base_price: 300
- tab: Potion/Return Scroll
  item: '[[items/consumable/25-mana-bottle-m|Mana Bottle (M)]]'
  base_price: 640
- tab: Potion/Return Scroll
  item: '[[items/consumable/26-mana-bottle-l|Mana Bottle (L)]]'
  base_price: 1190
- tab: Potion/Return Scroll
  item: '[[items/consumable/352-junon-polis-return-scroll|Junon Polis Return Scroll]]'
  base_price: 1200
- tab: Repair Tool
  item: '[[items/consumable/291-repair-hammer|Repair Hammer]]'
  base_price: 300
- tab: Magic Weapon
  item: '[[items/weapon/304-mage-s-rod|Mage''s Rod]]'
  base_price: 11080
- tab: Magic Weapon
  item: '[[items/weapon/305-white-staff|White Staff]]'
  base_price: 19035
- tab: Magic Weapon
  item: '[[items/weapon/306-wisp-staff|Wisp Staff]]'
  base_price: 30850
- tab: Magic Weapon
  item: '[[items/weapon/307-shadow-staff|Shadow Staff]]'
  base_price: 43000
- tab: Magic Weapon
  item: '[[items/weapon/308-golden-staff|Golden Staff]]'
  base_price: 60400
- tab: Magic Weapon
  item: '[[items/weapon/334-elven-wand|Elven Wand]]'
  base_price: 9880
- tab: Magic Weapon
  item: '[[items/weapon/335-silence-wand|Silence Wand]]'
  base_price: 16785
- tab: Magic Weapon
  item: '[[items/weapon/336-windphon-wand|Windphon Wand]]'
  base_price: 29000
- tab: Magic Weapon
  item: '[[items/weapon/337-flame-wand|Flame Wand]]'
  base_price: 43900
- tab: Magic Weapon
  item: '[[items/weapon/338-thunder-wand|Thunder Wand]]'
  base_price: 59600
- tab: Magic Weapon
  item: '[[items/subweapon/61-book-of-standards|Book of Standards]]'
  base_price: 1050
- tab: Magic Weapon
  item: '[[items/subweapon/62-oriental-drum|Oriental Drum]]'
  base_price: 7050
- tab: Magic Weapon
  item: '[[items/subweapon/63-book-of-concentration|Book of Concentration]]'
  base_price: 9000
- tab: Magic Weapon
  item: '[[items/subweapon/64-book-of-charm|Book of Charm]]'
  base_price: 14100
- tab: Magic Weapon
  item: '[[items/subweapon/65-mythical-flute|Mythical Flute]]'
  base_price: 8640
- tab: Magic Weapon
  item: '[[items/subweapon/66-elven-anthology|Elven Anthology]]'
  base_price: 10400
- tab: Magic Weapon
  item: '[[items/subweapon/67-basic-encyclopedia|Basic Encyclopedia]]'
  base_price: 14150
- tab: Magic Weapon
  item: '[[items/subweapon/68-magic-mirror|Magic Mirror]]'
  base_price: 17800
source:
  data: godot/data/npcs/1065-quartermaster-hask.tres; dialog godot/data/dialogs/em21-011-quartermaster-hask.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Quartermaster] Hask

Stands at the Arumic Expedition Camp in [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]. Sells potions, return scrolls, repair tools and magic weapons (rods, staffs and wands) (see [[rules/shops|Shops]]).

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Supplies for the expedition. If you're working for Ardeth, I'll sell you some.
  - **You:** Let me see your goods.  → opens the shop
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Valley of Luxem Tower redesign, using an existing iROSE model.
