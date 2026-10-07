// The key that signs game tokens. It is made on first start and kept in DATA_DIR; the game
// server checks tokens against the public half, served at /.well-known/jwks.json.

import fs from "node:fs";
import path from "node:path";
import { calculateJwkThumbprint, exportJWK, generateKeyPair, importJWK, SignJWT, type JWK, type CryptoKey } from "jose";
import { AUTH_AUDIENCE, AUTH_ISSUER, DATA_DIR, GAME_TOKEN_SECONDS } from "./config";

interface SigningKey {
  kid: string;
  privateKey: CryptoKey;
  publicJwk: JWK;
}

const ALG = "ES256";
const globalForKey = globalThis as unknown as { roseKey?: Promise<SigningKey> };

async function load(): Promise<SigningKey> {
  const file = path.join(DATA_DIR, "signing-key.json");
  let privateJwk: JWK;
  if (fs.existsSync(file)) {
    privateJwk = JSON.parse(fs.readFileSync(file, "utf8"));
  } else {
    const pair = await generateKeyPair(ALG, { extractable: true });
    privateJwk = await exportJWK(pair.privateKey);
    fs.mkdirSync(DATA_DIR, { recursive: true, mode: 0o700 });
    fs.writeFileSync(file, JSON.stringify(privateJwk), { mode: 0o600, flag: "wx" });
  }
  const { d: _d, ...publicPart } = privateJwk;
  const kid = await calculateJwkThumbprint(publicPart);
  const privateKey = (await importJWK(privateJwk, ALG)) as CryptoKey;
  return { kid, privateKey, publicJwk: { ...publicPart, kid, alg: ALG, use: "sig" } };
}

function signingKey(): Promise<SigningKey> {
  globalForKey.roseKey ??= load();
  return globalForKey.roseKey;
}

export async function jwks(): Promise<{ keys: JWK[] }> {
  return { keys: [(await signingKey()).publicJwk] };
}

/** A token the game client connects with. The subject is the account id, so SpacetimeDB
 * gives every account the same identity each time. */
export async function gameToken(accountId: string): Promise<string> {
  const key = await signingKey();
  return new SignJWT({})
    .setProtectedHeader({ alg: ALG, kid: key.kid, typ: "JWT" })
    .setIssuer(AUTH_ISSUER)
    .setSubject(accountId)
    .setAudience(AUTH_AUDIENCE)
    .setIssuedAt()
    .setExpirationTime(`${GAME_TOKEN_SECONDS}s`)
    .sign(key.privateKey);
}
