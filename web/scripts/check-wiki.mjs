#!/usr/bin/env node
// Checks the wiki: every page's data block parses, ids match file names, and every
// [[link]] points to a page. Usage: node web/scripts/check-wiki.mjs [wiki folder]

import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { parse } from "yaml";

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(process.argv[2] ?? path.join(here, "..", "..", "wiki"));
const LINK_RE = /\[\[([^\]|\\]+?)\\?(?:\|([^\]]+))?\]\]/g;
const STATUSES = new Set(["in-game", "changed-from-irose", "not-in-game-yet"]);

const files = [];
(function walk(dir, base) {
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    if (e.name.startsWith(".") || e.name === "assets") continue;
    const rel = base ? `${base}/${e.name}` : e.name;
    if (e.isDirectory()) walk(path.join(dir, e.name), rel);
    else if (e.name.endsWith(".md") && rel !== "README.md") files.push(rel);
  }
})(root, "");

const pages = new Set();
const folders = new Set();
for (const f of files) {
  const p = f.replace(/\.md$/, "").replace(/(^|\/)index$/, "");
  pages.add(p);
  let d = path.posix.dirname(f);
  while (d && d !== ".") {
    folders.add(d);
    d = path.posix.dirname(d);
  }
}

const problems = [];
let links = 0;
const collect = (v, out) => {
  if (typeof v === "string") for (const m of v.matchAll(LINK_RE)) out.push(m[1]);
  else if (Array.isArray(v)) v.forEach((x) => collect(x, out));
  else if (v && typeof v === "object") Object.values(v).forEach((x) => collect(x, out));
};

for (const f of files) {
  const text = fs.readFileSync(path.join(root, f), "utf8");
  let data = {};
  let body = text;
  if (text.startsWith("---\n")) {
    const end = text.indexOf("\n---", 3);
    if (end < 0) {
      problems.push(`${f}: data block is not closed with ---`);
      continue;
    }
    try {
      data = parse(text.slice(4, end + 1)) ?? {};
    } catch (e) {
      problems.push(`${f}: data block does not parse: ${e.message.split("\n")[0]}`);
      continue;
    }
    body = text.slice(text.indexOf("\n", end + 4) + 1);
  }
  if (data.status !== undefined && !STATUSES.has(data.status)) problems.push(`${f}: unknown status "${data.status}"`);
  const base = path.posix.basename(f, ".md");
  const m = /^(\d+)-/.exec(base);
  if (m && data.id !== undefined && String(data.id).split("/").pop() !== m[1]) {
    problems.push(`${f}: id ${data.id} does not match the file name`);
  }
  const targets = [];
  collect(data, targets);
  collect(body, targets);
  for (const t of targets) {
    links++;
    const target = t.trim().replace(/^\/+|\/+$/g, "").replace(/\.md$/, "").replace(/\/index$/, "");
    if (!pages.has(target) && !folders.has(target)) problems.push(`${f}: link to missing page [[${t}]]`);
  }
}

for (const p of problems.slice(0, Number(process.env.LIMIT ?? 200))) console.log(p);
if (problems.length > Number(process.env.LIMIT ?? 200)) console.log(`... and ${problems.length - Number(process.env.LIMIT ?? 200)} more`);
console.log(`${files.length} pages, ${links} links, ${problems.length} problems`);
process.exit(problems.length ? 1 : 0);
