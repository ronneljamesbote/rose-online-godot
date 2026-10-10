// One wiki page: title and badges, map, data block, text, the folder's page list, where its
// values come from, and the pages linking here.

import Link from "next/link";
import { notFound } from "next/navigation";
import { GITHUB_EDIT_BASE, comparePaths, linkHref, loadWiki, type Wiki, type WikiPage } from "@/lib/wiki";
import { renderInline, renderMarkdown } from "@/lib/wiki-markdown";
import DataView, { label } from "./DataView";
import Icon, { lookup } from "./Icon";
import PageList, { type ListRow } from "./PageList";
import ZoneMap from "./ZoneMap";

const STATUS: Record<string, [string, string]> = {
  "in-game": ["In game", "ok"],
  "changed-from-irose": ["Changed from iROSE", "warn"],
  "not-in-game-yet": ["Not in game yet", "bad"],
};

const KIND_NAMES: Record<string, string> = {
  rule: "Rule",
  skill: "Skill",
  monster: "Monster",
  npc: "NPC",
  quest: "Quest",
  item: "Item",
  zone: "Zone",
  backlog: "Not in game yet",
};

const REPO_BLOB = GITHUB_EDIT_BASE.replace("/edit/", "/blob/").replace(/wiki\/$/, "");

function folderRows(wiki: Wiki, folder: string, columns: string[]): { rows: ListRow[]; subfolders: string[] } {
  const rows: ListRow[] = [];
  const subfolders: string[] = [];
  for (const child of wiki.children.get(folder) ?? []) {
    if (wiki.children.has(child)) {
      subfolders.push(child);
      continue;
    }
    const page = wiki.pages.get(child);
    if (!page) continue;
    const icon = typeof page.data.icon === "string" ? lookup(page.data.icon) : undefined;
    rows.push({
      href: linkHref(child),
      name: page.title,
      id: page.data.id === undefined ? "" : String(page.data.id).split("/").pop()!,
      icon,
      status: typeof page.data.status === "string" ? page.data.status : undefined,
      cells: columns.map((c) => {
        const v = page.data[c];
        if (v === undefined || v === null) return { html: "", sort: null };
        if (typeof v === "number") return { html: v.toLocaleString("en-US"), sort: v };
        if (typeof v === "boolean") return { html: v ? "Yes" : "No", sort: v ? 1 : 0 };
        if (Array.isArray(v)) return { html: String(v.length), sort: v.length };
        const text = String(v);
        return { html: renderInline(text, wiki), sort: text.replace(/\[\[[^|\]]*\|?([^\]]*)\]\]/g, "$1").toLowerCase() };
      }),
    });
  }
  // Folders also hold folder pages of their own (items/weapon); those are listed as sections.
  for (const [path] of wiki.children) {
    if (!path || path === folder || !path.startsWith(folder ? `${folder}/` : "")) continue;
    const rest = folder ? path.slice(folder.length + 1) : path;
    if (!rest.includes("/") && !subfolders.includes(path) && !wiki.pages.has(path)) subfolders.push(path);
  }
  subfolders.sort(comparePaths);
  return { rows, subfolders };
}

function SourceBox({ page }: { page: WikiPage }) {
  const source = page.data.source;
  if (!source || typeof source !== "object") return null;
  const entries = Object.entries(source as Record<string, unknown>);
  const codeLink = (file: string) => {
    const m = /^([\w./-]+\.(rs|gd|ts|tsx|toml|md))(.*)$/.exec(file.trim());
    if (!m) return <>{file}</>;
    return (
      <>
        <a href={`${REPO_BLOB}${m[1]}`}>
          <code>{m[1]}</code>
        </a>
        {m[3]}
      </>
    );
  };
  return (
    <section className="source-box">
      <h2>Source</h2>
      <dl className="facts wiki-facts">
        {entries.map(([k, v]) => (
          <div key={k} className="fact">
            <dt>{label(k)}</dt>
            <dd>
              {(Array.isArray(v) ? v : [v]).map((item, i) => (
                <div key={i}>{k === "code" ? codeLink(String(item)) : String(item)}</div>
              ))}
            </dd>
          </div>
        ))}
      </dl>
    </section>
  );
}

export default function WikiPageView({ path }: { path: string }) {
  const wiki = loadWiki();
  const page = wiki.pages.get(path);
  const isFolder = wiki.children.has(path);
  if (!page && !isFolder) notFound();

  const data = page?.data ?? {};
  const title = page?.title ?? label(path.split("/").pop() ?? "Wiki");
  const status = typeof data.status === "string" ? STATUS[data.status] : undefined;
  const kindName = KIND_NAMES[String(data.kind)] ?? null;
  const id = data.id;
  const columns = Array.isArray(data.columns) ? data.columns.map(String) : [];
  const listing = isFolder ? folderRows(wiki, path, columns) : null;
  const backlinks = [...(wiki.backlinks.get(path) ?? [])].sort(comparePaths);
  const crumbs = path ? path.split("/").slice(0, -1) : [];

  return (
    <article className="wiki-page">
      {path && (
        <nav className="crumbs" aria-label="Breadcrumbs">
          <Link href="/wiki">Wiki</Link>
          {crumbs.map((_, i) => {
            const p = crumbs.slice(0, i + 1).join("/");
            return (
              <span key={p}>
                {" / "}
                <Link href={linkHref(p)}>{wiki.pages.get(p)?.title ?? label(crumbs[i])}</Link>
              </span>
            );
          })}
        </nav>
      )}
      <header className="wiki-title">
        {typeof data.icon === "string" && <Icon spec={data.icon} />}
        <div>
          <h1>{title}</h1>
          <div className="badges">
            {kindName && (
              <span className="badge">
                {kindName}
                {id !== undefined && id !== "" ? ` #${id}` : ""}
              </span>
            )}
            {status && <span className={`badge ${status[1]}`}>{status[0]}</span>}
          </div>
        </div>
        {page && (
          <a className="edit" href={`${GITHUB_EDIT_BASE}${page.file}`}>
            Edit on GitHub
          </a>
        )}
      </header>

      {page?.error && <p className="message error">This page&apos;s data block has an error: {page.error}</p>}
      <ZoneMap data={data} wiki={wiki} />
      <DataView data={data} wiki={wiki} />
      {page && page.body.trim() && (
        <div className="wiki-body" dangerouslySetInnerHTML={{ __html: renderMarkdown(page.body.replace(/^#\s+.+\n/, ""), wiki) }} />
      )}

      {listing && listing.subfolders.length > 0 && (
        <ul className="subfolders">
          {listing.subfolders.map((f) => (
            <li key={f}>
              <Link href={linkHref(f)}>{wiki.pages.get(f)?.title ?? label(f.split("/").pop()!)}</Link>
              <span className="muted"> {(wiki.children.get(f) ?? []).length.toLocaleString("en-US")} pages</span>
            </li>
          ))}
        </ul>
      )}
      {listing && listing.rows.length > 0 && <PageList columns={columns.map(label)} rows={listing.rows} />}

      {page && <SourceBox page={page} />}

      {backlinks.length > 0 && (
        <section className="backlinks">
          <h2>Linked from</h2>
          <ul>
            {backlinks.slice(0, 300).map((b) => (
              <li key={b}>
                <Link href={linkHref(b)}>{wiki.pages.get(b)?.title ?? b}</Link>
              </li>
            ))}
          </ul>
          {backlinks.length > 300 && <p className="muted">and {backlinks.length - 300} more</p>}
        </section>
      )}
    </article>
  );
}
