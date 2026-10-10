---
kind: backlog
id: time-coupons
name: Time coupons
status: not-in-game-yet
summary: Mall coupons that switch on a timed account perk, free storage, 30 extra storage slots or a personal shop look, for 7 days
source:
  data: LIST_USEITEM.STB rows 952-956 (item class Time Coupon; column 21 ability, column 22 value, column 8 shop look); abilities 94 No Storage Charge, 95 Storage Expansion, 96 Personal Shop Remodeling; LIST_USEITEM_S.STL; LIST_STRING.STL strings 608, 614, 618
  reference: iROSE 129_129en client data; AbilityType::BankFree, BankAddon and StoreSkin in crates/rose-data-irose/src/data_decoder.rs; module/src/items.rs
---
# Time coupons

## How it works in iROSE 129

Time coupons are consumables (item class "Time Coupon") sold in the item mall and received
through the [[backlog/item-delivery|item delivery]] storage. Using one starts a perk that
lasts a fixed time; the item is used up.

| Coupon | Ability (column 21) | Value (column 22) | Effect (item text) |
| --- | --- | --- | --- |
| [[items/consumable/952-free-storage-coupon\|Free Storage Coupon]] | 94 No Storage Charge | 1008 | no [[backlog/storage-fees\|storage fees]] for 7 days |
| [[items/consumable/953-storage-expansion-coupon\|Storage Expansion Coupon]] | 95 Storage Expansion | 1008 | 30 more storage slots (page 4) for 7 days |
| [[items/consumable/954-special-shop-a-coupon\|Special Shop A Coupon]] | 96 Personal Shop Remodeling | 1440 | personal shop look 1 for 7 days |
| [[items/consumable/955-special-shop-b-coupon\|Special Shop B Coupon]] | 96 | 1440 | personal shop look 3 for 7 days |
| [[items/consumable/956-special-shop-c-coupon\|Special Shop C Coupon]] | 96 | 1440 | personal shop look 2 for 7 days |

The shop look number is the coupon's column 8 (read as `store_skin`). With a look active,
the [[rules/personal-shops|personal shop]] sign over the seller is drawn with that design
instead of the plain one.

- The perk shows with its "Expiration Date" (string 618).
- Storage page 4 without the perk: "This tab is only for Platinum users." (608). Platinum was
  iROSE's paid account level (strings 590-609), and Platinum accounts had page 4 anyway.
- Using a coupon again while the perk runs most likely extends it.

> Open question: the time unit of column 22. 1008 is exactly 7 days in 10-minute units
> (the unit that also makes the Clan House skill's 4320 equal 30 days), but 1440 would then
> be 10 days while the shop coupons' text says 7 days. Check the original server before
> building.

> Open question: whether using a coupon again adds time or restarts it.

## What our game does today

Using a time coupon is refused with "that can't be used yet" (`module/src/items.rs`).
Storage page 4 is open to everyone (see [[rules/bank|Bank]]), so the expansion coupon would
have nothing to unlock unless page 4 goes back to being a perk. Storage is free, so the free
storage coupon has nothing to remove until [[backlog/storage-fees|storage fees]] exist.

## Building it

- **Server**: a per-account (or per-character) perk table with an end time; using a coupon
  sets or extends it; storage fees skip the fee while ability 94 is active; page 4 opens while
  ability 95 is active (if page 4 becomes a perk); the shop look is sent with the personal
  shop while ability 96 is active.
- **Client**: show active perks and their end dates; draw the three shop looks.
- **Data**: none, unless we change durations (an override of column 22).
