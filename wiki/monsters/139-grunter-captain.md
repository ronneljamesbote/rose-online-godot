---
kind: monster
id: 139
name: Grunter Captain
status: in-game
level: 55
hp: 2475
hp_per_level: 45
attack: 232
hit: 158
defence: 176
resistance: 114
avoid: 67
attack_speed: 100
attack_range: 3
damage: physical
walk_speed: 230
run_speed: 520
xp: 73
drop_item_rate: 75
drop_money_rate: 35
zones: 2
drops:
- item: '[[items/material/186-red-animal-tail-fur|Red Animal Tail Fur]]'
  slots_of_30: 4
- item: '[[items/consumable/172-stamina-75|Stamina (+75)]]'
  slots_of_30: 1
- item: '[[items/consumable/164-mp-point-200|MP Point (+200)]]'
  slots_of_30: 2
- item: '[[items/consumable/25-mana-bottle-m|Mana Bottle (M)]]'
  slots_of_30: 1
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 5
- item: '[[items/material/196-bird-feather|Bird Feather]]'
  slots_of_30: 5
- item: '[[items/jewellery/261-socket-necklace|Socket Necklace]]'
  slots_of_30: 1
- item: '[[items/consumable/419-grunter-leader|Grunter Leader]]'
  slots_of_30: 1
- item: '[[items/material/189-predator-claw|Predator Claw]]'
  slots_of_30: 1
- item: '[[items/jewellery/152-light-earring|Light Earring]]'
  slots_of_30: 1
- item: '[[items/hands/63-purple-gloves-of-witch|Purple Gloves of Witch]]'
  slots_of_30: 0.2
- item: '[[items/feet/93-ranger-boots|Ranger Boots]]'
  slots_of_30: 0.2
- item: '[[items/weapon/36-ghost-bat|Ghost Bat]]'
  slots_of_30: 0.6
- item: '[[items/weapon/134-tomahawk|Tomahawk]]'
  slots_of_30: 0.4
- item: '[[items/weapon/263-hard-launcher|Hard Launcher]]'
  slots_of_30: 0.2
- item: '[[items/weapon/165-bardiche|Bardiche]]'
  slots_of_30: 0.2
- item: '[[items/weapon/305-white-staff|White Staff]]'
  slots_of_30: 0.2
- item: '[[items/weapon/405-wolf-s-paw|Wolf''s Paw]]'
  slots_of_30: 0.2
- item: '[[items/hands/33-trunket-gloves|Trunket Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/63-purple-sandals-of-witch|Purple Sandals of Witch]]'
  slots_of_30: 0.4
- item: '[[items/head/93-ranger-hat|Ranger Hat]]'
  slots_of_30: 0.4
- item: '[[items/body/123-tamiya-vest|Tamiya Vest]]'
  slots_of_30: 0.4
- item: '[[items/subweapon/5-aspis|Aspis]]'
  slots_of_30: 0.4
- item: '[[items/head/417-trunket-helm|Trunket Helm]]'
  slots_of_30: 0.2
- item: '[[items/body/417-purple-vest-of-witch|Purple Vest of Witch]]'
  slots_of_30: 0.2
- item: '[[items/hands/517-ranger-gloves|Ranger Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/617-tamiya-shoes|Tamiya Shoes]]'
  slots_of_30: 0.2
spawns:
- zone: '[[zones/2-city-of-junon-polis|City of Junon Polis]]'
  x: 5116.6
  y: 5456.4
  count: 1
  group: reinforcements
- zone: '[[zones/2-city-of-junon-polis|City of Junon Polis]]'
  x: 5082.5
  y: 5498.2
  count: 1
  group: reinforcements
- zone: '[[zones/2-city-of-junon-polis|City of Junon Polis]]'
  x: 5035
  y: 5481.9
  count: 2
  group: reinforcements
- zone: '[[zones/2-city-of-junon-polis|City of Junon Polis]]'
  x: 5274.3
  y: 5514.1
  count: 1
  group: reinforcements
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5237.1
  y: 5424.5
  count: 1
  group: reinforcements
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5294
  y: 5334
  count: 1
  group: reinforcements
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5418.3
  y: 5185.6
  count: 1
  group: reinforcements
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5465.4
  y: 5171.9
  count: 1
  group: reinforcements
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5379.6
  y: 4966.9
  count: 2
  group: reinforcements
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5588
  y: 5014.1
  count: 1
  group: reinforcements
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5614.2
  y: 5002.6
  count: 1
  group: reinforcements
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5614.8
  y: 5044.5
  count: 2
  group: reinforcements
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5611.6
  y: 5020.2
  count: 2
  group: reinforcements
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5397.1
  y: 4908.8
  count: 2
  group: reinforcements
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5470.7
  y: 4898.9
  count: 2
  group: reinforcements
source:
  data: LIST_NPC.STB row 139, ITEM_DROP.STB row 164
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Grunter Captain

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
