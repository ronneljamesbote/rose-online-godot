// A zone's map with markers. The page's `map` gives the image and the world coordinates of
// its edges; every list of rows in the data block with numeric x and y (monster spawns,
// NPCs, warp gates) is drawn on it, each marker linking to the row's first [[link]].

import { LINK_RE, linkHref, normalizeLink, type Wiki, type WikiData } from "@/lib/wiki";
import { label } from "./DataView";

interface MapInfo {
  image: string;
  width: number;
  height: number;
  left: number;
  top: number;
  right: number;
  bottom: number;
}

const COLOURS = ["#ff8a80", "#9be29b", "#f4cf7a", "#8ab4ff", "#e39bff"];

function firstLink(row: Record<string, unknown>, wiki: Wiki): { href: string; title: string } | null {
  for (const v of Object.values(row)) {
    if (typeof v !== "string") continue;
    LINK_RE.lastIndex = 0;
    const m = LINK_RE.exec(v);
    if (m) {
      const target = normalizeLink(m[1]);
      return { href: linkHref(target), title: m[2] ?? wiki.pages.get(target)?.title ?? target };
    }
  }
  return null;
}

export default function ZoneMap({ data, wiki }: { data: WikiData; wiki: Wiki }) {
  const map = data.map as MapInfo | undefined;
  if (!map || !map.image) return null;
  const layers = Object.entries(data).filter(
    ([, v]) =>
      Array.isArray(v) &&
      v.some((r) => r && typeof r === "object" && typeof (r as Record<string, unknown>).x === "number"),
  ) as [string, Record<string, unknown>[]][];
  const px = (x: number) => ((x - map.left) / (map.right - map.left)) * map.width;
  const py = (y: number) => ((y - map.top) / (map.bottom - map.top)) * map.height;
  return (
    <figure className="zone-map">
      <svg viewBox={`0 0 ${map.width} ${map.height}`} role="img" aria-label={`Map of ${String(data.name ?? "")}`}>
        <image href={`/wiki-assets/${map.image}`} width={map.width} height={map.height} />
        {layers.map(([key, rows], li) =>
          rows.map((row, i) => {
            if (typeof row.x !== "number" || typeof row.y !== "number") return null;
            const link = firstLink(row, wiki);
            const cx = px(row.x);
            const cy = py(row.y);
            const dot = (
              <circle cx={cx} cy={cy} r={li === 0 ? 5 : 6} fill={COLOURS[li % COLOURS.length]} stroke="#120f1a" strokeWidth="1.5">
                <title>{link?.title ?? label(key)}</title>
              </circle>
            );
            return link ? (
              <a key={`${key}-${i}`} href={link.href}>
                {dot}
              </a>
            ) : (
              <g key={`${key}-${i}`}>{dot}</g>
            );
          }),
        )}
      </svg>
      <figcaption>
        {layers.map(([key], li) => (
          <span key={key} className="legend">
            <span className="swatch" style={{ background: COLOURS[li % COLOURS.length] }} /> {label(key)}
          </span>
        ))}
      </figcaption>
    </figure>
  );
}
