---
kind: backlog
id: memos
name: Memos (mailbox)
status: not-in-game-yet
summary: Short letters to friends and clan members that wait in the server until the receiver logs in, kept in a mailbox tab of the community window
source:
  data: 3DDATA/CONTROL/XML DlgMemo.xml (180 character text), DlgMemoView.xml, DlgComm.xml (Mailbox tab), DlgClan.xml (Memo button); LIST_STRING.STL strings 546, 548, 549
  reference: iROSE 129_129en client data and TRose.exe strings (DlgMemo, DlgMemoView, the SQLite table "memo (name, sender, content, time)")
---
# Memos (mailbox)

## How it works in iROSE 129

The community window (DlgComm) has three tabs: Friends, Chat Rooms and **Mailbox**. A memo
is a short letter to another character.

- **Writing**: from the friends list or the clan member list (Memo button) you open the
  memo window (DlgMemo): the receiver's name, a text of up to **180 characters** and Send.
- **Delivery**: the server delivers the memo when the receiver is online, or keeps it until
  they log in. Errors: "This character is currently rejecting messages." (546), "Your
  friend's mailbox is full." (549).
- **Reading**: memos arrive in the Mailbox tab with sender and time. The viewer (DlgMemoView)
  shows one memo with OK and Delete.
- **Storage**: the client keeps received memos in its own database, `Rose.db` in the game
  folder, table `memo (name, sender, content, time)` per character; deleting removes the
  row there. So read memos live on the player's computer, not the server.
- A player can refuse messages ("Declining message.", 548).

> Open question: how many memos the server keeps waiting for one character (the "mailbox is
> full" limit), whether memos can go to anyone or only to friends and clan members, and how
> a player switches "rejecting messages" on.

## What our game does today

No memos. Players can [[rules/chat|whisper]] online players and keep a
[[rules/friends|friends list]], but cannot leave a message for someone who is offline.

## Building it

- **Server**: a `memo` table (receiver, sender, text, time) with a per-receiver limit; a
  send reducer (length and limit checks, refusal flag); receivers see only their own memos
  (a view like `my_chat`); delete when read or after a time.
- **Client**: a Mailbox tab in the friends window, the write and read windows, a Memo
  button on friends (and later clan members). Keeping read memos on the server instead of a
  local database is simpler for us.
- **Data**: none.
