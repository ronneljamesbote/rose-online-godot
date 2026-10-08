import { allow } from "@/lib/rate-limit";
import { json, readJson } from "@/lib/request";
import { currentSession } from "@/lib/session";
import { sendVerificationEmail } from "@/lib/verification";

export async function POST(request: Request) {
  const body = await readJson(request);
  if (body instanceof Response) return body;
  const signedIn = await currentSession();
  if (!signedIn) return json({ error: "Sign in first." }, 401);
  const { account } = signedIn;
  if (account.email_verified_at) return json({ ok: true, message: "Your email is already confirmed." });
  if (!allow(`resend:${account.id}`, 3, 60 * 60 * 1000)) {
    return json({ error: "We already sent a few links. Check your spam folder, or try again in an hour." }, 429);
  }
  sendVerificationEmail(account);
  return json({ ok: true, message: `We sent a new link to ${account.email}.` });
}
