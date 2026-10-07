# ROSE server in Docker

One container runs SpacetimeDB with the `rose` module. It works the same on Windows,
macOS and Linux (x86_64 or ARM): anything that runs Docker.

## Start

Install Docker (Docker Desktop on Windows and macOS). Then, in the repository folder:

```sh
docker compose up -d
```

The first run builds the image (it compiles the module, a few minutes) and creates a fresh
world with Zant's monsters. Players connect to `ws://THIS-PC:3000`; on the same PC that is
`ws://127.0.0.1:3000`. Friends on the internet need TCP port 3000 forwarded to this PC on
the router; a rented server only needs port 3000 open in its firewall.

| Task | Command |
|---|---|
| Watch the log | `docker compose logs -f` |
| Stop / start again | `docker compose stop` / `docker compose start` |
| Install a new version of the module (keeps characters) | `docker compose up -d --build` |
| Wipe the world (needed after a breaking schema change) | `ROSE_CLEAR_WORLD=1 docker compose up -d --build`, then `docker compose up -d` once more |
| Run admin commands | `docker compose exec rose-server sh -c 'HOME=/data/cli spacetimedb-cli call --server http://127.0.0.1:3000 rose reset_monsters'` |

On Windows PowerShell the wipe is `$env:ROSE_CLEAR_WORLD=1; docker compose up -d --build`
followed by `Remove-Item Env:ROSE_CLEAR_WORLD; docker compose up -d`.

## What is kept

Everything lives in the `rose-data` volume, so the container can be rebuilt freely:

- `/data/db`: the world (characters, positions, HP).
- `/data/keys`: the key pair that signs player identities, made on the first start. If it
  is lost, every player gets a new character.
- `/data/cli`: the CLI login that published the module, which is the admin identity.

Moving to another machine: copy the volume, for example
`docker run --rm -v rose-stdb_rose-data:/data -v "$PWD":/out ubuntu tar czf /out/rose-data.tgz -C /data .`
and unpack it into the new machine's volume the same way. (The volume is named after the
folder, `rose-stdb_rose-data` here; `docker volume ls` shows it.)

## How it works

`server/Dockerfile` builds the module in a Rust stage, then puts SpacetimeDB 2.10.2's Linux
binaries and the module on Ubuntu 24.04. `server/entrypoint.sh` makes the key pair on the
first start, starts SpacetimeDB, and publishes the module on every start: unchanged it is a
no-op, changed it is an update that keeps the data.

Tested 2026-10-07 on Linux (Docker 29): fresh start, a Godot client signed in and moved,
and after `docker restart` the same player came back with its name and position.
