// Shows a page's data block: single values as a fact list, lists of rows as tables,
// nested groups as their own fact lists. [[links]] inside values become links.

import { renderInline, escapeHtml } from "@/lib/wiki-markdown";
import type { Wiki, WikiData } from "@/lib/wiki";
import Icon from "./Icon";

/** Keys the page shows elsewhere (title, badges, map, source box) or that only steer layout. */
const HIDDEN = new Set(["kind", "name", "title", "status", "source", "icon", "map", "columns", "id"]);

/** Short words that read better in capitals. */
const WORDS: Record<string, string> = { hp: "HP", mp: "MP", xp: "XP", npc: "NPC", npcs: "NPCs", pvp: "PvP", id: "ID" };

export function label(key: string): string {
  const words = key.split("_").map((w) => WORDS[w] ?? w);
  const text = words.join(" ");
  return text.charAt(0).toUpperCase() + text.slice(1);
}

function isRow(v: unknown): v is Record<string, unknown> {
  return !!v && typeof v === "object" && !Array.isArray(v);
}

export function Value({ value, wiki }: { value: unknown; wiki: Wiki }) {
  if (value === null || value === undefined || value === "") return <span className="muted">-</span>;
  if (typeof value === "boolean") return <>{value ? "Yes" : "No"}</>;
  if (typeof value === "number") return <span className="num">{value.toLocaleString("en-US")}</span>;
  if (typeof value === "string") {
    if (/^(item|skill)\/\d+$/.test(value)) return <Icon spec={value} />;
    return <span dangerouslySetInnerHTML={{ __html: renderInline(value, wiki) }} />;
  }
  if (Array.isArray(value)) {
    if (value.every(isRow)) return <RowsTable rows={value} wiki={wiki} />;
    return (
      <>
        {value.map((v, i) => (
          <span key={i}>
            {i > 0 && ", "}
            <Value value={v} wiki={wiki} />
          </span>
        ))}
      </>
    );
  }
  if (isRow(value)) return <Facts data={value} wiki={wiki} />;
  return <span dangerouslySetInnerHTML={{ __html: escapeHtml(String(value)) }} />;
}

export function RowsTable({ rows, wiki }: { rows: Record<string, unknown>[]; wiki: Wiki }) {
  const columns: string[] = [];
  for (const row of rows) for (const key of Object.keys(row)) if (!columns.includes(key)) columns.push(key);
  return (
    <div className="table-wrap">
      <table className="wiki-table">
        <thead>
          <tr>
            {columns.map((c) => (
              <th key={c}>{label(c)}</th>
            ))}
          </tr>
        </thead>
        <tbody>
          {rows.map((row, i) => (
            <tr key={i}>
              {columns.map((c) => (
                <td key={c} className={typeof row[c] === "number" ? "num" : undefined}>
                  <Value value={row[c]} wiki={wiki} />
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export function Facts({ data, wiki }: { data: Record<string, unknown>; wiki: Wiki }) {
  const entries = Object.entries(data).filter(([, v]) => !(Array.isArray(v) && v.length > 0 && v.every(isRow)));
  if (entries.length === 0) return null;
  return (
    <dl className="facts wiki-facts">
      {entries.map(([k, v]) => (
        <div key={k} className="fact">
          <dt>{label(k)}</dt>
          <dd>
            <Value value={v} wiki={wiki} />
          </dd>
        </div>
      ))}
    </dl>
  );
}

/** The whole data block: facts first, then one titled table per list of rows. */
export default function DataView({ data, wiki }: { data: WikiData; wiki: Wiki }) {
  const shown = Object.fromEntries(Object.entries(data).filter(([k]) => !HIDDEN.has(k)));
  const tables = Object.entries(shown).filter(([, v]) => Array.isArray(v) && v.length > 0 && v.every(isRow));
  return (
    <>
      <Facts data={shown} wiki={wiki} />
      {tables.map(([k, v]) => (
        <section key={k} className="data-table">
          <h2>{label(k)}</h2>
          <RowsTable rows={v as Record<string, unknown>[]} wiki={wiki} />
        </section>
      ))}
    </>
  );
}
