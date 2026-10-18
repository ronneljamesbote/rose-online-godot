"use client";

// The dev-client GM command for this page, with a button that copies it for the chat box.

import Link from "next/link";
import { useState } from "react";

const WHAT: Record<string, string> = {
  item: "Put this item in your bag",
  monster: "Spawn this monster next to you",
  zone: "Warp to this zone's start",
};

export default function GmCommand({ command, kind }: { command: string; kind: string }) {
  const [copied, setCopied] = useState(false);
  const copy = async () => {
    try {
      await navigator.clipboard.writeText(command);
    } catch {
      // Clipboard API unavailable (plain http on another host): copy through a selection.
      const area = document.createElement("textarea");
      area.value = command;
      document.body.appendChild(area);
      area.select();
      document.execCommand("copy");
      area.remove();
    }
    setCopied(true);
    setTimeout(() => setCopied(false), 1500);
  };
  return (
    <section className="gm-copy">
      <h2>GM command</h2>
      <p className="muted">{WHAT[kind]}: paste in the chat box.</p>
      <div className="gm-copy-row">
        <code>{command}</code>
        <button type="button" className="secondary" onClick={copy} aria-live="polite">
          {copied ? "Copied" : "Copy"}
        </button>
      </div>
      <p className="hint">
        Only works in the dev client on a dev server, not in the normal game. See{" "}
        <Link href="/wiki/rules/gm-commands">GM commands</Link>.
      </p>
    </section>
  );
}
