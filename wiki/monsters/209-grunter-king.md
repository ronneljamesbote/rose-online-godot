---
kind: monster
id: 209
name: Grunter King
status: in-game
level: 60
hp: 175
attack: 286
hit: 192
defence: 227
resistance: 161
avoid: 73
attack_speed: 95
attack_range: 3.5
damage: physical
walk_speed: 300
run_speed: 650
xp: 539
drop_item_rate: 82
drop_money_rate: 20
zones: 2
drops:
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 6
- item: '[[items/material/186-red-animal-tail-fur|Red Animal Tail Fur]]'
  slots_of_30: 4
- item: '[[items/material/161-green-crystal|Green Crystal]]'
  slots_of_30: 4
- item: '[[items/consumable/153-hp-point-200|HP Point (+200)]]'
  slots_of_30: 1
- item: '[[items/consumable/173-stamina-100|Stamina (+100)]]'
  slots_of_30: 2
- item: '[[items/consumable/58-vital-jam-5|Vital Jam (+5)]]'
  slots_of_30: 2
- item: '[[items/weapon/263-hard-launcher|Hard Launcher]]'
  slots_of_30: 0.4
- item: '[[items/weapon/207-iron-bow|Iron Bow]]'
  slots_of_30: 0.4
- item: '[[items/weapon/236-sorden-gun|Sorden Gun]]'
  slots_of_30: 0.4
- item: '[[items/weapon/208-maiya-bow|Maiya Bow]]'
  slots_of_30: 0.4
- item: '[[items/weapon/406-bear-s-paw|Bear''s Paw]]'
  slots_of_30: 0.4
- item: '[[items/head/123-tamiya-goggles|Tamiya Goggles]]'
  slots_of_30: 0.2
- item: '[[items/body/33-trunket-armor|Trunket Armor]]'
  slots_of_30: 0.2
- item: '[[items/hands/63-purple-gloves-of-witch|Purple Gloves of Witch]]'
  slots_of_30: 0.2
- item: '[[items/feet/93-ranger-boots|Ranger Boots]]'
  slots_of_30: 0.2
- item: '[[items/weapon/135-orc-axe|Orc Axe]]'
  slots_of_30: 0.2
- item: '[[items/weapon/614-orc-axe|Orc Axe]]'
  slots_of_30: 0.4
- item: '[[items/weapon/673-elf-bow|Elf Bow]]'
  slots_of_30: 0.4
- item: '[[items/weapon/704-sorden-gun|Sorden Gun]]'
  slots_of_30: 0.4
- item: '[[items/weapon/534-stone-hammer|Stone Hammer]]'
  slots_of_30: 0.4
- item: '[[items/weapon/731-hard-launcher|Hard Launcher]]'
  slots_of_30: 0.4
- item: '[[items/feet/619-tamiya-shoes|Tamiya Shoes]]'
  slots_of_30: 0.4
- item: '[[items/body/519-ranger-chest|Ranger Chest]]'
  slots_of_30: 0.4
- item: '[[items/hands/419-purple-gloves-of-witch|Purple Gloves of Witch]]'
  slots_of_30: 0.4
- item: '[[items/head/416-trunket-helm|Trunket Helm]]'
  slots_of_30: 0.4
- item: '[[items/material/154-pink-hearts|Pink Hearts]]'
  slots_of_30: 0.4
- item: '[[items/material/152-green-hearts|Green Hearts]]'
  slots_of_30: 1
- item: '[[items/material/211-wooden-cart-schematic|Wooden Cart Schematic]]'
  slots_of_30: 1
quests:
- '[[quests/131-eva-the-sorcerer|Eva the Sorcerer]]'
spawns:
- zone: '[[zones/2-city-of-junon-polis|City of Junon Polis]]'
  x: 5035
  y: 5481.9
  count: 1
  group: reinforcements
- zone: '[[zones/2-city-of-junon-polis|City of Junon Polis]]'
  x: 5130.4
  y: 5495.3
  count: 1
  group: reinforcements
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5446.7
  y: 5177.5
  count: 1
  group: basic
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5401.7
  y: 4962.6
  count: 1
  group: reinforcements
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5379.6
  y: 4966.9
  count: 1
  group: reinforcements
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5614.8
  y: 5044.5
  count: 1
  group: reinforcements
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5611.6
  y: 5020.2
  count: 1
  group: reinforcements
- zone: '[[zones/26-forest-of-wisdom|Forest of Wisdom]]'
  x: 5470.7
  y: 4898.9
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 209, ITEM_DROP.STB row 204
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Grunter King

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
