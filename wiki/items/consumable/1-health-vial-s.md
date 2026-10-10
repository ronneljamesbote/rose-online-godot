---
kind: item
id: consumable/1
name: Health Vial (S)
status: in-game
icon: item/1586
class: Medicine
effect: HP +200, HP Restoration (200), teaches skill 200, uses skill 200, fuel +200
price: 100
weight: 1
dropped_by:
- monster: '[[monsters/2-jelly-bean|Jelly Bean]]'
  level: 4
  slots_of_30: 1
- monster: '[[monsters/3-jellynut|JellyNut]]'
  level: 5
  slots_of_30: 1
- monster: '[[monsters/23-elder-pumpkin|Elder Pumpkin]]'
  level: 13
  slots_of_30: 1
- monster: '[[monsters/24-cowboy-pumpkin|Cowboy Pumpkin]]'
  level: 14
  slots_of_30: 1
- monster: '[[monsters/63-needle-hornet|Needle Hornet]]'
  level: 15
  slots_of_30: 1
- monster: '[[monsters/91-honey-rackie|Honey Rackie]]'
  level: 24
  slots_of_30: 1
- monster: '[[monsters/256-needle-bat|Needle Bat]]'
  level: 46
  slots_of_30: 1
sold_by:
- '[[npcs/1006-arumic-merchant-tryteh|Arumic Merchant Tryteh]]'
- '[[npcs/1091-arumic-merchant-chester|Arumic Merchant Chester]]'
- '[[npcs/1151-ferrell-guild-merchant-med|Ferrell Guild Merchant Med]]'
- '[[npcs/1161-clan-base-camp-manager-kushard|Clan Base Camp Manager Kushard]]'
- '[[npcs/1162-clan-base-camp-soldier-jason|Clan Base Camp Soldier Jason]]'
- '[[npcs/1212-arumic-researcher-catherine-clara|Arumic Researcher Catherine Clara]]'
quest_reward:
- '[[quests/852-solider-job-change-quest|Solider Job Change Quest]]'
- '[[quests/853-solider-job-change-quest|Solider Job Change Quest]]'
- '[[quests/902-muse-job-change-quest|Muse Job Change Quest]]'
- '[[quests/903-muse-job-change-quest|Muse Job Change Quest]]'
- '[[quests/1002-dealer-job-change-quest|Dealer Job Change Quest]]'
- '[[quests/1003-dealer-job-change-quest|Dealer Job Change Quest]]'
source:
  data: LIST_USEITEM.STB row 1
  code: module/src/items.rs
---
# Health Vial (S)

Potion that slowly restores HP.
