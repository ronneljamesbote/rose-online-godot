---
kind: backlog
id: clan-grades-and-skills
name: Clan grades, points and skills
status: not-in-game-yet
summary: Clans grow from grade 1 to 3 with clan points and material quests, and learn clan skills such as the Clan House
npcs:
  - "[[npcs/1115-clan-owner-burtland|Clan Owner Burtland]]"
source:
  data: QC001.QSD triggers Clan-011 to Clan-023, Clan-031, Clan-032 and 2751-01 to 2753-03; LIST_QUEST.STB rows 2751-2753; LIST_SKILL.STB rows 3401-3405, 3411-3412, 3416-3417, 3421-3422, 3431-3432 (skill type 15, tab 3); LIST_STRING.STL strings 45, 46, 48, 385
  reference: iROSE 129_129en client data; Burtland's dialog EM02-035.con; QSD reward types in crates/rose-file-readers/src/qsd.rs
---
# Clan grades, points and skills

Every [[backlog/clans|clan]] has a **grade** (clan level), **clan points**, **clan funds**
(Zulie) and a list of **clan skills**. Members also keep a personal **contribution**, the
clan points they earned.

## How it works in iROSE 129

### Grades

A clan starts at grade 1. Only the master can raise it, at
[[npcs/1115-clan-owner-burtland|Clan Owner Burtland]] ("I would like to manage my Clan",
then "I would like to raise my Clan Grade"). Grade 3 is the highest in the 129 data
("Your Clan has been raised to Grade 2, and can't be upgraded anymore" is shown at grade 3).

| From | To | Checks | Cost |
| --- | --- | --- | --- |
| 1 | 2 | master (position 6), grade 1, clan points ≥ 1,000 (trigger `Clan-011`) | none, points are not spent |
| 2 | 3 | master, grade 2, clan points ≥ 5,000; then one material quest below | the materials |

For grade 3 Burtland offers three "Clan House Extension" quests; the master picks one
item group (normal items, not quest items) and hands them in to finish it. Each quest's
last step removes the items and raises the grade (`ClanLevelIncrease`). Clan members can
collect the items for the master.

| Quest | Items |
| --- | --- |
| [[quests/2751-clan-house-extension-1\|Clan House Extension (1)]] | 600 × [[items/material/6-silver-iron\|Silver Iron]], 400 × [[items/material/36-pine-wood\|Pine Wood]], 10 × [[items/material/162-blue-crystal\|Blue Crystal]] |
| [[quests/2752-clan-house-extension-2\|Clan House Extension (2)]] | 200 × [[items/material/7-steel\|Steel]], 150 × [[items/material/37-cinnamon-wood\|Cinnamon Wood]], 50 × [[items/material/123-earth-stone\|Earth Stone]] |
| [[quests/2753-clan-house-extension-3\|Clan House Extension (3)]] | 100 × [[items/material/14-molive\|Molive]], 50 × [[items/material/8-chromium\|Chromium]], 2 × [[items/material/152-green-hearts\|Green Hearts]] |

When the grade goes up every member sees "Your Clan Grade has been raised." (string 385).
A higher grade raises the member limit and unlocks clan skills (Burtland's dialog).

### Clan points and funds

Clan points measure the clan's activity. Quest rewards can add clan points
(`ClanPoints`), add to a member's contribution (`ClanPointContribution`) and add clan funds
(`ClanMoney`). Burtland says points come from "achieving Clan Objectives in the Clan
Field".

> Open question: no quest in the 129 QSD files gives clan points or clan money, so in this
> data a clan has no way to reach 1,000 points. The original server probably granted points
> another way (for example kills in the [[backlog/clan-fields|clan fields]]); this needs a
> source before it can be built.

### Clan skills

Clan skills are LIST_SKILL.STB rows with skill type 15 on skill tab 3. They belong to the
clan, not to a character, and every member has them. Quest rewards add and remove them
(`AddClanSkill`, `RemoveClanSkill`) and quest checks test them (`HasClanSkill`).

| Skill | Rows | Notes |
| --- | --- | --- |
| [[skills/3401-clan-approval\|Clan Approval]] | 3401-3405 | five levels, no effect columns |
| [[skills/3411-clan-fields\|Clan Fields]] | 3411-3412 | no effect columns |
| [[skills/3416-clan-union\|Clan Union]] | 3416-3417 | alliance of clans |
| [[skills/3421-planet-war\|Planet War]] | 3421-3422 | no content in 129 |
| [[skills/3431-clan-house\|Clan House]] | 3431-3432 | 30 days, see [[backlog/clan-house\|Clan house]] |

Only the Clan House skill is given by anything in the 129 data (trigger `Clan-031`: master,
grade 3 or higher, pay 1,000,000 Zulie). Its "learn points" column (4320) is its lifetime:
in 10-minute units that is 30 days, which matches Burtland ("You can use your Clan House for
only 30 days").

> Open question: when the other clan skills were granted (for example Clan Approval by
> grade) is not in the client data.

## What our game does today

No clans, so no grades, points, funds or clan skills. The clan skill pages exist as plain
passive skills nobody can get. Burtland's grade options never pass their checks, and the
clan rewards in `module/src/quests.rs` fail.

## Building it

- **Server**: grade, points, funds and skill list (with expiry time) on the clan row;
  contribution per member; quest checks `ClanLevel`, `ClanPoints`, `HasClanSkill` and rewards
  `ClanLevelIncrease`, `ClanPoints`, `ClanMoney`, `ClanPointContribution`, `AddClanSkill`,
  `RemoveClanSkill`; expire timed clan skills; member limit by grade.
- **Client**: show grade, points, funds and skills in the clan window; the grade-up notice.
- **Data**: none; the grade steps are in QC001.QSD.
