"use client";

import Link from "next/link";
import { useSubmit } from "@/components/useSubmit";

export default function ForgotForm() {
  const { busy, result, submit } = useSubmit("/api/forgot-password");

  async function onSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const form = new FormData(event.currentTarget);
    await submit({ email: String(form.get("email") ?? "") });
  }

  return (
    <div className="card">
      <h1>Forgot your password?</h1>
      <p className="lead">Enter your account's email and we'll send you a link to choose a new password.</p>
      {result?.ok ? (
        <div className="message ok">{result.message} The link works once, for one hour.</div>
      ) : (
        <form onSubmit={onSubmit} noValidate>
          <label>
            Email
            <input name="email" type="email" autoComplete="email" required maxLength={254} aria-invalid={result?.field === "email"} />
          </label>
          {result?.error && <div className="message error">{result.error}</div>}
          <button type="submit" disabled={busy}>
            {busy ? "Sending..." : "Send reset link"}
          </button>
        </form>
      )}
      <div className="links">
        <Link href="/signup">Create an account</Link>
      </div>
    </div>
  );
}
