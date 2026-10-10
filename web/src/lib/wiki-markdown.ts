// Markdown to HTML for wiki pages: [[path|label]] links, ```math blocks (KaTeX), heading
// anchors. Raw HTML in a page is shown as text, never run.

import { Marked, type Tokens } from "marked";
import katex from "katex";
import { LINK_RE, linkHref, normalizeLink, type Wiki } from "./wiki";

export function escapeHtml(text: string): string {
  return text.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;");
}

function slug(text: string): string {
  return text
    .toLowerCase()
    .replace(/<[^>]+>/g, "")
    .replace(/&[a-z]+;/g, "")
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-|-$/g, "");
}

function makeMarked(wiki: Wiki | null) {
  const marked = new Marked({ gfm: true });
  marked.use({
    extensions: [
      {
        name: "wikilink",
        level: "inline",
        start(src: string) {
          const i = src.indexOf("[[");
          return i < 0 ? undefined : i;
        },
        tokenizer(src: string) {
          const m = /^\[\[([^\]|\\]+?)\\?(?:\|([^\]]+))?\]\]/.exec(src);
          if (!m) return undefined;
          return { type: "wikilink", raw: m[0], target: m[1], label: m[2] ?? "" };
        },
        renderer(token) {
          const t = token as unknown as { target: string; label: string };
          const target = normalizeLink(t.target);
          const page = wiki?.pages.get(target);
          const label = t.label || page?.title || target.split("/").pop() || target;
          const cls = wiki && !page ? ' class="missing"' : "";
          return `<a href="${escapeHtml(linkHref(target))}"${cls}>${escapeHtml(label)}</a>`;
        },
      },
    ],
    renderer: {
      html(token: Tokens.HTML | Tokens.Tag) {
        return escapeHtml(token.text);
      },
      code(token: Tokens.Code) {
        if (token.lang === "math") {
          try {
            return `<div class="math">${katex.renderToString(token.text, { displayMode: true, throwOnError: false })}</div>`;
          } catch {
            /* fall through to plain code */
          }
        }
        return `<pre><code>${escapeHtml(token.text)}</code></pre>`;
      },
      heading(token: Tokens.Heading) {
        const text = this.parser.parseInline(token.tokens);
        return `<h${token.depth} id="${slug(text)}">${text}</h${token.depth}>\n`;
      },
      blockquote(token: Tokens.Blockquote) {
        const inner = this.parser.parse(token.tokens);
        const open = /^\s*<p>\s*Open question/i.test(inner);
        return `<blockquote${open ? ' class="open-question"' : ""}>${inner}</blockquote>\n`;
      },
    },
  });
  return marked;
}

let cached: { wiki: Wiki | null; marked: Marked } | null = null;

function getMarked(wiki: Wiki | null): Marked {
  if (!cached || cached.wiki !== wiki) cached = { wiki, marked: makeMarked(wiki) };
  return cached.marked;
}

export function renderMarkdown(text: string, wiki: Wiki | null): string {
  return getMarked(wiki).parse(text, { async: false }) as string;
}

export function renderInline(text: string, wiki: Wiki | null): string {
  return getMarked(wiki).parseInline(text, { async: false }) as string;
}

export function hasLink(text: string): boolean {
  LINK_RE.lastIndex = 0;
  return LINK_RE.test(text);
}
