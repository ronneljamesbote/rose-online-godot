---
kind: backlog
id: faction-wars
name: Faction Wars
status: not-in-game-yet
summary: Scheduled team battles between factions in the Junon Cartel maps, fought over crystals for Zulie and faction points
npcs:
  - "[[npcs/1088-founder-of-junon-order-raw|Founder of Junon Order Raw]]"
  - "[[npcs/1089-manager-of-ferrell-arothel|Manager of Ferrell Arothel]]"
  - "[[npcs/1090-righteous-crusader-gawain|Righteous Crusader Gawain]]"
  - "[[npcs/1091-arumic-merchant-chester|Arumic Merchant Chester]]"
  - "[[npcs/1086-akram-minister-rodath|Akram Minister Rodath]]"
  - "[[npcs/1087-akram-minister-mel|Akram Minister Mel]]"
  - "[[npcs/1082-guide-eva|Guide Eva]]"
  - "[[npcs/1113-referee-leum|Referee Leum]]"
  - "[[npcs/1114-referee-pirre|Referee Pirre]]"
source:
  data: PvP10.qsd (quests 2851-2858, triggers PvP10-*); PvP13-01.qsd (quests 3101-3102, triggers PvP1301-*); LIST_QUEST.STB rows 2851-2858, 3101-3102; LIST_ZONE.STB rows 5, 8, 9 (join, kill and death triggers); LIST_NPC.STB rows 431-433 (Sunset, Sunrise and Dusk Crystal); NPC AI and dialogs of the NPCs listed
  reference: iROSE 129_129en client data, read with a scratch copy of crates/rose-file-readers; NPC dialogs (Raw explains the rules)
---
# Faction Wars

Factions (see [[backlog/faction-stores|Faction Point Shops]]) fight each other in timed team
battles. Two war scripts exist in the 129 data.

## How it works in iROSE 129

### The crystal war (PvP10.qsd)

Junon Order and Arumic fight the Righteous Crusaders and Ferrell Guild in
[[zones/9-zone-9|zone 9]] (the second Junon Cartel map, "JPVP05"). Each faction has an
attacking and a defending quest, 70 minutes long:

| Faction | Attack quest | Defend quest | Signs up at |
| --- | --- | --- | --- |
| Junon Order | [[quests/2851-retribution-upon-the-blasphemous\|Retribution upon the Blasphemous]] | [[quests/2852-protected-by-holiness\|Protected by Holiness]] | [[npcs/1088-founder-of-junon-order-raw\|Raw]] |
| Righteous Crusaders | [[quests/2853-recovery-of-justice\|Recovery of Justice]] | [[quests/2854-dignity-of-the-righteous-crusaders\|Dignity of the Righteous Crusaders]] | [[npcs/1090-righteous-crusader-gawain\|Gawain]] |
| Arumic | [[quests/2855-wounded-pride\|Wounded Pride]] | [[quests/2856-supremacy-of-mana-theory\|Supremacy of Mana Theory]] | [[npcs/1091-arumic-merchant-chester\|Chester]] |
| Ferrell Guild | [[quests/2857-taking-back-ferrell-road\|Taking Back Ferrell Road]] | [[quests/2858-justification-of-wealth\|Justification of Wealth]] | [[npcs/1089-manager-of-ferrell-arothel\|Arothel]] |

Sign-up (triggers 2851-01 and the like): a faction member on **channel 1** talks to their
faction NPC while it is recruiting. Each sign-up adds to a counter on the NPC: +1 below
level 70, +2 at 70-89, +3 at 90-109, +4 at 110 and up. When the counter reaches the target
(120 for an attack, 100 for a defence) the side is full and the NPC's state moves on. NPC
object variables and the NPCs' AI run the schedule; the NPCs announce the war in the zone
(NPC messages 25 and 26) and the warp gate at the fountain of Junon Polis opens (an event
object whose state the script switches).

