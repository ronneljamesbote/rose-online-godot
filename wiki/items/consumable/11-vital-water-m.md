---
kind: item
id: consumable/11
name: Vital Water (M)
status: in-game
icon: item/1589
class: Medicine
effect: HP +500, teaches [[skills/491-slow-shot|Slow Shot]] level 10, uses [[skills/491-slow-shot|Slow Shot]] level 10, fuel +500
price: 950
weight: 1
dropped_by:
- monster: '[[monsters/157-doonga-leader|Doonga Leader]]'
  level: 70
  slots_of_30: 1
- monster: '[[monsters/407-junon-s-doonga|Junon''s Doonga]]'
  level: 85
  slots_of_30: 1
dropped_in_zones:
- '[[zones/62-shady-jungle|Shady Jungle]]'
sold_by:
- '[[npcs/1185-magic-goods-seller-pabel|Magic Goods Seller Pabel]]'
quest_reward:
- '[[quests/141-the-prince-of-akram|The Prince of Akram]]'
- '[[quests/142-the-search-for-the-prince|The Search for the Prince]]'
- quest 1953
- '[[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]'
source:
  data: LIST_USEITEM.STB row 11
  code: module/src/items.rs
---
# Vital Water (M)

Mysterious water that rapidly restores HP.
