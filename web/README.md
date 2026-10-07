# ROSE account website

A small Next.js 16 site where players make an account (email and password), and reset a
forgotten password by email. The game signs in through it too: it sends the email and
password to `/api/game/login` and gets a short-lived token that the game server accepts.
`docker compose up -d` in the repository folder runs it on port 3001 next to the game
server (see server/README.md for the settings).

## Pages and endpoints

| Path | What it does |
|---|---|
| `/` | Start page with links to the two forms |
| `/signup` | Make an account: email (one account per email) and a password, typed twice |
| `/forgot-password` | Sends a reset link to the email, if it has an account |
| `/reset-password#token=...` | The page the email links to: choose a new password |
| `POST /api/signup`, `/api/forgot-password`, `/api/reset-password` | What the forms call (JSON) |
| `POST /api/game/login` | `{email, password}` → `{token, expires_in}` for the game |
| `/.well-known/openid-configuration`, `/.well-known/jwks.json` | Where SpacetimeDB finds the public key that checks game tokens |

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
- The forms only take JSON from this site: a post from another site's page is refused.
  Pages send a Content-Security-Policy, `X-Frame-Options: DENY` and `Referrer-Policy:
  no-referrer`.
- Game tokens are ES256 JWTs signed with a key made on first start (in `DATA_DIR`, mode
  600), valid for 15 minutes, with the account id as the subject and `rose` as the
  audience. The game server only lets in connections whose token comes from this issuer
  (`module/src/account.rs`). The password never reaches the game server, and the game
  never saves it.
- Use HTTPS when the site is on the internet (a reverse proxy such as Caddy in front of
  port 3001); without it, passwords cross the network readable.

## Settings (environment)

| Variable | Default | Meaning |
|---|---|---|
| `PUBLIC_URL` | `http://127.0.0.1:3001` | Where players open the site; used in email links |
| `ROSE_AUTH_ISSUER` | `PUBLIC_URL` | Issuer in game tokens; the game server fetches its keys from here |
| `DATA_DIR` | `./data` | Accounts database (`accounts.sqlite`) and `signing-key.json`; keep private |
| `SMTP_HOST`, `SMTP_PORT`, `SMTP_USER`, `SMTP_PASS`, `MAIL_FROM` | unset, 587 | Email for reset links; without a host the link is written to the log |
| `TRUST_PROXY` | `0` | `1` to take the client address from `X-Forwarded-For` |

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
