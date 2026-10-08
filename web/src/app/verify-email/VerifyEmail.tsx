"use client";

import Link from "next/link";
import { useEffect, useRef } from "react";
import { useSubmit } from "@/components/useSubmit";

export default function VerifyEmail() {
  const { result, submit } = useSubmit("/api/verify-email");
  const started = useRef(false);

  useEffect(() => {
    if (started.current) return;
    started.current = true;
    // The token is in the link's #fragment, which browsers never send to any server.
    const token = new URLSearchParams(window.location.hash.slice(1)).get("token") ?? "";
    window.history.replaceState(null, "", window.location.pathname);
    void submit({ token });
  }, [submit]);

  if (!result) {
    return (
      <div className="card">
        <h1>Confirming...</h1>
        <p className="lead">One moment.</p>
      </div>
    );
  }
  if (result.ok) {
    return (
      <div className="card">
        <h1>Email confirmed</h1>
        <p className="lead">Thanks! Start the game and sign in with your email and password.</p>
        <Link className="button" href="/account">
          Go to my account
        </Link>
      </div>
    );
  }
  return (
    <div className="card">
      <h1>Link not valid</h1>
      <p className="lead">{result.error}</p>
      <Link className="button" href="/account">
        Go to my account
      </Link>
    </div>
  );
}
