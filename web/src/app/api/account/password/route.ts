import { changePassword } from "@/lib/db";
import { hashPassword, verifyPassword } from "@/lib/password";
import { allow } from "@/lib/rate-limit";
import { json, readJson } from "@/lib/request";
import { currentSession } from "@/lib/session";
import { passwordProblem } from "@/lib/validate";

export async function POST(request: Request) {
  const body = await readJson(request);
  if (body instanceof Response) return body;
  const signedIn = await currentSession();
  if (!signedIn) return json({ error: "Sign in first." }, 401);
  const { account, session } = signedIn;
  if (!allow(`change-password:${account.id}`, 10, 15 * 60 * 1000)) {
    return json({ error: "Too many attempts. Wait a few minutes and try again." }, 429);
  }
  const current = typeof body.current === "string" ? body.current : "";
  if (!(await verifyPassword(current, account.password_hash))) {
    return json({ error: "Your current password is not right.", field: "current" }, 400);
  }
  const problem = passwordProblem(body.password, account.email);
  if (problem) return json({ error: problem, field: "password" }, 400);
  changePassword(account.id, await hashPassword(body.password as string), session.id_hash);
  return json({ ok: true, message: "Password changed. Other devices were signed out." });
}
