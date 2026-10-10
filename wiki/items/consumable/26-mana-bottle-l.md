---
kind: item
id: consumable/26
name: Mana Bottle (L)
status: in-game
icon: item/1592
class: Medicine
effect: MP +700, MP Restoration (700), teaches skill 700, uses skill 700, fuel +700
price: 1190
weight: 1
dropped_by:
- monster: '[[monsters/142-kaiman-guard|Kaiman Guard]]'
  level: 60
  slots_of_30: 1
- monster: '[[monsters/143-kaiman-warrior|Kaiman Warrior]]'
  level: 64
  slots_of_30: 1
- monster: '[[monsters/208-guardian-tree|Guardian Tree]]'
  level: 57
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
  data: LIST_USEITEM.STB row 26
  code: module/src/items.rs
---
# Mana Bottle (L)

A potion that restores MP at a slightly rapid rate.
