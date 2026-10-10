---
kind: backlog
id: storage-fees
name: Storage fees
status: not-in-game-yet
summary: Putting an item in storage costs Zulie per item, cheap for gear and dearer for goods whose price moves
npcs:
  - "[[npcs/1004-ferrell-guild-staff-crow|Ferrell Guild Staff Crow]]"
  - "[[npcs/1089-manager-of-ferrell-arothel|Manager of Ferrell Arothel]]"
  - "[[npcs/1180-ferrell-guild-banker-andre|Ferrell Guild Banker Andre]]"
  - "[[npcs/1222-storage-keeper-dustin-leta|Storage Keeper Dustin Leta]]"
  - "[[npcs/1161-clan-base-camp-manager-kushard|Clan Base Camp Manager Kushard]]"
source:
  data: item tables column 5 (base price) and column 6 (price change rate); LIST_STRING.STL strings 29, 330-334; LIST_USEITEM.STB row 952 (ability 94, No Storage Charge); Crow's dialog
  reference: iROSE 129_129en client data; module/src/bank.rs; AbilityType::BankFree in crates/rose-game-irose/src/data/ability_values.rs
---
# Storage fees

## How it works in iROSE 129

Storage (the bank) is shared by all characters of an account. Putting an item **in**
costs a fee; taking it out is free.

- The storage window shows the "Storage Fee for each item" (string 333).
- The fee depends on the kind of item. [[npcs/1004-ferrell-guild-staff-crow|Crow]] explains:
  "For items with steady prices, such as weapons and equipment, the storage fee is cheap.
  On the contrary, items with fluctuating prices, such as materials or usable items will
  require a more expensive storage fee", to stop hoarding. In the data, equipment has a
  price change rate (item column 6) of 0, while many materials, consumables and gems have a
  rate of 1 to 10, so the fee is most likely built from the base price and the price change
  rate.
- For a stack the fee is paid for each item moved.
- Not enough Zulie: "You do not have enough Zulie to use the Storage." (330).
- Zulie itself cannot be stored: "Cannot place Zulie in Storage." (29); Crow: "we do not
  hold your money in Storage".
- A [[items/consumable/952-free-storage-coupon|Free Storage Coupon]] removes the fee for 7
  days (ability 94, "No Storage Charge"); see [[backlog/time-coupons|Time coupons]].

> Open question: the exact fee formula. It is not in the data files. It is known to use the
> base price, the price change rate and the count; check the original client (the fee is
> shown before depositing) or rose-next before building.

Other storage messages: "You cannot put this item in Storage." (331) for items whose trade
restriction forbids it, "There is not enough space in your Storage for this item." (332).

## What our game does today

`bank_deposit` in `module/src/bank.rs` is free (see [[rules/bank|Bank]]). Everything else matches: 4 pages of 30
slots shared per account (in iROSE the fourth page needs a premium account or a
[[items/consumable/953-storage-expansion-coupon|Storage Expansion Coupon]]; our game gives
it to everyone).

## Building it

- **Server**: compute the fee in `bank_deposit` and take it from the player's Zulie, unless
  the player has an active No Storage Charge coupon.
- **Client**: show the fee per item in the storage window and ask before depositing.
- **Data**: none to change, unless we want different fees (that would be an override of
  the price change rate, which also moves shop prices, so a code constant may be better).
