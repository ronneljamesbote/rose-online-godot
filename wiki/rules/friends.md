---
kind: rule
id: friends
name: Friends
status: in-game
max_friends: 35
request_expires_s: 30
window_key: F
source:
  code:
    - module/src/friends.rs (friend_ask, friend_ask_entity, ask, friend_answer, answer, friend_remove, announce, expire_requests, my_friends, my_friend_requests, rekey)
    - module/src/lib.rs (client_connected, client_disconnected)
    - module/src/skills.rs (basic command Add Friend)
    - godot/scripts/friends_window.gd
---
# Friends

Your friends list (press **F**) shows the players you are friends with, whether they are
online, and where. A friendship is always two-way: you are on each other's lists.

## Adding a friend

- Type a player's name in the friends window, right-click a player and pick **Add
  friend**, or use the [[skills/18-add-friend|Add Friend]] action on a player you have
  selected (if it is on your skill list). The name is
  matched without caring about capital letters, and the player must be **online**.
- They get a message "*name* wants to be friends" and a request to accept or decline.
  A request that isn't answered within **30 seconds** is gone.
- If two players ask each other, the second request counts as a yes and they become
  friends straight away.
- Asking the same player again replaces your earlier request.
- You can't add yourself or someone who is already your friend.

When the request is accepted, both players are added to each other's lists and both see
"*name* is now your friend". A declined request tells the asker "*name* declined your friend
request".

## Limits

Each player can have up to **35** friends. You can't send a request when your list is
full, and a request can't be accepted when either list is full.

## What the list shows

For each friend: their name, whether they are online, their level and job, and the zone
they are in (only while they are online). When a friend logs in or out you see
"*name* is online" or "*name* went offline".

Only you can see your list and the requests sent to you.

## Talking to friends

The **Whisper** button (only for friends who are online) starts a whisper to them, the
same as typing `@name` in the chat box. See [[rules/chat|Chat]].

## Removing a friend

Press **Remove** on a friend's row. They are taken off your list and you are taken off
theirs. You see "Removed *name* from your friends" and they see "*name* removed you from
their friends".
