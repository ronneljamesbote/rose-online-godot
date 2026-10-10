---
kind: item
id: weapon/334
name: Elven Wand
status: in-game
icon: item/1329
class: Magic Tool
level: 39
attack: 41
attack_speed: 12
range: 18
damage: magic
two_handed: false
durability: 43
quality: 40
needs: Intelligence 80
bonus: MP Consumption +8
sockets: true
price: 9880
weight: 5
crafted_with: '[[skills/2491-magic-weapon-craft|Magic Weapon Craft]] level 2'
recipe:
- material: any Wooden Material
  quantity: 28
- material: '[[items/material/1-rusted-iron|Rusted Iron]]'
  quantity: 15
- material: '[[items/material/177-insect-wing|Insect Wing]]'
  quantity: 9
craft_difficulty: 26
dropped_by:
- monster: '[[monsters/53-old-smouly|Old Smouly]]'
  level: 48
  slots_of_30: 0.4
- monster: '[[monsters/136-grunter-warrior|Grunter Warrior]]'
  level: 45
  slots_of_30: 0.2
- monster: '[[monsters/161-jewel-golem|Jewel Golem]]'
  level: 47
  slots_of_30: 0.2
- monster: '[[monsters/181-small-clown|Small Clown]]'
  level: 47
  slots_of_30: 0.4
- monster: '[[monsters/183-fighter-clown|Fighter Clown]]'
  level: 52
  slots_of_30: 0.2
- monster: '[[monsters/185-ranger-clown|Ranger Clown]]'
  level: 48
  slots_of_30: 0.4
- monster: '[[monsters/186-hunter-clown|Hunter Clown]]'
  level: 50
  slots_of_30: 0.2
- monster: '[[monsters/206-wild-gorilla|Wild Gorilla]]'
  level: 43
  slots_of_30: 0.2
- monster: '[[monsters/256-needle-bat|Needle Bat]]'
  level: 46
  slots_of_30: 0.4
- monster: '[[monsters/261-goblin-jar|Goblin Jar]]'
  level: 45
  slots_of_30: 0.2
- monster: '[[monsters/271-goblin-worker|Goblin Worker]]'
  level: 51
  slots_of_30: 0.2
sold_by:
- '[[npcs/1006-arumic-merchant-tryteh|Arumic Merchant Tryteh]]'
- '[[npcs/1091-arumic-merchant-chester|Arumic Merchant Chester]]'
quest_reward:
- '[[quests/912-an-appropriate-compromise|An Appropriate Compromise]]'
source:
  data: LIST_WEAPON.STB row 334
  code: module/src/items.rs
---
# Elven Wand

A Wand containing the holy magic of the Elves.
