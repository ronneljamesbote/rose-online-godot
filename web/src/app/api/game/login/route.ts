// The game client signs in here and gets a short-lived token to connect to the game server.

import { findAccountByEmail } from "@/lib/db";
import { GAME_TOKEN_SECONDS } from "@/lib/config";
import { gameToken } from "@/lib/keys";
import { dummyHash, verifyPassword } from "@/lib/password";
import { allow, reset } from "@/lib/rate-limit";
import { clientIp, json, readJson } from "@/lib/request";
import { normalizeEmail } from "@/lib/validate";

const WINDOW_MS = 15 * 60 * 1000;

export async function POST(request: Request) {
  const body = await readJson(request);
  if (body instanceof Response) return body;
  const email = normalizeEmail(body.email);
  const password = typeof body.password === "string" ? body.password : "";
  const ip = clientIp(request);
  // Ten tries per account and a hundred per address every 15 minutes slow down guessing.
  if (!allow(`login-ip:${ip}`, 100, WINDOW_MS) || (email && !allow(`login:${email}`, 10, WINDOW_MS))) {
    return json({ error: "Too many sign-in attempts. Wait a few minutes and try again." }, 429);
  }
  const account = email ? findAccountByEmail(email) : undefined;
  const ok = await verifyPassword(password, account?.password_hash ?? (await dummyHash()));
  if (!account || !ok) return json({ error: "Wrong email or password." }, 401);
  reset(`login:${email}`);
  return json({ token: await gameToken(account.id), expires_in: GAME_TOKEN_SECONDS });
}
