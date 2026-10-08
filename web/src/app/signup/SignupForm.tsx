"use client";

import Link from "next/link";
import { useState } from "react";
import { useSubmit } from "@/components/useSubmit";

const MIN = 8;

export default function SignupForm() {
  const { busy, result, submit } = useSubmit("/api/signup");
  const [mismatch, setMismatch] = useState(false);
  const [done, setDone] = useState("");
  const [verifyRequired, setVerifyRequired] = useState(false);

  async function onSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const form = new FormData(event.currentTarget);
    const email = String(form.get("email") ?? "");
    const password = String(form.get("password") ?? "");
    if (password !== String(form.get("confirm") ?? "")) {
      setMismatch(true);
      return;
    }
    setMismatch(false);
    const answer = await submit({ email, password });
    if (answer.ok) {
      setVerifyRequired(!!answer.verify_required);
      setDone(email.trim().toLowerCase());
    }
  }

  if (done) {
    return (
      <div className="card">
        <h1>You&apos;re all set</h1>
        <p className="lead">
          Your account <strong>{done}</strong> is ready. We sent you an email with a link to confirm the address
          {verifyRequired ? "; open it before you play." : "."} Then start the game and sign in with this email and your
          password.
        </p>
        <a className="button" href="/account">
          Go to my account
        </a>
      </div>
    );
  }

  const fieldError = (field: string) => (result?.field === field ? result.error : undefined);

  return (
    <div className="card">
      <h1>Create an account</h1>
      <p className="lead">You sign in to the game with this email and password.</p>
      <form onSubmit={onSubmit} noValidate>
        <label>
          Email
          <input name="email" type="email" autoComplete="email" required maxLength={254} aria-invalid={!!fieldError("email")} />
          {fieldError("email") && <span className="field-error">{fieldError("email")}</span>}
        </label>
        <label>
          Password
          <input
            name="password"
            type="password"
            autoComplete="new-password"
            required
            minLength={MIN}
            maxLength={128}
            aria-invalid={!!fieldError("password")}
          />
          <span className="hint">At least {MIN} characters. A short sentence is easy to remember and hard to guess.</span>
          {fieldError("password") && <span className="field-error">{fieldError("password")}</span>}
        </label>
        <label>
          Repeat the password
          <input name="confirm" type="password" autoComplete="new-password" required aria-invalid={mismatch} />
          {mismatch && <span className="field-error">The passwords don't match.</span>}
        </label>
        {result?.error && !result.field && <div className="message error">{result.error}</div>}
        <button type="submit" disabled={busy}>
          {busy ? "Creating..." : "Create account"}
        </button>
      </form>
      <div className="links">
        <Link href="/forgot-password">Forgot password?</Link>
      </div>
    </div>
  );
}