The battle (Raw's explanation and triggers PvP10-031 onwards):

- Junon Order and Arumic players are team 11, Righteous Crusaders and Ferrell Guild players
  team 13. On entering zone 9 (or [[zones/8-lion-s-plains|Lion's Plains]], which has the
  same triggers) the zone's join trigger (PvP10-331 to PvP10-339) sets the player's team
  from their war quest and sets their get-up point to (5199.91, 4784.50).
- The referee spawns 2 **Sunrise Crystals** near the centre for the defenders and a **Dusk
  Crystal** for the attackers. Destroying a Sunrise Crystal makes the **Sunset Crystal**
  appear.
- The attackers win by destroying the Sunset Crystal within 30 minutes. The defenders win
  by holding out for 30 minutes. Destroying the other side's Dusk Crystal sends that side
  back to its get-up point.
- Kills and deaths in the zone run its kill and death triggers, which count up and down
  quest variable 9 (0 to 20) for each player.
- At the end the referee runs a trigger for every player of each team (`TriggerForZoneTeam`):
  winners get Zulie (reward formulas 2 and 3 with 1,000 and 2,000) and **+12 faction points**;
  losers get Zulie (formula 2 with 500). Everyone is sent to Junon Polis at (5596.20, 5034.28)
  and the quest ends.
- [[npcs/1113-referee-leum|Referee Leum]] and [[npcs/1114-referee-pirre|Referee Pirre]]
  send a player back to town on request.

### The Cartel war (PvP13-01.qsd)

Junon Order against Righteous Crusaders in [[zones/5-junon-cartel|Junon Cartel]], with
quests [[quests/3101-wickedness-of-the-righteous-crusaders|Wickedness of the Righteous
Crusaders]] (Junon Order) and [[quests/3102-arrogance-of-the-junon-order|Arrogance of the
Junon Order]] (Crusaders). [[npcs/1088-founder-of-junon-order-raw|Raw]] and
[[npcs/1090-righteous-crusader-gawain|Gawain]] sign members up until 30 have joined each
side; [[npcs/1082-guide-eva|Guide Eva]] shouts the state of recruiting;
[[npcs/1086-akram-minister-rodath|Akram Minister Rodath]] and
[[npcs/1087-akram-minister-mel|Akram Minister Mel]] run the timer. Players are moved in at
(5171.44, 5100.15) or (5232.64, 5460.18), each side on its own team (11 or 13), and two
leaders are spawned for the teams. The winning side gets experience (formula 1, 300), 10,000
Zulie (formula 3), +20 faction points and +5 to ability 37; the losing side gets 5,000 Zulie.
Everyone returns to Junon Polis at (5515.00, 5252.00). Entering any normal zone runs its join
trigger `PvP1301-340`, which sets the team back to 2 (the normal player team).

> Open question: when the wars start. The NPC AI files drive the NPC variables; the
> schedule (times of day, how often) must be read from those AI scripts before building.

> Open question: monsters 351 and 352 are spawned as the two leaders in the Cartel war;
> their pages show them as ordinary monsters ([[monsters/351-yeti-rider|Yeti Rider]],
> [[monsters/352-rider-wizard|Rider Wizard]]). Check whether the war used them as is.

## What our game does today

Not at all. Our QSD reader (`crates/rose-file-readers/src/qsd.rs`, reward 19
`TriggerForZoneTeam`) fails on both PvP10.qsd and PvP13-01.qsd, so none of these triggers
load and the quest pages for 2851-2858 show no steps. In the files the reward is: zone (2
bytes), team (2 bytes), 2 bytes that are not a length, then the trigger name as a zero-ended
string filling the rest of the record; the reader treats those 2 bytes as a string length.
Even with that fixed, team numbers, zone triggers, `TriggerForZoneTeam`, event object states
and the NPC-variable schedule are missing.

## Building it

- **Server**: fix the reward 19 reader; teams and zone join, kill and death triggers (see
  [[backlog/pvp-teams-and-zone-triggers|PvP teams and zone triggers]]);
  `TriggerForZoneTeam`; `SpawnMonster` with a team number so crystals belong to a side; event
  object state for the fountain gate; the `ServerChannelNumber` check.
- **Client**: show the gate open or closed; team colours for players and crystals.
- **Data**: none to change.
