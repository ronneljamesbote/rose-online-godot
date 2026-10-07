import { createHash } from "node:crypto";
import { resetPassword } from "@/lib/db";
import { hashPassword } from "@/lib/password";
import { allow } from "@/lib/rate-limit";
import { clientIp, json, readJson } from "@/lib/request";
import { passwordProblem } from "@/lib/validate";

export async function POST(request: Request) {
  const body = await readJson(request);
  if (body instanceof Response) return body;
  if (!allow(`reset-ip:${clientIp(request)}`, 30, 60 * 60 * 1000)) {
    return json({ error: "Too many attempts. Try again later." }, 429);
  }
  const token = typeof body.token === "string" ? body.token : "";
  if (!/^[A-Za-z0-9_-]{43}$/.test(token)) return json({ error: "This reset link is not valid. Ask for a new one." }, 400);
  const problem = passwordProblem(body.password);
  if (problem) return json({ error: problem, field: "password" }, 400);
  const tokenHash = createHash("sha256").update(token).digest("hex");
  if (!resetPassword(tokenHash, await hashPassword(body.password as string))) {
    return json({ error: "This reset link has expired or was already used. Ask for a new one." }, 400);
  }
  return json({ ok: true });
}
