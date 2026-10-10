---
kind: backlog
id: chat-tabs
name: Chat tabs, trade, clan and ally chat
status: not-in-game-yet
summary: The iROSE chat box has tabs that filter the log (All, Whisper, Trade, Party, Clan, Ally) and three channels our chat lacks
source:
  data: 3DDATA/CONTROL/XML DlgChat.xml (tab buttons CHAT_BTN_ALL, WHISPER, TRADE, PARTY, CLAN, ALLY), DlgChatFilter.xml
  reference: iROSE 129_129en client data; wiki/rules/chat.md
---
# Chat tabs, trade, clan and ally chat

## How it works in iROSE 129

The chat box has six tab buttons along its top: **All**, **Whisper**, **Trade**, **Party**,
**Clan** and **Ally**, plus a **Filter** button (DlgChatFilter, a menu of check marks).

- Each tab shows only that kind of message in the log; All shows everything. The filter
  menu hides kinds of messages from the All tab.
- Typing while a tab is selected sends to that tab's channel.
- Three channels go beyond the ones our game has:
  - **Trade**: buy and sell messages, heard more widely than nearby talk.
  - **Clan**: every online member of your [[backlog/clans|clan]], wherever they are.
  - **Ally**: your allied clans, or your faction.

> Open question: the sign that picks each of these channels when typing (like `!` for shout
> and `#` for party), how far trade chat reaches (zone or server), and whether "Ally" means
> the allied clan or the faction. These are not in the data files; check the original
> client.

## What our game does today

One log with nearby, shout, party and whisper messages, each in its own colour (see
[[rules/chat|Chat]]). No tabs, no filter, no trade, clan or ally channel.

## Building it

- **Server**: new chat kinds (trade, clan, ally) with their audiences in
  `module/src/chat.rs`; clan and ally need [[backlog/clans|clans]] first.
- **Client**: the tab row and filter menu in the chat window; the typing signs.
- **Data**: none.
