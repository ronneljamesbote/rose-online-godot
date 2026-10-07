// Password hashing with scrypt (memory-hard, built into Node, no native modules to build).
// Parameters follow OWASP's scrypt recommendation: N=2^17, r=8, p=1. Each hash is stored
// with its own random salt and its parameters, so they can be raised later.

import { randomBytes, scrypt, timingSafeEqual, type ScryptOptions } from "node:crypto";

const N = 2 ** 17;
const R = 8;
const P = 1;
const KEY_LENGTH = 64;
const SALT_LENGTH = 16;

function derive(password: string, salt: Buffer, n: number, r: number, p: number): Promise<Buffer> {
  const options: ScryptOptions = { N: n, r, p, maxmem: 256 * n * r + 1024 * 1024 };
  return new Promise((resolve, reject) =>
    scrypt(password.normalize("NFKC"), salt, KEY_LENGTH, options, (error, key) => (error ? reject(error) : resolve(key))),
  );
}

/** "scrypt$N$r$p$salt$hash", salt and hash in base64url. */
export async function hashPassword(password: string): Promise<string> {
  const salt = randomBytes(SALT_LENGTH);
  const key = await derive(password, salt, N, R, P);
  return ["scrypt", N, R, P, salt.toString("base64url"), key.toString("base64url")].join("$");
}

export async function verifyPassword(password: string, stored: string): Promise<boolean> {
  const parts = stored.split("$");
  if (parts.length !== 6 || parts[0] !== "scrypt") return false;
  const [n, r, p] = parts.slice(1, 4).map(Number);
  if (![n, r, p].every(Number.isSafeInteger) || n > 2 ** 20 || r > 32 || p > 16) return false;
  const expected = Buffer.from(parts[5], "base64url");
  const key = await derive(password, Buffer.from(parts[4], "base64url"), n, r, p);
  return key.length === expected.length && timingSafeEqual(key, expected);
}

/** A hash of nothing, so a login for an unknown email takes as long as a real one. */
let dummy: Promise<string> | undefined;
export function dummyHash(): Promise<string> {
  dummy ??= hashPassword(randomBytes(16).toString("hex"));
  return dummy;
}
