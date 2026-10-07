# ROSE server in Docker

Two containers: `rose-server` runs SpacetimeDB with the `rose` module, and `rose-web` is
the account website (`web/`, see web/README.md) where players sign up and reset their
password. It works the same on Windows, macOS and Linux (x86_64 or ARM): anything that
runs Docker.

## Start

Install Docker (Docker Desktop on Windows and macOS). The server reads its items, monsters,
skills, quests and zones from a ROSE client, so tell it where yours is: make a file named
`.env` in the repository folder with one line, the folder that has `data.idx` in it, for example

```
ROSE_CLIENT=C:\Games\iRose_129_129
```

Then, in the repository folder:

```sh
docker compose up -d
```

The first run builds the images (it compiles the module, a few minutes), creates a fresh
world and uploads the game data from that folder (the log shows `game data ready`). The
container only reads the folder.

Players make an account at `http://THIS-PC:3001` (on the same PC, `http://127.0.0.1:3001`),
then sign in to the game with that email and password; the game's start screen asks for
the server (`ws://THIS-PC:3000`) and the website address. Friends on the internet need TCP
ports 3000 and 3001 forwarded to this PC on the router; a rented server only needs them
open in its firewall. Set `ROSE_WEBSITE_URL` in `.env` to the address players use (for
example `ROSE_WEBSITE_URL=http://192.168.1.20:3001`), so the links in reset emails work.

Password reset emails need an SMTP account (your email provider's, or a service such as
Brevo or Mailgun). Put it in `.env`:

```
SMTP_HOST=smtp.example.com
SMTP_PORT=587
SMTP_USER=you@example.com
SMTP_PASS=the-smtp-password
MAIL_FROM=ROSE <you@example.com>
```

Without `SMTP_HOST` no email goes out: the reset link is written to the website's log
(`docker compose logs rose-web`), which is enough while testing. Never commit `.env`.

| Task | Command |
|---|---|
| Watch the log | `docker compose logs -f` |
| Stop / start again | `docker compose stop` / `docker compose start` |
| Install a new version of the module (keeps characters) | `docker compose up -d --build` |
| Upload the game data again (after changing the client files) | `ROSE_UPLOAD_GAME_DATA=1 docker compose up -d`, then `docker compose up -d` once more |
| Wipe the world (needed after a breaking schema change) | `ROSE_CLEAR_WORLD=1 docker compose up -d --build`, then `docker compose up -d` once more |
| Run admin commands | `docker compose exec rose-server sh -c 'HOME=/data/cli spacetimedb-cli call --server http://127.0.0.1:3000 rose reset_monsters'` |
| Give a character made before accounts to an account | `docker compose exec rose-server /opt/rose/assign-character.sh NAME EMAIL` (asks for the password) |
| Let anyone in without an account (load-test bots) | `ROSE_AUTH_ISSUER=` in `.env`, then `docker compose up -d` |

On Windows PowerShell the wipe is `$env:ROSE_CLEAR_WORLD=1; docker compose up -d --build`
followed by `Remove-Item Env:ROSE_CLEAR_WORLD; docker compose up -d`.

## Accounts

Characters made before accounts belong to the old identities and can't sign in any more;
`assign-character.sh` (above) moves one to an account that hasn't made a character yet.
With accounts on, a new account makes its character on the creation screen after
signing in. How the sign-in works is in web/README.md.

## What is kept

Accounts live in the `rose-web-data` volume (the accounts database and the key that signs
game tokens; if the key is lost, players just sign in again). Everything else lives in the
`rose-data` volume, so the containers can be rebuilt freely:

- `/data/db`: the world (characters with their levels, stats and items, and the uploaded game data).
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
no-op, changed it is an update that keeps the data. Then, if the module has no game data
yet (or `ROSE_UPLOAD_GAME_DATA=1`), it runs `upload-game-data.sh` on the `data.idx` it finds
in `/game`: the import tool packs the files the databases read (about 2,500 files, 33 MB for
129_129en) and the script sends them to the module's admin reducers in batches.

Tested 2026-10-07 on Linux (Docker 29): fresh start, a Godot client signed in and moved,
and after `docker restart` the same player came back with its name and position.
