---
kind: item
id: weapon/262
name: Basic Launcher
status: in-game
icon: item/1249
class: Launcher
level: 39
attack: 67
attack_speed: 12
range: 25
damage: physical
two_handed: true
durability: 40
quality: 48
needs: Strength 36
sockets: true
price: 21915
weight: 20
crafted_with: '[[skills/1211-damage-support|Damage Support]] level 2'
recipe:
- material: any Metal
  quantity: 55
- material: '[[items/material/52-cotton-cloth|Cotton Cloth]]'
  quantity: 25
- material: '[[items/material/179-weed-root|Weed Root]]'
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
- monster: '[[monsters/185-ranger-clown|Ranger Clown]]'
  level: 48
  slots_of_30: 0.2
- monster: '[[monsters/186-hunter-clown|Hunter Clown]]'
  level: 50
  slots_of_30: 0.4
- monster: '[[monsters/206-wild-gorilla|Wild Gorilla]]'
  level: 43
  slots_of_30: 0.4
- monster: '[[monsters/256-needle-bat|Needle Bat]]'
  level: 46
  slots_of_30: 0.2
- monster: '[[monsters/261-goblin-jar|Goblin Jar]]'
  level: 45
  slots_of_30: 0.2
sold_by:
- '[[npcs/1011-eccentric-inventor-spero|Eccentric Inventor Spero]]'
- '[[npcs/1096-ferrell-guild-merchant-mildun|Ferrell Guild Merchant Mildun]]'
quest_reward:
- '[[quests/1008-the-humble-smith|The Humble Smith]]'
source:
  data: LIST_WEAPON.STB row 262
  code: module/src/items.rs
---
# Basic Launcher

A basic Launcher that is fairly stable.
