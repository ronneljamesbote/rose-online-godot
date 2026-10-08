// Website sign-in. The cookie holds a random token; the database keeps only its hash.
// It is HttpOnly (scripts can't read it), SameSite=Lax (other sites' forms don't carry it)
// and, on HTTPS, Secure with the __Host- prefix.

import { cookies } from "next/headers";
import type { NextResponse } from "next/server";
import { SECURE_COOKIES, SESSION_DAYS } from "./config";
import { addSession, deleteExpiredSessions, findSession, touchSession, type Account, type Session } from "./db";
import { hashToken, looksLikeToken, newToken } from "./tokens";

export const SESSION_COOKIE = SECURE_COOKIES ? "__Host-rose_session" : "rose_session";
const SESSION_MS = SESSION_DAYS * 24 * 60 * 60 * 1000;

export function startSession(response: NextResponse, accountId: string, userAgent: string): void {
  deleteExpiredSessions();
  const { token, hash } = newToken();
  addSession(hash, accountId, Date.now() + SESSION_MS, userAgent);
  response.cookies.set(SESSION_COOKIE, token, {
    httpOnly: true,
    secure: SECURE_COOKIES,
    sameSite: "lax",
    path: "/",
    maxAge: SESSION_DAYS * 24 * 60 * 60,
  });
}

export function endSessionCookie(response: NextResponse): void {
  response.cookies.set(SESSION_COOKIE, "", { httpOnly: true, secure: SECURE_COOKIES, sameSite: "lax", path: "/", maxAge: 0 });
}

/** The signed-in account for this request, if any. */
export async function currentSession(): Promise<{ session: Session; account: Account } | null> {
  const token = (await cookies()).get(SESSION_COOKIE)?.value;
  if (!looksLikeToken(token)) return null;
  const hash = hashToken(token);
  const found = findSession(hash);
  if (!found) return null;
  touchSession(hash, Date.now() + SESSION_MS);
  return found;
}
