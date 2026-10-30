---
kind: npc
id: 1075
name: '[Caravan Outfitter] Nessa'
status: in-game
zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
services: shop
locations:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5217
  y: 5062
shop:
- tab: Soldier
  item: '[[items/body/31-soldier-armor|Soldier Armor]]'
  base_price: 2450
- tab: Soldier
  item: '[[items/body/32-iron-armor|Iron Armor]]'
  base_price: 7200
- tab: Soldier
  item: '[[items/head/31-soldier-helm|Soldier Helm]]'
  base_price: 1100
- tab: Soldier
  item: '[[items/head/32-iron-helm|Iron Helm]]'
  base_price: 2400
- tab: Soldier
  item: '[[items/hands/31-soldier-gloves|Soldier Gloves]]'
  base_price: 650
- tab: Soldier
  item: '[[items/hands/32-iron-gauntlets|Iron Gauntlets]]'
  base_price: 1950
- tab: Soldier
  item: '[[items/feet/31-soldier-boots|Soldier Boots]]'
  base_price: 1050
- tab: Soldier
  item: '[[items/feet/32-iron-boots|Iron Boots]]'
  base_price: 2950
- tab: Hawker
  item: '[[items/body/91-hunter-chest|Hunter Chest]]'
  base_price: 3150
- tab: Hawker
  item: '[[items/body/92-criker-chest|Criker Chest]]'
  base_price: 8600
- tab: Hawker
  item: '[[items/head/91-hunter-hat|Hunter Hat]]'
  base_price: 1350
- tab: Hawker
  item: '[[items/head/92-criker-hat|Criker Hat]]'
  base_price: 3100
- tab: Hawker
  item: '[[items/hands/91-hunter-gloves|Hunter Gloves]]'
  base_price: 750
- tab: Hawker
  item: '[[items/hands/92-criker-gloves|Criker Gloves]]'
  base_price: 1800
- tab: Hawker
  item: '[[items/feet/91-hunter-boots|Hunter Boots]]'
  base_price: 1050
- tab: Hawker
  item: '[[items/feet/92-criker-boots|Criker Boots]]'
  base_price: 2750
- tab: Muse
  item: '[[items/body/61-spiritual-vest|Spiritual Vest]]'
  base_price: 3300
- tab: Muse
  item: '[[items/body/62-green-vest-of-witch|Green Vest of Witch]]'
  base_price: 9450
- tab: Muse
  item: '[[items/head/61-spiritual-beret|Spiritual Beret ]]'
  base_price: 950
- tab: Muse
  item: '[[items/head/62-green-hat-of-witch|Green Hat of Witch]]'
  base_price: 2950
- tab: Muse
  item: '[[items/hands/61-spiritual-gloves|Spiritual Gloves]]'
  base_price: 850
- tab: Muse
  item: '[[items/hands/62-green-gloves-of-witch|Green Gloves of Witch]]'
  base_price: 1900
- tab: Muse
  item: '[[items/feet/61-spiritual-shoes|Spiritual Shoes]]'
  base_price: 900
- tab: Muse
  item: '[[items/feet/62-green-sandals-of-witch|Green Sandals of Witch]]'
  base_price: 2500
- tab: Dealer
  item: '[[items/body/121-brown-vest|Brown Vest]]'
  base_price: 3650
- tab: Dealer
  item: '[[items/body/122-vibe-vest|Vibe Vest]]'
  base_price: 9500
- tab: Dealer
  item: '[[items/head/121-brown-turban|Brown Turban ]]'
  base_price: 1100
- tab: Dealer
  item: '[[items/head/122-vibe-turban|Vibe Turban]]'
  base_price: 2650
- tab: Dealer
  item: '[[items/hands/121-brown-gloves|Brown Gloves]]'
  base_price: 900
- tab: Dealer
  item: '[[items/hands/122-vibe-gloves|Vibe Gloves]]'
  base_price: 2050
- tab: Dealer
  item: '[[items/feet/121-brown-shoes|Brown Shoes]]'
  base_price: 1300
- tab: Dealer
  item: '[[items/feet/122-vibe-shoes|Vibe Shoes]]'
  base_price: 2750
source:
  data: godot/data/npcs/1075-caravan-outfitter-nessa.tres; dialog godot/data/dialogs/em21-018-caravan-outfitter-nessa.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Caravan Outfitter] Nessa

Stands at the Sandwind Caravan Camp in [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]. Sells armour and clothes for soldiers, hawkers, muses and dealers, (see [[rules/shops|Shops]]).

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Sand gets into everything. Good leather keeps most of it out.
  - **You:** Let me see your goods.  → opens the shop
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Valley of Luxem Tower redesign, using an existing iROSE model.
