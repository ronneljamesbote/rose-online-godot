// Settings from the environment (see web/README.md).

import path from "node:path";

/** Where people reach the website, for links in emails. */
export const PUBLIC_URL = (process.env.PUBLIC_URL ?? "http://127.0.0.1:3001").replace(/\/+$/, "");

/** The JWT issuer the game server trusts. SpacetimeDB fetches
 * ISSUER/.well-known/openid-configuration from it, so it must be reachable from there. */
export const AUTH_ISSUER = (process.env.ROSE_AUTH_ISSUER ?? PUBLIC_URL).replace(/\/+$/, "");

/** Audience of game tokens; the game server checks it. */
export const AUTH_AUDIENCE = "rose";

/** Accounts database and the token signing key live here. Never commit this folder. */
export const DATA_DIR = path.resolve(/* turbopackIgnore: true */ process.env.DATA_DIR ?? "data");

/** Game tokens are only used to connect, so they are short lived. */
export const GAME_TOKEN_SECONDS = 15 * 60;

export const RESET_TOKEN_MINUTES = 60;

export const VERIFY_TOKEN_HOURS = 24;

/** Signed-in website sessions last this long without a visit. */
export const SESSION_DAYS = 30;

/** Cookies get the Secure flag (and the __Host- prefix) when the site is on HTTPS. */
export const SECURE_COOKIES = PUBLIC_URL.startsWith("https://");

/** Accounts must confirm their email before playing. Defaults to on when email can be sent
 * (SMTP_HOST set), since without email nobody could confirm. Empty counts as unset, as
 * compose passes the variable through even when .env leaves it out. */
export const REQUIRE_VERIFIED_EMAIL =
  (process.env.ROSE_REQUIRE_VERIFIED_EMAIL || (process.env.SMTP_HOST ? "1" : "0")) === "1";

/** The game server's HTTP address and database, for the who's-online list and the
 * character shown on the account page. */
export const GAME_SERVER_URL = (process.env.GAME_SERVER_URL ?? "http://127.0.0.1:3000").replace(/\/+$/, "");
export const GAME_DATABASE = process.env.GAME_DATABASE ?? "rose";
