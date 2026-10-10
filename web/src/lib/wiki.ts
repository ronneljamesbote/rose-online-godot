// The game wiki: Markdown pages with a YAML data block, read from the repository's wiki/
// folder (WIKI_DIR). Pages are loaded once and kept in memory; in development they are
// read again when a file changes.

import fs from "node:fs";
import path from "node:path";
import { parse as parseYaml } from "yaml";

export type WikiData = Record<string, unknown>;

export interface WikiPage {
  /** Path inside wiki/ without .md, e.g. "monsters/1-jelly-bean"; a folder's index.md is "monsters". */
  path: string;
  /** File path relative to wiki/, for the Edit link. */
  file: string;
  title: string;
  kind: string;
  data: WikiData;
  body: string;
  /** Problem reading the data block, shown on the page. */
  error?: string;
}

export interface Wiki {
  pages: Map<string, WikiPage>;
  /** For each page, the pages that link to it. */
  backlinks: Map<string, Set<string>>;
  /** For each folder path, the pages directly inside it (not its index). */
  children: Map<string, string[]>;
}

export const LINK_RE = /\[\[([^\]|\\]+?)\\?(?:\|([^\]]+))?\]\]/g;

export function wikiDir(): string {
  if (process.env.WIKI_DIR) return process.env.WIKI_DIR;
  const local = path.resolve(process.cwd(), "wiki");
  return fs.existsSync(local) ? local : path.resolve(process.cwd(), "..", "wiki");
}

export const GITHUB_EDIT_BASE =
  process.env.WIKI_EDIT_BASE ?? "https://github.com/ronneljamesbote/rose-online-godot/edit/main/wiki/";

export function splitFrontMatter(text: string): { yaml: string | null; body: string } {
  if (!text.startsWith("---\n") && !text.startsWith("---\r\n")) return { yaml: null, body: text };
  const end = text.indexOf("\n---", 3);
  if (end < 0) return { yaml: null, body: text };
  const after = text.indexOf("\n", end + 4);
  return { yaml: text.slice(text.indexOf("\n") + 1, end + 1), body: after < 0 ? "" : text.slice(after + 1) };
}

export function parsePage(file: string, text: string): WikiPage {
  const rel = file.replace(/\\/g, "/").replace(/\.md$/, "");
  const pagePath = rel === "index" ? "" : rel.endsWith("/index") ? rel.slice(0, -"/index".length) : rel;
  const { yaml, body } = splitFrontMatter(text);
  let data: WikiData = {};
  let error: string | undefined;
  if (yaml !== null) {
    try {
      const parsed = parseYaml(yaml);
      if (parsed && typeof parsed === "object" && !Array.isArray(parsed)) data = parsed as WikiData;
    } catch (e) {
      error = e instanceof Error ? e.message : String(e);
    }
  }
  const heading = /^#\s+(.+)$/m.exec(body);
  const title = String(data.name ?? data.title ?? heading?.[1] ?? path.posix.basename(pagePath || "Wiki"));
  const kind = String(data.kind ?? "page");
  return { path: pagePath, file: file.replace(/\\/g, "/"), title, kind, data, body, error };
}

function walk(dir: string, base: string, out: string[]) {
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    if (entry.name.startsWith(".") || entry.name === "assets") continue;
    const full = path.join(dir, entry.name);
    const rel = base ? `${base}/${entry.name}` : entry.name;
    if (entry.isDirectory()) walk(full, rel, out);
    else if (entry.name.endsWith(".md") && rel !== "README.md") out.push(rel);
  }
}

function collectLinks(value: unknown, out: Set<string>) {
  if (typeof value === "string") {
    for (const m of value.matchAll(LINK_RE)) out.add(normalizeLink(m[1]));
  } else if (Array.isArray(value)) {
    for (const v of value) collectLinks(v, out);
  } else if (value && typeof value === "object") {
    for (const v of Object.values(value)) collectLinks(v, out);
  }
}

export function normalizeLink(target: string): string {
  return target.trim().replace(/^\/+|\/+$/g, "").replace(/\.md$/, "").replace(/\/index$/, "");
}

let cache: { wiki: Wiki; stamp: number; checked: number } | null = null;

function newestChange(dir: string): number {
  let newest = 0;
  const visit = (d: string) => {
    for (const entry of fs.readdirSync(d, { withFileTypes: true })) {
      if (entry.name.startsWith(".")) continue;
      const full = path.join(d, entry.name);
      if (entry.isDirectory()) {
        if (entry.name !== "assets") visit(full);
      } else if (entry.name.endsWith(".md")) newest = Math.max(newest, fs.statSync(full).mtimeMs);
    }
  };
  visit(dir);
  return newest;
}

export function loadWiki(): Wiki {
  const dir = wikiDir();
  const dev = process.env.NODE_ENV !== "production";
  if (cache && (!dev || Date.now() - cache.checked < 2000)) return cache.wiki;
  if (cache && dev) {
    const stamp = newestChange(dir);
    cache.checked = Date.now();
    if (stamp <= cache.stamp) return cache.wiki;
  }
  const files: string[] = [];
  if (fs.existsSync(dir)) walk(dir, "", files);
  const pages = new Map<string, WikiPage>();
  for (const file of files) {
    const page = parsePage(file, fs.readFileSync(path.join(dir, file), "utf8"));
    pages.set(page.path, page);
  }
  const backlinks = new Map<string, Set<string>>();
  const children = new Map<string, string[]>();
  for (const page of pages.values()) {
    const links = new Set<string>();
    collectLinks(page.data, links);
    collectLinks(page.body, links);
    for (const target of links) {
      if (target === page.path) continue;
      if (!backlinks.has(target)) backlinks.set(target, new Set());
      backlinks.get(target)!.add(page.path);
    }
    if (page.path) {
      const parent = page.path.includes("/") ? page.path.slice(0, page.path.lastIndexOf("/")) : "";
      if (!children.has(parent)) children.set(parent, []);
      children.get(parent)!.push(page.path);
    }
  }
  for (const list of children.values()) list.sort(comparePaths);
  const wiki = { pages, backlinks, children };
  cache = { wiki, stamp: dev ? newestChange(dir) : 0, checked: Date.now() };
  return wiki;
}

/** Sort "monsters/2-x" before "monsters/10-y" by the leading number, then by name. */
export function comparePaths(a: string, b: string): number {
  const na = /\/(\d+)-[^/]*$/.exec(a);
  const nb = /\/(\d+)-[^/]*$/.exec(b);
  if (na && nb && na[1] !== nb[1]) return Number(na[1]) - Number(nb[1]);
  return a.localeCompare(b);
}

/** Folders that have pages under them but no index page of their own still list their pages. */
export function isFolder(wiki: Wiki, pagePath: string): boolean {
  return wiki.children.has(pagePath);
}

export function linkHref(target: string): string {
  const t = normalizeLink(target);
  return t ? `/wiki/${t}` : "/wiki";
}

export interface SearchEntry {
  p: string;
  t: string;
  k: string;
  i?: string;
}

export function searchIndex(wiki: Wiki): SearchEntry[] {
  const out: SearchEntry[] = [];
  for (const page of wiki.pages.values()) {
    if (!page.path) continue;
    const id = page.data.id;
    out.push({ p: page.path, t: page.title, k: page.kind, ...(id !== undefined ? { i: String(id) } : {}) });
  }
  return out.sort((a, b) => a.t.localeCompare(b.t));
}
