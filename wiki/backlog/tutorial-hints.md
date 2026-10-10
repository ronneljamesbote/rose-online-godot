---
kind: backlog
id: tutorial-hints
name: Tutorial hints
status: not-in-game-yet
summary: New characters get blinking hint buttons at set levels and experience points that open short help messages and point at menu buttons
source:
  data: SCRIPTS/TUTORIAL.LUA; 3DDATA/STB/EVENTBUTTON.STB (button to script), LEVELUPEVENT.STB (level to button), LIST_EVENTSTRING.STL (message texts 1-10); DlgNotify.xml
  reference: iROSE 129_129en client data
---
# Tutorial hints

## How it works in iROSE 129

The client shows a small **event button** at the side of the screen when a new player
reaches certain points. Clicking it runs a function in `SCRIPTS/TUTORIAL.LUA`, which opens a
notice window (DlgNotify) with a help text and can open the menu and make one of its
buttons blink.

| Button | When it appears (EVENTBUTTON.STB) | Script | What it does |
| --- | --- | --- | --- |
| 1 | first login (level 1, 0 experience) | Tutorial01 | message 1 (welcome, how to move and fight) |
| 2 | reaching level 2 | Tutorial02 | message 2, blinks the Character button |
| 3 | level 2 with 150 experience | Tutorial03 | message 3 |
| 4 | level 2 with 220 experience | Tutorial04 | message 4, blinks the Inventory button |
| 5 | reaching level 3 | Tutorial05 | message 5, blinks the Skill button |
| 6 | level 3 with 100 experience | Tutorial06 | message 6 |
| 7 | getting quest 5051 | Tutorial07 | message 7, blinks the Quest button |
| 8 | reaching level 4 | Tutorial08 | message 8 |
| 9 | after the `T-warp` trigger | Tutorial09 | message 9 |
| 10 | reaching level 10 | Tutorial10 | message 10 ("You've reached Level 10. Now, you can get a job.") |

- Level-up buttons come from LEVELUPEVENT.STB (level 2 → button 2, 3 → 5, 4 → 8, 10 → 10).
- Experience buttons come from `CheckTutorialEvent` in the script, which compares your
  experience before and after each gain.
- Buttons 7 and 9 are created by quest triggers `T77` and `T-warp` in TUTORIAL.qsd, which
  call `Tutorial07button` and `Tutorial09button`.
- The texts are LIST_EVENTSTRING.STL entries 1 to 10.
- The script can also put markers on NPCs and map coordinates (`SC_AddNpcIndicator`,
  `SC_AddCoordinatesIndicator`), though the 129 tutorial does not use them.

> Open question: whether a button stays until clicked and whether hints can be turned off
> in the options.

## What our game does today

None of this; new players get no hints. The tutorial quests themselves work.

## Building it

- **Client only**: watch level, experience and quest changes; show the event buttons;
  show the texts from LIST_EVENTSTRING.STL; blink the menu buttons. No server change.
- **Data**: none to change.
