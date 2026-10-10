---
kind: rule
id: trade
name: Trade
status: in-game
max_items_per_side: 10
trade_range_m: 15
request_expires_s: 30
steps:
  - { step: 1, what: "Ask: use Trade on a player within 15 m" }
  - { step: 2, what: "They accept the request; the trade window opens for both" }
  - { step: 3, what: "Each side puts up to 10 bag items and some Zuly on the table" }
  - { step: 4, what: "Each side locks their offer" }
  - { step: 5, what: "Each side presses Trade; when both have, the items and Zuly swap" }
source:
  code:
    - module/src/trade.rs (trade_ask, trade_answer, check_offer, trade_offer, trade_lock, trade_accept, take_offer, give, exchange, trade_cancel, cancel_for, expire_requests)
    - module/src/shop.rs (exchangeable)
    - module/src/lib.rs (client_disconnected)
    - godot/scripts/trade_window.gd
  data: LIST_*.STB item column 3 (trade restriction, bit 2 = can't be traded)
---
# Trade

Two players standing near each other can swap items and Zuly through the trade window.
Both sides see what the other offers, and nothing changes hands until both have locked
their offer and pressed **Trade**.

## Starting a trade

- Right-click another player and pick **Trade**, or use the [[skills/20-trade|Trade]]
  action on them. They must be online, in the same zone and within **15 m** of you.
- They see "*name* wants to trade" and can accept or decline. A request that isn't
  answered within **30 seconds** is gone.
- You can't trade with yourself, with a player who is already trading, or while either of
  you has a [[rules/personal-shops|personal shop]] open. You must still be within 15 m when
  the request is accepted.

## Putting items on the table

- Each side can offer up to **10** bag items and any amount of Zuly they carry.
- For a stack, you choose how many to offer (Shift + right-click offers one).
- Items marked as not tradeable in the item data can't be offered.
- Every change to an offer **unlocks both sides** and clears both acceptances, so nobody
  can be tricked by an offer that changes at the last moment.

## Locking and accepting

1. When you are happy with your offer, press **Lock**. Unlocking it again (to change it)
   also clears both acceptances.
2. When both offers are locked, each side presses **Trade**.
3. When both have pressed Trade, the swap happens at once.

At that moment the game checks again that:

- the two players are still within 15 m of each other in the same zone,
- both still have every offered item and the offered Zuly,
- each bag has room for what it receives, and nobody goes over the Zuly they can carry.

If any check fails, nothing changes hands, the trade window closes and both players see
"The trade failed" with the reason.

## Ending a trade

The trade ends with nothing exchanged when:

- either player presses **Cancel** (the other sees "*name* stopped trading"),
- either player walks more than 15 m away (the game client cancels it),
- either player logs out.

While trading you can't open a personal shop, buy from or sell to a shop, or get on
someone's cart.

> Open question: the server only checks the 15 m range when the swap happens; walking away cancels the trade only in the game client, and a teleport or warp does not end the trade on the server.
