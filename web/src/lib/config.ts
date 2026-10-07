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
