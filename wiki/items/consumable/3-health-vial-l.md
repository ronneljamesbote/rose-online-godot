---
kind: item
id: consumable/3
name: Health Vial (L)
status: in-game
icon: item/1586
class: Medicine
effect: HP +800, HP Restoration (800), teaches skill 800, uses skill 800, fuel +800
price: 480
weight: 1
dropped_by:
- monster: '[[monsters/163-master-golem|Master Golem]]'
  level: 84
  slots_of_30: 1
- monster: '[[monsters/165-elder-stone-golem|Elder Stone Golem]]'
  level: 76
  slots_of_30: 1
- monster: '[[monsters/258-elder-bloodbat|Elder BloodBat]]'
  level: 60
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
- '[[quests/131-eva-the-sorcerer|Eva the Sorcerer]]'
- '[[quests/132-forbidden-spell|Forbidden Spell]]'
- '[[quests/3006-in-readiness-for-war|In Readiness for War]]'
- '[[quests/3007-the-arumics-request|The Arumics'' Request]]'
- '[[quests/3406-majesty-of-the-righteous-crusaders|Majesty of the Righteous Crusaders]]'
- '[[quests/3606-magic-weapon-production|Magic Weapon Production]]'
- '[[quests/3806-reassuring-support|Reassuring Support]]'
- '[[quests/3807-new-recovery-potion|New Recovery Potion]]'
source:
  data: LIST_USEITEM.STB row 3
  code: module/src/items.rs
---
# Health Vial (L)

Potion that slowly restores HP.
