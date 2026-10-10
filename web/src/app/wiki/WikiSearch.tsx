"use client";

// Search every wiki page by name or ID. The list of names loads the first time the box gets focus.

import { useEffect, useRef, useState } from "react";

interface Entry {
  p: string;
  t: string;
  k: string;
  i?: string;
}

const KIND_NAMES: Record<string, string> = {
  rule: "Rule",
  skill: "Skill",
  monster: "Monster",
  npc: "NPC",
  quest: "Quest",
  item: "Item",
  zone: "Zone",
  backlog: "Not in game yet",
  list: "List",
};

export default function WikiSearch() {
  const [entries, setEntries] = useState<Entry[] | null>(null);
  const [query, setQuery] = useState("");
  const [open, setOpen] = useState(false);
  const box = useRef<HTMLDivElement>(null);

  const load = () => {
    if (entries) return;
    fetch("/api/wiki/search")
      .then((r) => r.json())
      .then((data: Entry[]) => setEntries(data))
      .catch(() => setEntries([]));
  };

  useEffect(() => {
    const close = (e: MouseEvent) => {
      if (box.current && !box.current.contains(e.target as Node)) setOpen(false);
    };
    document.addEventListener("mousedown", close);
    return () => document.removeEventListener("mousedown", close);
  }, []);

  const q = query.trim().toLowerCase();
  const results =
    q && entries
      ? entries
          .filter((e) => e.t.toLowerCase().includes(q) || e.i === q)
          .sort((a, b) => Number(!a.t.toLowerCase().startsWith(q)) - Number(!b.t.toLowerCase().startsWith(q)))
          .slice(0, 30)
      : [];

  return (
    <div className="wiki-search" ref={box}>
      <input
        type="search"
        placeholder="Search the wiki"
        aria-label="Search the wiki"
        value={query}
        onFocus={() => {
          load();
          setOpen(true);
        }}
        onChange={(e) => {
          setQuery(e.target.value);
          setOpen(true);
        }}
        onKeyDown={(e) => {
          if (e.key === "Enter" && results[0]) window.location.href = `/wiki/${results[0].p}`;
          if (e.key === "Escape") setOpen(false);
        }}
      />
      {open && q && (
        <ul className="results">
          {!entries && <li className="muted">Loading…</li>}
          {entries && results.length === 0 && <li className="muted">Nothing found</li>}
          {results.map((r) => (
            <li key={r.p}>
              <a href={`/wiki/${r.p}`}>
                {r.t}
                <span className="kind">
                  {KIND_NAMES[r.k] ?? r.k}
                  {r.i ? ` #${r.i}` : ""}
                </span>
              </a>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
