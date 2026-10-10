---
kind: backlog
id: clans
name: Clans
status: not-in-game-yet
summary: Player clans with a master, ranks, invites, a slogan and notice, founded and disbanded at Burtland in Junon Polis
npcs:
  - "[[npcs/1115-clan-owner-burtland|Clan Owner Burtland]]"
source:
  data: LIST_STRING.STL strings 36, 44-78, 90-98, 364, 385; STR_CLAN.STL (rank names); QC001.QSD triggers Clan-001 to Clan-004; 3DDATA/CONTROL/XML DlgClan.xml, DlgOrganizeClan.xml, DlgClanRegNotice.xml; NPC dialog EM02-035.con
  reference: iROSE 129_129en client data and TRose.exe strings (CTEventClan, DlgClan, DlgOrganizeClan); rose-offline vendored types in crates/rose-game-common/src/components/clan.rs, crates/rose-data/src/clan.rs and the clan messages in crates/rose-game-common/src/messages
---
# Clans

A clan is a named group of players with one master. Clans are the base for the other
clan pages: [[backlog/clan-grades-and-skills|clan grades, points and skills]],
[[backlog/clan-marks|clan marks]], the [[backlog/clan-house|clan house]] and the
[[backlog/clan-fields|clan fields]].

## How it works in iROSE 129

### Founding a clan

You found a clan by talking to [[npcs/1115-clan-owner-burtland|Clan Owner Burtland]] in
[[zones/2-city-of-junon-polis|City of Junon Polis]] and choosing "I would like to create a
Clan". His script calls `GF_organizeClan`, which opens the clan founding window.

| Requirement | Value |
| --- | --- |
| Level | 30 or higher (dialog check and string 98) |
| Money | 1,000,000 Zulie, taken when the clan is made (string 98) |
| Clan | you must not be in a clan (trigger `Clan-001`: HasClan false) |

The founding window (DlgOrganizeClan) has:

- **Clan name**: up to 16 characters. A name that already exists is refused with "This
  Clan name already exists." (string 59).
- **Slogan**: up to 64 characters, required ("Please enter your Clan slogan.", string 78).
- **Clan mark**: a background and a symbol picked from two grids (see
  [[backlog/clan-marks|Clan marks]]).

On success the founder becomes **Master** and sees "You are now the master of a new Clan."
(string 55). The new clan is grade 1 with 0 clan points. Failure messages: "You have failed
to make a Clan." (58), "You do not have the authority to create a Clan." (60), "There are
still some unfulfilled requirements to create a Clan." (77).

### Ranks

Each member has a position. The names come from STR_CLAN.STL; the numbers are the values
quest checks compare (`ClanPosition`).

| Value | Rank | Can do |
| --- | --- | --- |
| 0 | Rookie (penalty) | nothing extra |
| 1 | Rookie | nothing extra |
| 2 | Veteran | nothing extra |
| 3 | Captain | nothing extra |
| 4 | Commander | can be given the mastery |
| 5 | Deputy Master | manage the clan at Burtland (trigger `Clan-004`), warp the clan to the training camp |
| 6 | Master | everything: grade up, clan skills, disband, entrust mastery |

> Open question: which ranks may invite, kick, promote and demote. The client has the
> buttons for every rank and the server answers "You do not have the authority" (string 66
> for invites, 76 in general); the exact rule needs checking against an iROSE server source.

### Members

- **Invite**: a member with the right rank invites an online player. The player gets
  "*name* has invited you to join a Clan." (71) and accepts or declines ("*name* has declined
  Clan invitation.", 72). Everyone in the clan sees "*name* has been invited to Clan by
  *name*." (64). A player already in a clan cannot be invited (67). A full clan refuses with
  "Your Clan is full." (96).
- **Member limit**: 15 members at grade 1, more at higher grades (Burtland's dialog). The
  clan window shows it as "Max Clan Member".
- **Leave** (Withdrawal button): "*name* has left the Clan." (70). The master cannot leave
  (75).
- **Kick** (Ban button, with a yes/no question, 92): "*name* has been kicked out of the Clan by
  *name*." (68).
- **Promote / demote** (Change Class button, questions 93 and 94): the member sees "Your
  Clan grade has changed to *rank*." (73).
- **Entrust mastery**: the master hands the clan to a member of rank Commander or higher
  (string 90, question 91). The old master steps down.
- A clan master cannot delete their character (74).
- Members see each other log in and out; the member list shows name, rank, level, job,
  channel and personal contribution (clan points that member earned).

### The clan window

Opened with the Clan button on the menu bar (`MENU_BTN_CLAN`). DlgClan has four tabs:

1. **Members**: the list above, with Entrust, Ban, Change Class, Memo (see
   [[backlog/memos|Memos]]) and Request Join buttons.
2. **Info**: clan name, grade, clan points, slogan, clan funds, allied clan, member limit,
   your position and your contribution (strings 44 to 53), and the mark with the Preview
   and Register Clan Mark buttons.
3. **Skills**: the clan skills the clan has (see
   [[backlog/clan-grades-and-skills|Clan grades, points and skills]]).
4. **Notice**: a clan notice board. The master writes a notice of up to 250 characters
   (DlgClanRegNotice) that every member sees.

Other players see the clan name and mark over the heads of clan members.

### Disbanding

Only the master can disband (trigger `Clan-002`: position 6), at Burtland ("I'd like to
disorganize my Clan", script `GF_disorganizeClan`), after a warning that all clan
information is deleted (95). It only works when no other members remain: "You cannot
disorganize your Clan if there are remaining members." (97). Members see "Your Clan has been
disorganized." (57/61).

### Clan chat and allies

The chat window has a Clan tab and an Ally tab; clan messages reach every online member.
The Info tab shows an "Allied Clan". See [[backlog/chat-tabs|Chat tabs]].

> Open question: how two clans become allies in 129 (the Clan Union skills 3416 and 3417
> exist, but no NPC or quest grants them).

## What our game does today

Nothing. There are no clans: Burtland's create and disband choices only say the service
is not in the game, quest checks on clans fail, quest rewards that touch a clan do nothing
(`module/src/quests.rs`), and "everyone except clan" PvP zones let everyone fight
(`module/src/pvp.rs`).

## Building it

- **Server**: `clan` and `clan_member` tables (name, slogan, notice, mark, grade, points,
  funds, member ranks and contributions); reducers to found (level, Zulie and name checks),
  invite and answer, leave, kick, promote, demote, entrust and disband; online and offline
  notices; clan checks and rewards in the quest code (`HasClan`, `ClanPosition`,
  `ClanLevel`, `ClanPoints`, clan rewards); clan-aware PvP in `module/src/pvp.rs`.
- **Client**: the founding window, the clan window with its four tabs, invite prompts,
  clan name over heads, a Clan button on the menu bar; wire `GF_organizeClan` and
  `GF_disorganizeClan` in `godot/rust/src/conversation.rs`.
- **Data**: none to change; ranks come from STR_CLAN.STL and messages from LIST_STRING.STL.

> Open question: the member limit for each grade above 1 is not in the client data.
> rose-offline's server code (not vendored here) or rose-next may have the table.
