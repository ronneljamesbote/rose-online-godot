---
kind: backlog
id: chat-rooms
name: Chat rooms
status: not-in-game-yet
summary: Player-made chat rooms with a title, an optional password and a member limit, listed in the community window
source:
  data: 3DDATA/CONTROL/XML DlgComm.xml (Chat Room tab), DlgChatRoom.xml (title up to 25 characters, password, member settings); LIST_STRING.STL strings 538-542
  reference: iROSE 129_129en client data and TRose.exe strings (ChatRoom, DlgChatRoom)
---
# Chat rooms

## How it works in iROSE 129

The community window's **Chat Rooms** tab lists open chat rooms. A chat room is a private
group chat that anyone can make and that works across zones.

- **Making a room**: the room settings pane of the chat room window takes a **title** (up to
  25 characters), an optional **password** and the **member limit**. The maker is the room's
  owner. Failure: "You have failed to make a Chat Room." (538).
- **Joining**: pick a room from the list. A locked room asks for its password ("Incorrect
  Password.", 539). "This Chat Room does not exist." (540), "This Chat Room is full." (541).
- **Talking**: the chat room window (DlgChatRoom) has its own text box and log; only room
  members see the messages. It can be minimised.
- **Members**: a member pane lists who is in the room. The owner can remove members: "You
  have been kicked out of the Chat Room." (542).
- The room closes when everyone has left.

> Open question: the highest member limit allowed, what happens to the owner role when the
> owner leaves, and whether rooms are per channel or per server.

## What our game does today

No chat rooms. Chat has nearby, shout, party and whisper channels only (see
[[rules/chat|Chat]]).

## Building it

- **Server**: `chat_room` and `chat_room_member` tables; reducers to create, join (password
  check, limit), leave, kick and talk; room messages stored like other chat messages so only
  members can read them.
- **Client**: a Chat Rooms tab in the friends (community) window and a chat room window.
- **Data**: none.
