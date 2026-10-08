// The game client signs in here and gets a short-lived token to connect to the game server.

import { GAME_TOKEN_SECONDS, REQUIRE_VERIFIED_EMAIL } from "@/lib/config";
import { checkCredentials } from "@/lib/credentials";
import { gameToken } from "@/lib/keys";
import { json, readJson } from "@/lib/request";

export async function POST(request: Request) {
  const body = await readJson(request);
  if (body instanceof Response) return body;
  const account = await checkCredentials(request, body);
  if (account instanceof Response) return account;
  if (REQUIRE_VERIFIED_EMAIL && !account.email_verified_at) {
    return json(
      { error: "Confirm your email first: open the link we sent you. Sign in on the website to send a new one." },
      403,
    );
  }
  return json({ token: await gameToken(account.id), expires_in: GAME_TOKEN_SECONDS });
}
