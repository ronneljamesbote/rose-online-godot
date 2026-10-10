---
kind: backlog
id: item-delivery
name: Item delivery (mall storage)
status: not-in-game-yet
summary: An account storage at the Ferrell storage keeper that holds items bought from the item mall, which can be taken out or sent to another character
npcs:
  - "[[npcs/1004-ferrell-guild-staff-crow|Ferrell Guild Staff Crow]]"
source:
  data: 3DDATA/CONTROL/XML DeliveryStore.xml; LIST_STRING.STL strings 581-589, 614; NPC dialog of Crow (GF_openDeliveryStore)
  reference: iROSE 129_129en client data and TRose.exe strings (DeliveryStore); godot/rust/src/conversation.rs
---
# Item delivery (mall storage)

## How it works in iROSE 129

iROSE sold items on its website (the item mall, paid with mileage points). Bought items did
not appear in the game straight away: they went to a per-account **Mileage Item Storage**
(string 614), opened from a storage NPC.

- [[npcs/1004-ferrell-guild-staff-crow|Ferrell Guild Staff Crow]] has an unnamed choice
  next to "I'd like to use [My Storage]" that calls `GF_openDeliveryStore`, which opens the
  delivery window (DeliveryStore.xml).
- The window looks like the storage window with a single tab of item slots.
- **Taking an item**: move it to the inventory. "Failed to bring item." (587) when that does
  not work (for example a full inventory).
- **Sending an item** (the Gift button): the Item Transfer window (589) has a Receiver field
  (581, a character name) and Items to send (582). The server checks the name ("The character
  name does not exist.", 583; "Incorrect target.", 584) and moves the item into that
  character's delivery storage, showing the Sender (588). Results: "Item transfer has
  successfully completed." (585) or "Item transfer has failed." (586).
- Items in this storage are the mall items: time coupons (see
  [[backlog/time-coupons|Time coupons]]), the salon, plastic surgery and gender change
  coupons used by [[npcs/1010-designer-keenu|Designer Keenu]], boosters and similar.

> Open question: the number of slots, whether the receiver must be on the same account or
> any account, and whether delivered items expire. None of this is in the client data.

## What our game does today

There is no item mall and no delivery storage. Crow's choice shows "That service is not
in the game yet".

## Building it

- **Server**: a `delivery` table (account, item, sender); reducers to take an item into the
  inventory and to send one to a named character; a way for the website or an admin to put
  items in (our game has no mall, so this is the only source).
- **Client**: the delivery window and the Item Transfer window, opened by
  `GF_openDeliveryStore` in `godot/rust/src/conversation.rs`.
- **Website**: optionally a page that grants rewards into the delivery storage.

> Open question: whether we want an item mall at all. Without one, this storage is only
> useful as a mailbox for rewards and gifts.
