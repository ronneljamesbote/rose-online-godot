// Shared request checks for the JSON API routes.

import { NextResponse } from "next/server";
import { PUBLIC_URL } from "./config";

export function json(body: unknown, status = 200): NextResponse {
  return NextResponse.json(body, { status, headers: { "cache-control": "no-store" } });
}

/** Reads a JSON body from this site (or from the game, which sends no Origin header).
 * A form posted from another site is refused, so nobody can sign people up or reset
 * passwords from their own pages. */
export async function readJson(request: Request): Promise<Record<string, unknown> | NextResponse> {
  const origin = request.headers.get("origin");
  if (origin && origin !== new URL(PUBLIC_URL).origin && origin !== new URL(request.url).origin) {
    return json({ error: "Requests from other sites are not allowed." }, 403);
  }
  if (!request.headers.get("content-type")?.includes("application/json")) {
    return json({ error: "Send JSON." }, 415);
  }
  try {
    const body = await request.json();
    return body && typeof body === "object" ? (body as Record<string, unknown>) : json({ error: "Bad request." }, 400);
  } catch {
    return json({ error: "Bad request." }, 400);
  }
}

/** The caller's address for rate limits: the proxy's X-Forwarded-For when TRUST_PROXY=1. */
export function clientIp(request: Request): string {
  if (process.env.TRUST_PROXY === "1") {
    const forwarded = request.headers.get("x-forwarded-for")?.split(",")[0]?.trim();
    if (forwarded) return forwarded;
  }
  return request.headers.get("x-real-ip") ?? "local";
}
