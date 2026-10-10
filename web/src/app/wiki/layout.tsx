import type { Metadata } from "next";
import Link from "next/link";
import "katex/dist/katex.min.css";
import WikiSearch from "./WikiSearch";

export const metadata: Metadata = { title: "Wiki · ROSE Online" };

const SECTIONS: [string, string][] = [
  ["rules", "Rules"],
  ["skills", "Skills"],
  ["monsters", "Monsters"],
  ["npcs", "NPCs"],
  ["quests", "Quests"],
  ["items", "Items"],
  ["zones", "Zones"],
  ["backlog", "Not in game yet"],
];

export default function WikiLayout({ children }: { children: React.ReactNode }) {
  return (
    <div className="wiki">
      <aside className="wiki-side">
        <Link href="/wiki" className="wiki-home">
          Game wiki
        </Link>
        <WikiSearch />
        <nav aria-label="Wiki sections">
          {SECTIONS.map(([path, name]) => (
            <Link key={path} href={`/wiki/${path}`}>
              {name}
            </Link>
          ))}
        </nav>
      </aside>
      <div className="wiki-main">{children}</div>
    </div>
  );
}
