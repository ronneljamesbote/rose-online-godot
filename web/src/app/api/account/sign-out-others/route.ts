import { deleteOtherSessions } from "@/lib/db";
import { json, readJson } from "@/lib/request";
import { currentSession } from "@/lib/session";

export async function POST(request: Request) {
  const body = await readJson(request);
  if (body instanceof Response) return body;
  const signedIn = await currentSession();
  if (!signedIn) return json({ error: "Sign in first." }, 401);
  const count = deleteOtherSessions(signedIn.account.id, signedIn.session.id_hash);
  return json({ ok: true, message: count ? `Signed out ${count} other ${count === 1 ? "device" : "devices"}.` : "No other devices were signed in." });
}
