---
kind: rule
id: accounts
name: Accounts and new characters
status: changed-from-irose
characters_per_account: 1
password_length: 8 to 128 characters
email_max_length: 254 characters
verify_link_hours: 24
reset_link_minutes: 60
website_session_days: 30
game_token_minutes: 15
name_length: 3 to 16 characters
name_characters: letters (A-Z, a-z) and digits, starting with a letter
genders: male, female
faces: [1, 8, 15, 22, 29, 36, 43]
hair_styles: [0, 5, 10, 15, 20]
start_zone: "[[zones/22-adventurer-s-plain|Adventurer's Plain]]"
start_position_m: "(5760.45, 5166.60), the zone's start spot"
start_quest: "[[quests/801-junon-planet-identification|Junon Planet Identification]]"
start_level: 1
start_job: Visitor
start_zuly: 0
start_stats:
  - { stat: STR, male: 15, female: 15 }
  - { stat: DEX, male: 15, female: 15 }
  - { stat: INT, male: 15, female: 15 }
  - { stat: CON, male: 15, female: 15 }
  - { stat: CHA, male: 10, female: 10 }
  - { stat: SEN, male: 10, female: 10 }
start_items:
  - { item: "[[items/weapon/2-short-sword|Short Sword]]", where: "equipped", who: "everyone" }
  - { item: "[[items/body/30-visitor-look|Visitor Look]]", where: "equipped", who: "everyone" }
  - { item: "[[items/weapon/202-short-bow|Short Bow]]", where: "bag", who: "everyone" }
  - { item: "[[items/material/301-wooden-arrow|Wooden Arrow]] x999", where: "bag", who: "everyone" }
  - { item: "[[items/head/222-peacock-feather|Peacock Feather]]", where: "bag", who: "male" }
  - { item: "[[items/head/221-tulip-ribbon|Tulip Ribbon]]", where: "bag", who: "female" }
start_skills:
  - "[[skills/11-sit|Sit]]"
  - "[[skills/12-pick-up|Pick Up]]"
  - "[[skills/13-jump|Jump]]"
  - "[[skills/16-normal-attack|Normal Attack]]"
  - "[[skills/19-party|Party]]"
  - "[[skills/20-trade|Trade]]"
  - "[[skills/21-vending|Vending]]"
  - "[[skills/25-ride-request|Ride Request]]"
  - "[[skills/41-hi|Hi]]"
  - "[[skills/42-grateful-bow|Grateful Bow]]"
  - "[[skills/43-salutation|Salutation]]"
  - "[[skills/44-recovery-kiss|Recovery Kiss]]"
  - "[[skills/45-charming-kiss|Charming Kiss]]"
  - "[[skills/46-laugh|Laugh]]"
  - "[[skills/47-fight-cheer|Fight Cheer]]"
  - "Break Down (skill 48)"
  - "[[skills/49-tantrum|Tantrum]]"
  - "[[skills/50-applause|Applause]]"
source:
  code:
    - module/src/account.rs (check_connection, name_problem, create_character, assign_character, set_auth_issuer)
    - module/src/character.rs (new_player)
    - crates/rose-game-data/src/lib.rs (CharacterCreator)
    - module/src/lib.rs (START_ZONE, START_QUEST_TRIGGER, start_place, new_character, client_connected, spawn_player_entity, set_name)
    - crates/rose-game-data/src/lib.rs (CharacterCreator)
    - godot/scripts/character_create.gd
    - godot/scripts/connect_panel.gd
    - godot/scripts/main.gd (_go_online)
    - web/src/lib/validate.ts (normalizeEmail, passwordProblem)
    - web/src/lib/config.ts
    - web/src/app/api/signup/route.ts
    - web/src/app/api/game/login/route.ts
    - web/src/app/api/forgot-password/route.ts
  data: the new character resources godot/data/new_characters/male.tres and female.tres (RoseNewCharacter; stats, clothes, bag)
---
# Accounts and new characters

You make your account on the website, then sign in with the same email and password in
the game. Each account has **one character**, which you make the first time you sign in.

## Your account (website)

- **Sign up** with an email address and a password. The email is stored in lower case and
  can be up to 254 characters. The password must be **8 to 128 characters**, can't be your
  email, and can't be one character repeated (like `aaaaaaaa`).
