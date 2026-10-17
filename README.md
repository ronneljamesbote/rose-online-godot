# ROSE Online on SpacetimeDB

A variant of ROSE Online with its own feature set: a SpacetimeDB server module that runs the
game rules, a Godot 4 client, an account website and a wiki that describes the game. The base
game data comes from the iROSE 129_129 client files; our changes to it live in `overrides/`.

## Quick start

| Want to | Do | More |
| --- | --- | --- |
| Run the server | Put `ROSE_CLIENT=<folder with data.idx>` in `.env`, then `docker compose up -d`. Game on port 3000, website on 3001. | [`server/README.md`](server/README.md) |
| Run the dev server (GM commands, no accounts) | `docker compose --profile dev up -d`, port 3100 | [`server/README.md`](server/README.md#dev-server) |
| Build the Windows client | `godot/tools/build-windows.ps1` on Windows or `godot/tools/build-windows.sh` on Linux; `-Dev` / `--dev` for the dev client | [`godot/README.md`](godot/README.md#windows-build) |
| Run the client from source | Build the `rose-gd` extension, then `godot --path godot -- --data-idx=...` | [`godot/README.md`](godot/README.md#build-and-run) |
| Change the game | Edit the wiki page first; the code follows it | [`wiki/README.md`](wiki/README.md), [`AGENTS.md`](AGENTS.md) |

## Layout

| Folder | What it is |
| --- | --- |
| [`module/`](module/README.md) | The SpacetimeDB server module (Rust, wasm32): every game rule, table and reducer |
| [`godot/`](godot/README.md) | The Godot 4 client, with its Rust GDExtension in `godot/rust/` |
| [`web/`](web/README.md) | The account website (Next.js): sign-up, sign-in for the game, the wiki at `/wiki` |
| [`wiki/`](wiki/README.md) | The game as it runs: rules, skills, monsters, NPCs, quests, items, zones |
| [`overrides/`](overrides/README.md) | Our changes to the iROSE data, plus the zones and hit markers exported from Godot |
| [`server/`](server/README.md) | Docker images and scripts for the server, the website and HTTPS |
| [`crates/`](crates/UPSTREAM.md) | rose-offline's file readers, game data and game rules, vendored and changed |
| `import/` | `rose-stdb-import`: packs the client files the server reads for upload (see [`server/README.md`](server/README.md#how-it-works)) |
| [`bot/`](bot/README.md) | A headless test client for quick server checks |
| [`tools/wiki-gen/`](tools/wiki-gen/README.md) | Wrote the first draft of the wiki's skill, monster, NPC, quest, item and zone pages from the game data |
| `rose-client/` | Empty; the default ROSE client folder `compose.yaml` mounts |

## Where the docs are

- **How the game plays** (formulas, numbers, what each skill or NPC does): `wiki/`, shown on
  the website at `/wiki`. The wiki is the spec; the code follows it.
- **How the code works and how to build it**: the README in each folder above.
- **Rules for working in this repository** (wiki sync, where game values come from):
  [`AGENTS.md`](AGENTS.md).
