---
kind: rule
id: chat
name: Chat
status: in-game
max_message_length: 200 characters
nearby_range_m: 50
rate_limit: 6 messages per 5 seconds
shout_cooldown_s: 5
messages_kept_on_server_s: 60
chat_log_lines: 100
bubble_seconds: 6
channels:
  - { channel: "Nearby", type: "text", who_hears: "Players in the same zone within 50 m" }
  - { channel: "Shout", type: "!text", who_hears: "Every player online in the same zone" }
  - { channel: "Party", type: "#text", who_hears: "Every member of your party" }
  - { channel: "Whisper", type: "@name text", who_hears: "That one player (and you)" }
  - { channel: "Reply", type: "/r text", who_hears: "The last player who whispered to you" }
source:
  code:
    - module/src/chat.rs (send_chat, check_rate, expire_messages, my_chat)
    - godot/rust/src/net.rs (send_chat, poll_chat)
    - godot/scripts/chat_window.gd (say, add_message)
    - godot/scripts/online.gd
    - godot/scripts/ui/world_overlay.gd (bubble)
---
# Chat

The chat box sits at the bottom left. Press **Enter** to start typing and **Enter** again
to send; **Escape** stops typing. What you type starts with a sign that picks who hears it,
like in iROSE.

## Channels

- **Nearby** (no sign): heard by players in the same zone within **50 m** of you, and
  shown in a speech bubble over your head.
- **Shout** (`!hello`): heard by every player online in your zone. You can shout once
  every **5 seconds**.
- **Party** (`#hello`): heard by every member of your [[rules/party|party]], wherever they
  are. You need to be in a party.
- **Whisper** (`@Name hello`): heard only by that player. The name doesn't care about
  capital letters, and the player must be online. Your own copy shows as "[To *name*]", theirs
  as "[From *name*]".
- **Reply** (`/r hello`): whispers to the last player who whispered to you since you logged
  in.

The [[rules/friends|friends]] window's **Whisper** button fills in `@name ` for you.

Each channel has its own colour in the chat log. The log keeps the last **100** lines.

## Limits

- A message is at most **200** characters. Longer messages are refused. Control characters
  are removed and spaces at the start and end are trimmed; an empty message isn't sent.
- You can send at most **6 messages in 5 seconds**, counting all channels. The window
  starts with your first message and resets 5 seconds later. More than that is refused
  with "you are talking too fast".
- On top of that, only one shout every **5 seconds**.

## Speech bubbles

A nearby message appears in a bubble over the speaker's head for **6 seconds**, then fades
out. Bubbles can be turned off in the options ("Chat bubbles"). Shouts, party messages and
whispers don't make bubbles.

## How long messages are kept

The server keeps each message for **60 seconds**, then deletes it; the game client keeps
its own log. Only the players a message was meant for can read it, so nobody else can see
a whisper or a party message.

> Open question: typing `/r text` before anyone has whispered to you sends "/r text" as a nearby message instead of giving an error.
