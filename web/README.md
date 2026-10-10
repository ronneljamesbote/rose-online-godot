# ROSE account website

A small Next.js 16 site where players make an account (email and password), confirm their
email, sign in to see their account, reset a forgotten password by email, and see who is
playing, and it serves the game wiki at `/wiki`. The game signs in through it too: it sends the email and password to
`/api/game/login` and gets a short-lived token that the game server accepts.
`docker compose up -d` in the repository folder runs it on port 3001 next to the game
server (see server/README.md for the settings).

## Pages and endpoints

| Path | What it does |
|---|---|
| `/` | Start page with links to the forms |
| `/signup` | Make an account: email (one account per email) and a password, typed twice. Sends the confirmation email and signs you in |
| `/login` | Sign in with email and password; goes on to `/account` |
| `/account` | Signed in only: email with its confirmed state (and a button to send a new link), member since, password last changed; your character (name, level, job, whether it is in the game); change password; the devices signed in, with "Sign out other devices" |
| `/online` | Who's online: how many players are in the game, with each character's name, job and level. Anyone can open it, and it refreshes itself every 15 seconds |
| `/verify-email#token=...` | The page the confirmation email links to |
| `/forgot-password` | Sends a reset link to the email, if it has an account |
| `/reset-password#token=...` | The page the reset email links to: choose a new password |
| `POST /api/signup`, `/api/login`, `/api/logout`, `/api/verify-email`, `/api/forgot-password`, `/api/reset-password` | What the forms call (JSON) |
| `POST /api/account/password`, `/api/account/resend-verification`, `/api/account/sign-out-others` | The account page's buttons (signed in only) |
| `GET /api/online` | The who's-online list as JSON (count and players) |
| `POST /api/game/login` | `{email, password}` → `{token, expires_in}` for the game |
| `/.well-known/openid-configuration`, `/.well-known/jwks.json` | Where SpacetimeDB finds the public key that checks game tokens |
| `/wiki`, `/wiki/<page>` | The game wiki: the Markdown pages in the repository's `wiki/` folder, shown as web pages (see wiki/README.md). Anyone can read it |
| `GET /api/wiki/search` | Every wiki page's name, kind and ID, for the search box |
| `/wiki-assets/<file>` | The wiki's pictures (`wiki/assets`): zone maps and icon sheets |

The wiki is read from `WIKI_DIR` (default: `wiki/` next to the site, else `../wiki`). In
Docker, compose.yaml passes the repository's `wiki/` folder into the image, so a wiki change
shows after `docker compose build rose-web`. During `npm run dev` pages reload on save.
`node scripts/check-wiki.mjs` checks every page's data block and links.

The account and who's-online pages read the characters from the game server's database
(`GAME_SERVER_URL`, SQL over HTTP with the website's own service token; only public
tables are read).

## Security

- Passwords are hashed with scrypt (N=2^17, r=8, p=1, the OWASP setting) and a random salt
  per password; only the hash is stored. Comparing uses a constant-time check, and an
  unknown email costs the same hashing time as a wrong password.
- Emails are unique, ignoring case. Passwords need 8 to 128 characters (NIST SP 800-63B:
  length, no composition rules), and can't be the email.
- Reset links carry 32 random bytes; the database keeps only their SHA-256. A link works
  once, for one hour, and asking again cancels the earlier link. The token sits in the
  link's `#fragment`, which browsers never send to a server or put in a Referer, and the
  page removes it from the address bar. The forgot-password answer is the same whether or
  not the email has an account, and it answers before the email goes out, so its timing
  doesn't tell either.
- Rate limits (in memory): 10 sign-in tries per account and 100 per address every 15
  minutes, 3 reset emails per account and 30 per address an hour, 20 sign-ups per address
  an hour. Behind a reverse proxy set `TRUST_PROXY=1` so the limits see real addresses.
- Website sessions: signing in sets a cookie holding 32 random bytes; the database keeps
  only their SHA-256. The cookie is HttpOnly and SameSite=Lax, and on HTTPS it is Secure
  with the `__Host-` prefix. A session lasts 30 days from the last visit. Changing the
  password signs out every other device, and a password reset signs out all of them.
- Email confirmation: the confirmation link works like a reset link (32 random bytes,
  only the hash kept, 24 hours, fragment only). With `ROSE_REQUIRE_VERIFIED_EMAIL=1` the
  game refuses to sign in an account whose email isn't confirmed yet (the website still
  lets it in, to send a new link). It defaults to on when `SMTP_HOST` is set and off
  otherwise, since without email nobody could confirm.
- The forms only take JSON from pages of this site: the browser's `Origin` must be
  `PUBLIC_URL` or have the same host as the address the browser asked for (the `Host`
  header, or `X-Forwarded-Host` with `TRUST_PROXY=1`), so the site works whether it is
  opened as localhost, 127.0.0.1 or a LAN address, and a post from another site's page is
  refused.
  Pages send a Content-Security-Policy, `X-Frame-Options: DENY` and `Referrer-Policy:
  no-referrer`.
- Game tokens are ES256 JWTs signed with a key made on first start (in `DATA_DIR`, mode
  600), valid for 15 minutes, with the account id as the subject and `rose` as the
  audience. The game server only lets in connections whose token comes from this issuer
  (`module/src/account.rs`). The password never reaches the game server, and the game
  never saves it.
- Use HTTPS when the site is on the internet; without it, passwords cross the network
  readable. The compose file has a Caddy service for it (server/README.md, "HTTPS").

## Settings (environment)

| Variable | Default | Meaning |
|---|---|---|
| `PUBLIC_URL` | `http://127.0.0.1:3001` | Where players open the site; used in email links |
| `ROSE_AUTH_ISSUER` | `PUBLIC_URL` | Issuer in game tokens; the game server fetches its keys from here |
| `DATA_DIR` | `./data` | Accounts database (`accounts.sqlite`) and `signing-key.json`; keep private |
| `SMTP_HOST`, `SMTP_PORT`, `SMTP_USER`, `SMTP_PASS`, `MAIL_FROM` | unset, 587 | Email for confirmation and reset links; without a host the link is written to the log |
| `ROSE_REQUIRE_VERIFIED_EMAIL` | `1` with `SMTP_HOST`, else `0` | Game sign-in needs a confirmed email |
| `GAME_SERVER_URL` | `http://127.0.0.1:3000` | The game server's HTTP address, for the account and who's-online pages |
| `GAME_DATABASE` | `rose` | The game's database name on that server |
| `TRUST_PROXY` | `0` | `1` behind a reverse proxy: take the client address from `X-Forwarded-For` and the host from `X-Forwarded-Host` |

## Development

```sh
cd web
npm install
npm run dev        # http://127.0.0.1:3001
npm test           # password hashing and input rules
npm run typecheck
```

The accounts database uses Node's built-in `node:sqlite` (Node 22.13 or newer), so there
are no native modules to build. To try the game against a local server, tell it to trust
the dev site: `spacetimedb-cli call --server http://127.0.0.1:3000 rose set_auth_issuer
'"http://127.0.0.1:3001"'` (`'""'` turns accounts off again).
