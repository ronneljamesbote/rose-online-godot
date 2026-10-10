// An item or skill icon cut from the game's icon sheets (wiki/assets/icons). `spec` is
// "item/<icon number>" or "skill/<icon number>"; icons.json maps it to a sheet and a spot.

import fs from "node:fs";
import path from "node:path";
import { wikiDir } from "@/lib/wiki";

export type Spot = [string, number, number, number, number];
let icons: Record<string, Record<string, Spot>> | null = null;

export function lookup(spec: string): Spot | undefined {
  if (!icons) {
    try {
      icons = JSON.parse(fs.readFileSync(path.join(wikiDir(), "assets", "icons", "icons.json"), "utf8"));
    } catch {
      icons = {};
    }
  }
  const [group, n] = spec.split("/");
  return icons?.[group]?.[n];
}

export default function Icon({ spec }: { spec: string }) {
  const spot = lookup(spec);
  if (!spot) return null;
  const [sheet, x, y, w, h] = spot;
  return (
    <span
      className="wiki-icon"
      aria-hidden="true"
      style={{
        width: w,
        height: h,
        backgroundImage: `url(/wiki-assets/icons/${sheet})`,
        backgroundPosition: `-${x}px -${y}px`,
      }}
    />
  );
}
