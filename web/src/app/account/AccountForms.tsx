"use client";

import { useState } from "react";
import { useSubmit } from "@/components/useSubmit";

const MIN = 8;

export function ChangePasswordForm() {
  const { busy, result, submit } = useSubmit("/api/account/password");
  const [mismatch, setMismatch] = useState(false);

  async function onSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const formElement = event.currentTarget;
    const form = new FormData(formElement);
    const password = String(form.get("password") ?? "");
    if (password !== String(form.get("confirm") ?? "")) {
      setMismatch(true);
      return;
    }
    setMismatch(false);
    const answer = await submit({ current: String(form.get("current") ?? ""), password });
    if (answer.ok) formElement.reset();
  }

  const fieldError = (field: string) => (result?.field === field ? result.error : undefined);
  return (
    <form onSubmit={onSubmit} noValidate>
      <label>
        Current password
        <input name="current" type="password" autoComplete="current-password" required aria-invalid={!!fieldError("current")} />
        {fieldError("current") && <span className="field-error">{fieldError("current")}</span>}
      </label>
      <label>
        New password
        <input
          name="password"
          type="password"
          autoComplete="new-password"
          required
          minLength={MIN}
          maxLength={128}
          aria-invalid={!!fieldError("password")}
        />
        <span className="hint">At least {MIN} characters.</span>
        {fieldError("password") && <span className="field-error">{fieldError("password")}</span>}
      </label>
      <label>
        Repeat the new password
        <input name="confirm" type="password" autoComplete="new-password" required aria-invalid={mismatch} />
        {mismatch && <span className="field-error">The passwords don&apos;t match.</span>}
      </label>
      {result?.error && !result.field && <div className="message error">{result.error}</div>}
      {result?.ok && <div className="message ok">{result.message}</div>}
      <button type="submit" disabled={busy}>
        {busy ? "Saving..." : "Change password"}
      </button>
    </form>
  );
}

function ActionButton({ url, label, busyLabel, secondary, after }: { url: string; label: string; busyLabel: string; secondary?: boolean; after?: () => void }) {
  const { busy, result, submit } = useSubmit(url);
  return (
    <div className="action-button">
      <button
        type="button"
        className={secondary ? "secondary small" : "small"}
        disabled={busy}
        onClick={async () => {
          const answer = await submit({});
          if (answer.ok) after?.();
        }}
      >
        {busy ? busyLabel : label}
      </button>
      {result?.message && <span className="ok-text">{result.message}</span>}
      {result?.error && <span className="field-error">{result.error}</span>}
    </div>
  );
}

export function ResendButton() {
  return <ActionButton url="/api/account/resend-verification" label="Send a new link" busyLabel="Sending..." />;
}

export function SignOutOthersButton() {
  return (
    <ActionButton
      url="/api/account/sign-out-others"
      label="Sign out other devices"
      busyLabel="Signing out..."
      secondary
      after={() => window.location.reload()}
    />
  );
}

export function SignOutButton() {
  return (
    <ActionButton url="/api/logout" label="Sign out" busyLabel="Signing out..." secondary after={() => window.location.assign("/")} />
  );
}
