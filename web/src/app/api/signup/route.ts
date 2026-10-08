import { randomUUID } from "node:crypto";
import { REQUIRE_VERIFIED_EMAIL } from "@/lib/config";
import { createAccount } from "@/lib/db";
import { hashPassword } from "@/lib/password";
import { allow } from "@/lib/rate-limit";
import { clientIp, json, readJson } from "@/lib/request";
import { startSession } from "@/lib/session";
import { normalizeEmail, passwordProblem } from "@/lib/validate";
import { sendVerificationEmail } from "@/lib/verification";

export async function POST(request: Request) {
  const body = await readJson(request);
  if (body instanceof Response) return body;
  if (!allow(`signup:${clientIp(request)}`, 20, 60 * 60 * 1000)) {
    return json({ error: "Too many sign-ups from here. Try again later." }, 429);
  }
  const email = normalizeEmail(body.email);
  if (!email) return json({ error: "Enter a valid email address.", field: "email" }, 400);
  const problem = passwordProblem(body.password, email);
  if (problem) return json({ error: problem, field: "password" }, 400);
  const id = randomUUID();
  if (!createAccount(id, email, await hashPassword(body.password as string))) {
    return json({ error: "There is already an account with this email.", field: "email" }, 409);
  }
  sendVerificationEmail({ id, email });
  // Signed in on the website straight away.
  const response = json({ ok: true, verify_required: REQUIRE_VERIFIED_EMAIL }, 201);
  startSession(response, id, request.headers.get("user-agent") ?? "");
  return response;
}
