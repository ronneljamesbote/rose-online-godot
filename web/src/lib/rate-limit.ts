// Counts attempts per key in a sliding window, in memory (one website process).

const buckets = new Map<string, number[]>();

/** Records an attempt; false when the key already used up its attempts in the window. */
export function allow(key: string, limit: number, windowMs: number): boolean {
  const now = Date.now();
  const recent = (buckets.get(key) ?? []).filter((t) => now - t < windowMs);
  if (recent.length >= limit) {
    buckets.set(key, recent);
    return false;
  }
  recent.push(now);
  buckets.set(key, recent);
  if (buckets.size > 10_000) {
    for (const [k, times] of buckets) if (times.every((t) => now - t >= windowMs)) buckets.delete(k);
  }
  return true;
}

/** Clears a key, e.g. failed logins after a good one. */
export function reset(key: string): void {
  buckets.delete(key);
}
