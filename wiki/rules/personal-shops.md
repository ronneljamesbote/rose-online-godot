---
kind: rule
id: personal-shops
name: Personal shops
status: changed-from-irose
max_items_for_sale: 30
max_items_on_buy_list: 30
max_title_characters: 50
max_stack_on_buy_list: 999
max_price_per_item: 4294967295
buy_range_m: 20
browse_window_closes_beyond_m: 18
search_min_letters: 2
source:
  code:
    - module/src/shop.rs (store_open, store_buy, store_sell, store_close, close, exchangeable, near_store)
    - module/src/lib.rs (stand_up, despawn, move and attack checks)
    - module/src/world.rs (teleport closes the shop)
    - module/src/vehicle.rs (no getting on a cart with a shop open)
    - module/src/trade.rs (no trading with a shop open)
    - godot/scripts/shop_window.gd
    - godot/scripts/online.gd (select, player menu)
  data: "item STBs column 3 (trade restriction: items that can't be exchanged)"
---
# Personal shops

Any player can sell items to others, and buy from them, by opening a personal shop with
the [[skills/21-vending|Vending]] action. The shop owner sits down and the shop's title
shows over them. Other players click them to see what is for sale.

## Opening a shop

1. Use **Vending** from the skill window or the hotbar. The setup window opens.
2. Tick the bag items to sell, how many of each (for stacks) and the price for one.
   Up to **30** items.
3. Optionally add items to the **buy list** (below).
4. Give the shop a title (up to **50** characters; empty gives "<your name>'s shop") and
   open it.

A shop needs at least one item to sell or one item to buy. Opening it stops what you are
doing (moving, attacking, casting) and sits you down.

You can't open a shop:

- while dead, stunned or otherwise unable to act, or while using a skill;
- while driving or riding a cart (get off first);
- in a PvP zone (see [[rules/pvp|PvP]]);
- while trading.

Items that can't be traded can't be sold or bought (see the item's `trade` field). Every
price must be at least 1 Zuly and at most 4,294,967,295 Zuly per item.

## While the shop is open

The owner can't move, attack, use skills (except sitting), trade or get on a cart; the
game says "close your shop first". The shop closes when the owner:

- stands up (X, the Sit action, or the Close button);
- dies, leaves the game or changes zone;
- has nothing left to sell or buy ("Your shop sold out" or "Your shop has nothing left to
  sell or buy").

The listed items stay in the owner's bag until someone buys them. If the owner no longer
has an item when someone tries to buy it, that item leaves the shop as sold out.

## Buying from a shop

Left-click a player with a shop title, or right-click them and pick the shop entry. Pick
how many and press **Buy**. You must be within **20 m** of the shop; the window closes
when you walk more than 18 m away. You can't buy from your own shop.

Each purchase is checked again at that moment: the owner must still have the item, you
must have the Zuly and the bag space, and the owner must be able to carry the Zuly. The
Zuly goes straight to the owner, and both of you get a notice.

## Buy list (the shop buys from others)

Under the sell list in the setup window is the **Buy list**. Type at least 2 letters of
an item name, press **Add** on a match, then set how many you want and the price for one.

- Up to **30** entries. Stackable items (potions, materials, gems, quest items) go up to
  999 each; equipment is one at a time.
- The same item can't be on the list twice at different prices.
- A shop can sell only, buy only, or both.

Other players see a **Buying** section in your shop. If they carry a matching item it
shows how many and a **Sell** button; otherwise it says they have none. When they sell:

- the owner pays at that moment, so the sale fails if the owner no longer has the Zuly or
  the bag space;
- broken equipment (life 0) can't be sold;
- the same 20 m range applies, and you can't sell to your own shop;
- each sale lowers the amount wanted, and the entry goes away at 0.

## Changed from iROSE

- iROSE has no range check for buying; here buyers must be within 20 m, so only people
  who can see the shop can use it.
