---
kind: monster
id: 169
name: Grandmaster Golem
status: in-game
level: 90
hp: 19800
hp_per_level: 220
attack: 429
hit: 260
defence: 350
resistance: 256
avoid: 112
attack_speed: 100
attack_range: 2.5
damage: physical
walk_speed: 350
run_speed: 750
xp: 827
drop_item_rate: 83
drop_money_rate: 1
zones: 1
drops:
- item: '[[items/material/201-steam-oil|Steam Oil]]'
  slots_of_30: 1
- item: '[[items/consumable/173-stamina-100|Stamina (+100)]]'
  slots_of_30: 1
- item: '[[items/material/202-steam-heat|Steam Heat]]'
  slots_of_30: 1
- item: '[[items/consumable/165-mp-point-300|MP Point (+300)]]'
  slots_of_30: 1
- item: '[[items/material/163-red-crystal|Red Crystal]]'
  slots_of_30: 2
- item: '[[items/material/203-steam-viper|Steam Viper]]'
  slots_of_30: 2
- item: '[[items/consumable/155-hp-point-500|HP Point (+500)]]'
  slots_of_30: 1
- item: '[[items/material/204-mana-metal|Mana Metal]]'
  slots_of_30: 1
- item: '[[items/material/205-mana-oil|Mana Oil]]'
  slots_of_30: 1
- item: '[[items/consumable/58-vital-jam-5|Vital Jam (+5)]]'
  slots_of_30: 2
- item: '[[items/material/206-mana-heat|Mana Heat]]'
  slots_of_30: 1
- item: '[[items/material/207-mana-generator|Mana Generator]]'
  slots_of_30: 1
- item: '[[items/material/208-mana-crystal|Mana Crystal]]'
  slots_of_30: 2
- item: '[[items/jewellery/271-socket-earring|Socket Earring]]'
  slots_of_30: 1
- item: '[[items/material/209-mana-article|Mana Article]]'
  slots_of_30: 1
- item: '[[items/consumable/314-advanced-mp-scroll-solo|Advanced MP Scroll (Solo)]]'
  slots_of_30: 1
- item: '[[items/body/34-chrome-armor|Chrome Armor]]'
  slots_of_30: 0.4
- item: '[[items/hands/64-violet-gloves|Violet Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/94-black-pirate-boots|Black Pirate Boots]]'
  slots_of_30: 0.4
- item: '[[items/head/124-semiya-goggles|Semiya Goggles]]'
  slots_of_30: 0.4
- item: '[[items/weapon/66-dark-bow-gun|Dark Bow Gun]]'
  slots_of_30: 0.4
- item: '[[items/weapon/170-lightning-spear|Lightning Spear]]'
  slots_of_30: 0.4
- item: '[[items/weapon/437-viking-sword-axe|Viking Sword & Axe]]'
  slots_of_30: 0.4
- item: '[[items/weapon/339-blizzard-wand|Blizzard Wand]]'
  slots_of_30: 0.4
- item: '[[items/weapon/267-mythril-launcher|Mythril Launcher]]'
  slots_of_30: 0.4
- item: '[[items/weapon/138-silver-axe|Silver Axe]]'
  slots_of_30: 0.4
- item: '[[items/head/35-mighty-helm|Mighty Helm]]'
  slots_of_30: 0.4
- item: '[[items/body/65-coat-of-shadow|Coat of Shadow]]'
  slots_of_30: 0.4
- item: '[[items/hands/95-mariner-s-gloves|Mariner''s Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/125-merchant-boots|Merchant Boots]]'
  slots_of_30: 0.4
- item: '[[items/back/247-buffalo-backshield|Buffalo Backshield]]'
  slots_of_30: 0.4
- item: '[[items/weapon/767-anima-staff|Anima Staff]]'
  slots_of_30: 0.4
- item: '[[items/weapon/508-viking-sword|Viking Sword]]'
  slots_of_30: 0.4
- item: '[[items/weapon/828-dual-patar|Dual Patar]]'
  slots_of_30: 0.8
- item: '[[items/weapon/796-blizzard-wand|Blizzard Wand]]'
  slots_of_30: 0.4
- item: '[[items/gem/351-peridot-1|Peridot 1]]'
  slots_of_30: 1
quests:
- '[[quests/157-skin-of-steel|Skin of Steel]]'
spawns:
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5269.1
  y: 5438.6
  count: 1
  group: basic
- zone: '[[zones/28-gorge-of-silence|Gorge of Silence]]'
  x: 5303.5
  y: 5437.6
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 169, ITEM_DROP.STB row 185
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Grandmaster Golem

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
