"use client";

import { useState } from "react";

export interface ApiResult {
  ok?: boolean;
  error?: string;
  field?: string;
  message?: string;
  verify_required?: boolean;
}

/** Posts a form's values as JSON and keeps the answer for the page to show. */
export function useSubmit(url: string) {
  const [busy, setBusy] = useState(false);
  const [result, setResult] = useState<ApiResult | null>(null);

  async function submit(values: Record<string, string>): Promise<ApiResult> {
    setBusy(true);
    setResult(null);
    let answer: ApiResult;
    try {
      const response = await fetch(url, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify(values),
      });
      answer = await response.json().catch(() => ({ error: "Something went wrong. Try again." }));
      if (!response.ok && !answer.error) answer.error = "Something went wrong. Try again.";
    } catch {
      answer = { error: "Can't reach the server. Check your connection and try again." };
    }
    setResult(answer);
    setBusy(false);
    return answer;
  }

  return { busy, result, submit };
}
