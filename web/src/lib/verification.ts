// Email confirmation links.

import { PUBLIC_URL, VERIFY_TOKEN_HOURS } from "./config";
import { addEmailVerification, type Account } from "./db";
import { sendMail } from "./mail";
import { newToken } from "./tokens";

/** Sends a new confirmation link (the older one stops working). Sent in the background. */
export function sendVerificationEmail(account: Pick<Account, "id" | "email">): void {
  const { token, hash } = newToken();
  addEmailVerification(hash, account.id, Date.now() + VERIFY_TOKEN_HOURS * 60 * 60 * 1000);
  const link = `${PUBLIC_URL}/verify-email#token=${token}`;
  sendMail(
    account.email,
    "Confirm your ROSE account",
    `Welcome to ROSE!\n\nOpen this link to confirm your email (it works for ${VERIFY_TOKEN_HOURS} hours):\n${link}\n\nIf you didn't make an account, ignore this email.\n`,
  ).catch((error) => console.error("[mail] sending the confirmation email failed:", error instanceof Error ? error.message : error));
}
