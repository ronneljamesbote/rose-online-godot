"use client";

// The dev-client GM command for this page as the last fact, with an icon that copies it
// for the chat box.

import Link from "next/link";
import { toast } from "sonner";

export default function GmCommand({ command }: { command: string }) {
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
    toast(`Copied ${command}`);
  };
  return (
    <div className="fact gm-copy">
      <dt>
        GM command (<Link href="/wiki/rules/gm-commands">dev client only</Link>)
      </dt>
      <dd>
        <code>{command}</code>
        <button type="button" onClick={copy} title="Copy" aria-label={`Copy ${command}`}>
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
            <rect x="9" y="9" width="12" height="12" rx="2" />
            <path d="M5 15H4a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v1" />
          </svg>
        </button>
      </dd>
    </div>
  );
}
