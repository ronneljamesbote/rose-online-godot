// Reads the game server (SpacetimeDB's HTTP SQL API) for the who's-online page and the
// account page. Only public tables are read, with the website's own service token.

import { GAME_DATABASE, GAME_SERVER_URL } from "./config";
import { gameToken, serviceToken } from "./keys";
import { setGameIdentity, type Account } from "./db";

export interface Character {
  name: string;
  job: number;
  className: string;
  level: number;
  online: boolean;
  gender: "male" | "female";
}

/** iROSE jobs (LIST_CLASS.STB): first job at level 10, second at 70. */
const CLASSES: Record<number, string> = {
  0: "Visitor",
  111: "Soldier",
  121: "Knight",
  122: "Champion",
  211: "Muse",
  221: "Mage",
  222: "Cleric",
  311: "Hawker",
  321: "Raider",
  322: "Scout",
  411: "Dealer",
  421: "Bourgeois",
  422: "Artisan",
};

export function className(job: number): string {
  return CLASSES[job] ?? "Visitor";
}

let cachedToken: { token: string; until: number } | null = null;

async function token(): Promise<string> {
  if (!cachedToken || cachedToken.until < Date.now()) {
    cachedToken = { token: await serviceToken(), until: Date.now() + 5 * 60 * 1000 };
  }
  return cachedToken.token;
}

type Row = unknown[];

async function sql(query: string): Promise<Row[]> {
  const response = await fetch(`${GAME_SERVER_URL}/v1/database/${encodeURIComponent(GAME_DATABASE)}/sql`, {
    method: "POST",
    headers: { authorization: `Bearer ${await token()}`, "content-type": "text/plain" },
    body: query,
    cache: "no-store",
    signal: AbortSignal.timeout(5000),
  });
  if (!response.ok) throw new Error(`game server answered ${response.status}: ${(await response.text()).slice(0, 200)}`);
  const result = (await response.json()) as { rows: Row[] }[];
  return result[0]?.rows ?? [];
}

function toCharacter(row: Row): Character {
  const [name, job, level, online, gender] = row as [string, number, number, boolean, number];
  return { name, job, className: className(job), level, online, gender: gender === 1 ? "female" : "male" };
}

const COLUMNS = "name, job, level, online, gender";

let onlineCache: { at: number; players: Character[] } | null = null;

/** Characters in the game right now, highest level first. Kept for 10 seconds, so a busy
 * page doesn't load the game server. Null when the game server can't be reached. */
export async function onlinePlayers(): Promise<Character[] | null> {
  if (onlineCache && Date.now() - onlineCache.at < 10_000) return onlineCache.players;
  try {
    const players = (await sql(`SELECT ${COLUMNS} FROM player WHERE online = true`)).map(toCharacter);
    players.sort((a, b) => b.level - a.level || a.name.localeCompare(b.name));
    onlineCache = { at: Date.now(), players };
    return players;
  } catch (error) {
    console.error("[game] reading who is online failed:", error instanceof Error ? error.message : error);
    return null;
  }
}

/** The account's SpacetimeDB identity: the game server works it out from the token's
 * issuer and subject, so ask it once and keep the answer. */
async function identityOf(account: Account): Promise<string> {
  if (account.game_identity) return account.game_identity;
  const response = await fetch(`${GAME_SERVER_URL}/v1/identity/websocket-token`, {
    method: "POST",
    headers: { authorization: `Bearer ${await gameToken(account.id)}` },
    cache: "no-store",
    signal: AbortSignal.timeout(5000),
  });
  if (!response.ok) throw new Error(`game server answered ${response.status}`);
  const { token: shortToken } = (await response.json()) as { token: string };
  const payload = JSON.parse(Buffer.from(shortToken.split(".")[1], "base64url").toString("utf8")) as { hex_identity?: string };
  const identity = payload.hex_identity ?? "";
  if (!/^[0-9a-f]{64}$/.test(identity)) throw new Error("the game server gave no identity");
  setGameIdentity(account.id, identity);
  return identity;
}

/** The account's character; null if it has none yet, "unavailable" if the game server
 * can't be reached. */
export async function characterOf(account: Account): Promise<Character | null | "unavailable"> {
  try {
    const identity = await identityOf(account);
    const rows = await sql(`SELECT ${COLUMNS} FROM player WHERE identity = 0x${identity}`);
    return rows.length ? toCharacter(rows[0]) : null;
  } catch (error) {
    console.error("[game] reading the character failed:", error instanceof Error ? error.message : error);
    return "unavailable";
  }
}
