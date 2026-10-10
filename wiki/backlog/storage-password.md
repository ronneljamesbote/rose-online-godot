---
kind: backlog
id: storage-password
name: Storage password
status: not-in-game-yet
summary: An optional second password that must be typed before the account storage opens
npcs:
  - "[[npcs/1004-ferrell-guild-staff-crow|Ferrell Guild Staff Crow]]"
  - "[[npcs/1180-ferrell-guild-banker-andre|Ferrell Guild Banker Andre]]"
source:
  data: LIST_STRING.STL strings 533-536
  reference: iROSE 129_129en client data
---
# Storage password

## How it works in iROSE 129

The 129 client has the messages for a storage password, a second password that protects
the account storage (the bank) even when someone else has the account password:

- Opening storage with a password set asks "Please enter your Storage Password." (533).
- A wrong password: "Incorrect Storage Password." (535); the storage stays closed.
- The owner can change it ("Your Storage Password has been changed.", 534) or remove it
  ("Your Storage Password has been deleted.", 536).

> Open question: whether the 129 server used this. The strings are in the client data, but
> the storage window (DlgBank.xml) has no password field, so the prompt would use a generic
> input box. Where a player sets the password (a storage NPC choice, a chat command or the
> account website) is not in the data, and neither are length limits or lockouts.

## What our game does today

No storage password. The storage (see [[rules/bank|Bank]]) opens at any storage NPC once the player is logged in.
Accounts are protected by the website login (see [[rules/accounts|Accounts]]).

## Building it

- **Server**: a hashed storage password per account; `bank_deposit` and `bank_withdraw` refuse until the right
  password was given in this session; set, change and clear reducers; attempt limits like
  the website's sign-in limits.
- **Client**: a password prompt before the storage window; a place to set it (the account
  website would be simplest).
- **Data**: none.
