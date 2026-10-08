import { verifyEmail } from "@/lib/db";
import { allow } from "@/lib/rate-limit";
import { clientIp, json, readJson } from "@/lib/request";
import { hashToken, looksLikeToken } from "@/lib/tokens";

export async function POST(request: Request) {
  const body = await readJson(request);
  if (body instanceof Response) return body;
  if (!allow(`verify-ip:${clientIp(request)}`, 30, 60 * 60 * 1000)) {
    return json({ error: "Too many attempts. Try again later." }, 429);
  }
  if (!looksLikeToken(body.token) || !verifyEmail(hashToken(body.token))) {
    return json({ error: "This link has expired or is not valid. Sign in on the website to get a new one." }, 400);
  }
  return json({ ok: true });
}
