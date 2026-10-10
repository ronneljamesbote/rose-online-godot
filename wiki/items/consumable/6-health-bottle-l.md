---
kind: item
id: consumable/6
name: Health Bottle (L)
status: in-game
icon: item/1587
class: Medicine
effect: HP +1000, HP Restoration (1000), teaches [[skills/991-enchant-armor|Enchant Armor]] level 10, uses [[skills/991-enchant-armor|Enchant Armor]] level 10, fuel +1000
price: 1000
weight: 1
dropped_by:
- monster: '[[monsters/145-kaiman-hunter|Kaiman Hunter]]'
  level: 63
  slots_of_30: 1
- monster: '[[monsters/271-goblin-worker|Goblin Worker]]'
  level: 51
  slots_of_30: 1
sold_by:
- '[[npcs/1006-arumic-merchant-tryteh|Arumic Merchant Tryteh]]'
- '[[npcs/1091-arumic-merchant-chester|Arumic Merchant Chester]]'
- '[[npcs/1151-ferrell-guild-merchant-med|Ferrell Guild Merchant Med]]'
- '[[npcs/1161-clan-base-camp-manager-kushard|Clan Base Camp Manager Kushard]]'
- '[[npcs/1162-clan-base-camp-soldier-jason|Clan Base Camp Soldier Jason]]'
- '[[npcs/1185-magic-goods-seller-pabel|Magic Goods Seller Pabel]]'
- '[[npcs/1212-arumic-researcher-catherine-clara|Arumic Researcher Catherine Clara]]'
quest_reward:
- '[[quests/151-lunarian-coffee|Lunarian Coffee]]'
source:
  data: LIST_USEITEM.STB row 6
  code: module/src/items.rs
---
# Health Bottle (L)

Medicine that restores HP at a slightly rapid rate.
