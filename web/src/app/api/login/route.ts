// Website sign-in (the account page).

import { checkCredentials } from "@/lib/credentials";
import { json, readJson } from "@/lib/request";
import { startSession } from "@/lib/session";

export async function POST(request: Request) {
  const body = await readJson(request);
  if (body instanceof Response) return body;
  const account = await checkCredentials(request, body);
  if (account instanceof Response) return account;
  const response = json({ ok: true });
  startSession(response, account.id, request.headers.get("user-agent") ?? "");
  return response;
}
