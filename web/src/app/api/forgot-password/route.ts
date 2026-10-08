import { PUBLIC_URL, RESET_TOKEN_MINUTES } from "@/lib/config";
import { addPasswordReset, deleteExpiredResets, findAccountByEmail } from "@/lib/db";
import { sendMail } from "@/lib/mail";
import { allow } from "@/lib/rate-limit";
import { clientIp, json, readJson } from "@/lib/request";
import { newToken } from "@/lib/tokens";
import { normalizeEmail } from "@/lib/validate";

const HOUR_MS = 60 * 60 * 1000;

// The answer is the same whether or not the email has an account, so this page can't be
// used to find out who plays.
const SENT = { ok: true, message: "If that email has an account, we sent it a link to choose a new password." };

export async function POST(request: Request) {
  const body = await readJson(request);
  if (body instanceof Response) return body;
  const email = normalizeEmail(body.email);
  if (!email) return json({ error: "Enter a valid email address.", field: "email" }, 400);
  if (!allow(`forgot-ip:${clientIp(request)}`, 30, HOUR_MS) || !allow(`forgot:${email}`, 3, HOUR_MS)) {
    return json({ error: "Too many reset requests. Try again later." }, 429);
  }
  deleteExpiredResets();
  const account = findAccountByEmail(email);
  if (account) {
    // The link carries the token; only its hash is stored.
    const { token, hash } = newToken();
    addPasswordReset(hash, account.id, Date.now() + RESET_TOKEN_MINUTES * 60 * 1000);
    const link = `${PUBLIC_URL}/reset-password#token=${token}`;
    // Sent in the background, so the answer comes back as fast as for an unknown email.
    sendMail(
      account.email,
      "Choose a new ROSE password",
      `Someone asked to reset the password of your ROSE account.\n\nOpen this link to choose a new one (it works once, for ${RESET_TOKEN_MINUTES} minutes):\n${link}\n\nIf it wasn't you, ignore this email; your password stays the same.\n`,
    ).catch((error) => console.error("[mail] sending the reset email failed:", error instanceof Error ? error.message : error));
  }
  return json(SENT);
}
