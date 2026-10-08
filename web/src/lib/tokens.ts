// Random tokens for links and sessions. Only their SHA-256 goes in the database, so a
// copy of the database can't be used to sign in or reset a password.

import { createHash, randomBytes } from "node:crypto";

/** 32 random bytes, base64url (43 characters). */
export function newToken(): { token: string; hash: string } {
  const token = randomBytes(32).toString("base64url");
  return { token, hash: hashToken(token) };
}

export function hashToken(token: string): string {
  return createHash("sha256").update(token).digest("hex");
}

export function looksLikeToken(token: unknown): token is string {
  return typeof token === "string" && /^[A-Za-z0-9_-]{43}$/.test(token);
}
