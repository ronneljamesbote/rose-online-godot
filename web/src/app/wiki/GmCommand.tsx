"use client";

// The dev-client GM command for this page as the last fact, with a button that copies it
// for the chat box.

import Link from "next/link";
import { useState } from "react";

export default function GmCommand({ command }: { command: string }) {
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
    <div className="fact gm-copy">
      <dt>
        GM command (<Link href="/wiki/rules/gm-commands">dev client only</Link>)
      </dt>
      <dd>
        <code>{command}</code>
        <button type="button" className="secondary" onClick={copy} aria-live="polite">
          {copied ? "Copied" : "Copy"}
        </button>
      </dd>
    </div>
  );
}
