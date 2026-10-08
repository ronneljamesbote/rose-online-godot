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
  if (!fromThisSite(request)) {
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

/** True when the browser says the request comes from a page of this site: its Origin is
 * PUBLIC_URL, or has the same host as the address the browser asked for (so localhost,
 * 127.0.0.1 and a LAN address all work). Requests without an Origin (the game) pass. */
function fromThisSite(request: Request): boolean {
  const origin = request.headers.get("origin");
  if (!origin) return true;
  let originHost: string;
  try {
    originHost = new URL(origin).host;
  } catch {
    return false;
  }
  if (originHost === new URL(PUBLIC_URL).host) return true;
  const hosts = [request.headers.get("host")];
  if (process.env.TRUST_PROXY === "1") hosts.push(request.headers.get("x-forwarded-host")?.split(",")[0]?.trim() ?? null);
  return hosts.some((host) => host && host.toLowerCase() === originHost.toLowerCase());
}

/** The caller's address for rate limits: the proxy's X-Forwarded-For when TRUST_PROXY=1. */
export function clientIp(request: Request): string {
  if (process.env.TRUST_PROXY === "1") {
    const forwarded = request.headers.get("x-forwarded-for")?.split(",")[0]?.trim();
    if (forwarded) return forwarded;
  }
  return request.headers.get("x-real-ip") ?? "local";
}
