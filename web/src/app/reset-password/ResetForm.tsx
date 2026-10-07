"use client";

import Link from "next/link";
import { useEffect, useState } from "react";
import { useSubmit } from "@/components/useSubmit";

const MIN = 8;

export default function ResetForm() {
  const { busy, result, submit } = useSubmit("/api/reset-password");
  // The token is in the link's #fragment, which browsers never send to any server.
  const [token, setToken] = useState<string | null>(null);
  const [mismatch, setMismatch] = useState(false);

  useEffect(() => {
    const value = new URLSearchParams(window.location.hash.slice(1)).get("token") ?? "";
    setToken(value);
    // Keep the token out of the address bar and history.
    window.history.replaceState(null, "", window.location.pathname);
  }, []);

  async function onSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const form = new FormData(event.currentTarget);
    const password = String(form.get("password") ?? "");
    if (password !== String(form.get("confirm") ?? "")) {
      setMismatch(true);
      return;
    }
    setMismatch(false);
    await submit({ token: token ?? "", password });
  }

  if (result?.ok) {
    return (
      <div className="card">
        <h1>Password changed</h1>
        <p className="lead">Sign in to the game with your new password.</p>
        <Link className="button" href="/">
          Back to the start
        </Link>
      </div>
    );
  }

  if (token === "") {
    return (
      <div className="card">
        <h1>Link not valid</h1>
        <p className="lead">This page needs the link from the reset email. Ask for a new one if it has expired.</p>
        <Link className="button" href="/forgot-password">
          Ask for a new link
        </Link>
      </div>
    );
  }

  return (
    <div className="card">
      <h1>Choose a new password</h1>
      <p className="lead">Pick a new password for your ROSE account.</p>
      <form onSubmit={onSubmit} noValidate>
        <label>
          New password
          <input name="password" type="password" autoComplete="new-password" required minLength={MIN} maxLength={128} />
          <span className="hint">At least {MIN} characters.</span>
          {result?.field === "password" && <span className="field-error">{result.error}</span>}
        </label>
        <label>
          Repeat the new password
          <input name="confirm" type="password" autoComplete="new-password" required aria-invalid={mismatch} />
          {mismatch && <span className="field-error">The passwords don't match.</span>}
        </label>
        {result?.error && result.field !== "password" && (
          <div className="message error">
            {result.error} <Link href="/forgot-password">Ask for a new link</Link>
          </div>
        )}
        <button type="submit" disabled={busy || token === null}>
          {busy ? "Saving..." : "Save new password"}
        </button>
      </form>
    </div>
  );
}
