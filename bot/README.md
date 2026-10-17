# Test bot

`rose-stdb-bot`, a headless client (Rust, SpacetimeDB SDK 2.10.2) for checking the server
without starting the game: it connects, plays one scripted check and prints a timeline of
what the server did. Its checks cover combat timing and skill casting, which are hard to
see by eye in the client.

## Run

```sh
cd bot
cargo run --release -- ws://127.0.0.1:3100 ranged
```

The bots sign in without an account, so use the dev server (`docker compose --profile dev
up -d`, port 3100) or a server with no account issuer (`ROSE_AUTH_ISSUER=` in `.env`).
The database is always `rose`.

| Mode | What it checks |
| --- | --- |
| `ranged` (default), `melee` | Fights the nearest monster. Moving mid-swing cancels it (no damage, cooldown refunded); moving right after a hit is free but keeps the cooldown. |
| `cast` | Skill cast cancelling: the grace window, a locked-in cast holding the next command, and going back to the attack after a skill. Needs Blessing in active skill slot 0, Berserk in slot 1 and plenty of MP; it waits for MP 500 while you set that up (`give_skill`, `set_hp_mp`). |
| `gcd` | The 0.25 s global cooldown between two skills (Blessing then Berserk, as in `cast`). |
| `flee` | A started swing always finishes: hits the nearest monster until it fights back, then keeps stepping away; every swing the monster starts should land. |
| `load N SECS` | Load test: N bots (default 50) moving and attacking at random for SECS seconds (default 60), then the worst gap between combat ticks. |

## Layout

- `src/main.rs`: every mode, one function each.
- `src/module_bindings/`: generated from `../module`; regenerate them after a table or
  reducer change (see `../module/README.md`).

The bot has its own Cargo workspace (its SDK pins differ from the client's), so it is
excluded from the root one.
