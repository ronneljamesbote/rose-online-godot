// Email and password checks shared by the website and game sign-in.

import { findAccountByEmail, type Account } from "./db";
import { dummyHash, verifyPassword } from "./password";
import { allow, reset } from "./rate-limit";
import { clientIp, json } from "./request";
import { normalizeEmail } from "./validate";

const WINDOW_MS = 15 * 60 * 1000;

/** The account, or the error answer to send. */
export async function checkCredentials(request: Request, body: Record<string, unknown>): Promise<Account | Response> {
  const email = normalizeEmail(body.email);
  const password = typeof body.password === "string" ? body.password : "";
  // Ten tries per account and a hundred per address every 15 minutes slow down guessing.
  if (!allow(`login-ip:${clientIp(request)}`, 100, WINDOW_MS) || (email && !allow(`login:${email}`, 10, WINDOW_MS))) {
    return json({ error: "Too many sign-in attempts. Wait a few minutes and try again." }, 429);
  }
  const account = email ? findAccountByEmail(email) : undefined;
  // An unknown email costs the same hashing time as a wrong password.
  const ok = await verifyPassword(password, account?.password_hash ?? (await dummyHash()));
  if (!account || !ok) return json({ error: "Wrong email or password." }, 401);
  reset(`login:${email}`);
  return account;
}
