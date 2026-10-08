"use client";

import Link from "next/link";
import { useSubmit } from "@/components/useSubmit";

export default function LoginForm() {
  const { busy, result, submit } = useSubmit("/api/login");

  async function onSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const form = new FormData(event.currentTarget);
    const answer = await submit({ email: String(form.get("email") ?? ""), password: String(form.get("password") ?? "") });
    // A full load, so the header shows the account link.
    if (answer.ok) window.location.assign("/account");
  }

  return (
    <div className="card">
      <h1>Sign in</h1>
      <p className="lead">Use the email and password of your ROSE account.</p>
      <form onSubmit={onSubmit} noValidate>
        <label>
          Email
          <input name="email" type="email" autoComplete="email" required maxLength={254} />
        </label>
        <label>
          Password
          <input name="password" type="password" autoComplete="current-password" required maxLength={128} />
        </label>
        {result?.error && <div className="message error">{result.error}</div>}
        <button type="submit" disabled={busy || result?.ok}>
          {busy ? "Signing in..." : "Sign in"}
        </button>
      </form>
      <div className="links">
        <Link href="/forgot-password">Forgot password?</Link>
        <Link href="/signup">Create an account</Link>
      </div>
    </div>
  );
}
