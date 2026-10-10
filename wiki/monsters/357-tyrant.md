---
kind: monster
id: 357
name: Tyrant
status: in-game
level: 112
hp: 18592
hp_per_level: 166
attack: 541
hit: 336
defence: 375
resistance: 372
avoid: 171
attack_speed: 105
attack_range: 3
damage: physical
walk_speed: 230
run_speed: 570
xp: 497
drop_item_rate: 80
drop_money_rate: 0
zones: 1
drops:
- item: '[[items/material/194-ice-shard|Ice Shard]]'
  slots_of_30: 6
- item: '[[items/material/265-frozen-scales|Frozen Scales]]'
  slots_of_30: 7
- item: '[[items/material/120-setil|Setil]]'
  slots_of_30: 1
- item: '[[items/material/125-light-stone|Light Stone]]'
  slots_of_30: 1
- item: '[[items/material/144-ice-key-fragment|Ice Key Fragment]]'
  slots_of_30: 3
- item: '[[items/material/142-coin-of-eucar-tribe|Coin of Eucar Tribe]]'
  slots_of_30: 1
- item: '[[items/weapon/43-scorpion-club|Scorpion Club]]'
  slots_of_30: 0.2
- item: '[[items/weapon/113-dragon-sword|Dragon Sword]]'
  slots_of_30: 0.2
- item: '[[items/weapon/440-ice-flare-swords|Ice & Flare Swords]]'
  slots_of_30: 0.2
- item: '[[items/weapon/13-katana|Katana]]'
  slots_of_30: 0.2
- item: '[[items/weapon/68-anima-bow-gun|Anima Bow Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/112-flamberge|Flamberge]]'
  slots_of_30: 0.4
- item: '[[items/weapon/142-flame-axe|Flame Axe]]'
  slots_of_30: 0.4
- item: '[[items/weapon/173-gungnir|Gungnir]]'
  slots_of_30: 0.4
- item: '[[items/weapon/214-bow-of-wind-spirit|Bow of Wind Spirit]]'
  slots_of_30: 0.4
- item: '[[items/weapon/242-rp-911|RP-911]]'
  slots_of_30: 0.4
- item: '[[items/hands/65-gloves-of-shadow|Gloves of Shadow]]'
  slots_of_30: 0.6
- item: '[[items/feet/95-mariner-s-boots|Mariner''s Boots]]'
  slots_of_30: 1.2
- item: '[[items/body/35-mighty-armor|Mighty Armor]]'
  slots_of_30: 0.6
- item: '[[items/head/125-merchant-hat|Merchant Hat]]'
  slots_of_30: 0.6
spawns:
- zone: '[[zones/52-mana-snowfields|Mana Snowfields]]'
  x: 5039.9
  y: 4970.5
  count: 1
  group: basic
- zone: '[[zones/52-mana-snowfields|Mana Snowfields]]'
  x: 5264.2
  y: 4940.9
  count: 1
  group: reinforcements
- zone: '[[zones/52-mana-snowfields|Mana Snowfields]]'
  x: 5414
  y: 4700.7
  count: 1
  group: reinforcements
- zone: '[[zones/52-mana-snowfields|Mana Snowfields]]'
  x: 5580.5
  y: 4744.8
  count: 1
  group: reinforcements
- zone: '[[zones/52-mana-snowfields|Mana Snowfields]]'
  x: 5475.1
  y: 4658.6
  count: 1
  group: basic
- zone: '[[zones/52-mana-snowfields|Mana Snowfields]]'
  x: 5613.6
  y: 4798.6
  count: 1
  group: reinforcements
- zone: '[[zones/52-mana-snowfields|Mana Snowfields]]'
  x: 5516.4
  y: 4506.1
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 357, ITEM_DROP.STB row 371
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Tyrant

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