- One account per email address.
- You are signed in on the website straight away, and get an email with a link to
  **confirm your email**. The link works once, for **24 hours**; the website can send a new
  one. When the server sends email, you can't enter the game until your email is confirmed.
- **Forgot your password?** Ask for a reset link on the website. It works once, for
  **60 minutes**. The website gives the same answer whether or not the email has an account.
  Resetting the password signs you out everywhere.
- Staying signed in on the website lasts **30 days** from your last visit.

The game never stores your password. When you sign in from the game, the website gives the
game a pass that is good for **15 minutes** to connect.

A [[rules/gm-commands|dev server]] needs no account: the dev client's **Connect without
account** button signs in with a key the game keeps on the PC, one per server.

Too many attempts are slowed down: 10 sign-in attempts per email (and 100 per address) in
15 minutes, 20 sign-ups per address per hour, and 3 reset requests per email per hour.

## Making your character

After signing in to an account with no character, you see **Create your character**:

- **Name**: 3 to 16 characters, only letters (A-Z) and digits, starting with a letter. A
  name is taken if another character has it, ignoring capital letters (so "Rose" and "ROSE"
  are the same name). A name can't be changed later.
- **Gender**: male or female.
- **Face**: one of 7 faces (face numbers 1, 8, 15, 22, 29, 36, 43).
- **Hair**: one of 5 hair styles (hair numbers 0, 5, 10, 15, 20).

The screen shows a turning preview of the character.

## What a new character starts with

- **Where**: [[zones/22-adventurer-s-plain|Adventurer's Plain]], at the zone's start spot
  (5760.45 m, 5166.60 m), 4 m in front of [[npcs/1031-visitor-guide-ronway|Visitor Guide Ronway]]
  and the [[npcs/1030-visitor-guide-fairy-of-arua|Fairy of Arua]], who face it.
  The start spot is the "start" marker in the zone's Godot scene, so moving the marker
  moves where new characters start. Dying before choosing a save point gets you up at the
  zone's nearest revive point (see [[rules/death|Death]]).
- **Level** 1, 0 experience, job Visitor, no stat points or skill points, 0 Zuly, full HP
  and MP.
- **Stats**: STR 15, DEX 15, INT 15, CON 15, CHA 10, SEN 10 (the same
  for both genders).
- **Gear**: wearing the [[items/body/30-visitor-look|Visitor Look]] and holding a
  [[items/weapon/2-short-sword|Short Sword]]. In the bag: a
  [[items/weapon/202-short-bow|Short Bow]] with 999
  [[items/material/301-wooden-arrow|Wooden Arrows]], and a hat: the
  [[items/head/222-peacock-feather|Peacock Feather]] for men or the
  [[items/head/221-tulip-ribbon|Tulip Ribbon]] for women.
- **Skills**: the basic actions and emotes listed in the data block (sit, pick up, jump,
  attack, party, trade, vending, ride request and ten emotes).
- **Quest**: [[quests/801-junon-planet-identification|Junon Planet Identification]], with its
  [[items/quest/101-identification|Identification]] papers in the bag (quest step `801-01`,
  run once when the character is made).

See [[rules/stats|Stats]] for what the stats do and [[rules/zones-and-warps|Zones and warps]]
for getting around.

## Changed from iROSE

- iROSE has up to several characters per account, made on a character select screen; here
  each account has exactly one.
- iROSE (INIT_AVATAR.STB) gives a [[items/weapon/1-wooden-sword|Wooden Sword]]; we give a
  Short Sword instead, plus the Short Bow and arrows to try ranged combat.
- Accounts live on our website (email and password) instead of the iROSE login server.
- iROSE starts new characters on [[zones/20-birth-island|Birth Island]], where the Fairy of
  Arua gives the Party and Vending skills, the Junon Planet Identification quest and its
  papers when you leave for Adventurer's Plain. We start them in
  [[zones/22-adventurer-s-plain|Adventurer's Plain]] with all of that already, keep Birth
  Island for testing only, and place one Fairy of Arua next to Ronway in Adventurer's Plain.

> Open question: the Add Friend action is not on a new character's skill list; players use the right-click menu on another player instead.
