import { deleteSession } from "@/lib/db";
import { json, readJson } from "@/lib/request";
import { currentSession, endSessionCookie } from "@/lib/session";

export async function POST(request: Request) {
  const body = await readJson(request);
  if (body instanceof Response) return body;
  const signedIn = await currentSession();
  if (signedIn) deleteSession(signedIn.session.id_hash);
  const response = json({ ok: true });
  endSessionCookie(response);
  return response;
}
