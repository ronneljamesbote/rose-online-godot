// Images the wiki pages use (zone maps, icon sheets), served from wiki/assets.

import fs from "node:fs/promises";
import path from "node:path";
import { wikiDir } from "@/lib/wiki";

const TYPES: Record<string, string> = {
  ".png": "image/png",
  ".webp": "image/webp",
  ".jpg": "image/jpeg",
  ".json": "application/json",
};

export async function GET(_req: Request, { params }: { params: Promise<{ path: string[] }> }) {
  const parts = (await params).path;
  if (parts.some((p) => p === ".." || p.startsWith(".") || p.includes("\\"))) return new Response("Not found", { status: 404 });
  const root = path.join(wikiDir(), "assets");
  const file = path.join(root, ...parts);
  const type = TYPES[path.extname(file).toLowerCase()];
  if (!type || !file.startsWith(root + path.sep)) return new Response("Not found", { status: 404 });
  try {
    const data = await fs.readFile(file);
    return new Response(new Uint8Array(data), {
      headers: { "Content-Type": type, "Cache-Control": "public, max-age=3600", "X-Content-Type-Options": "nosniff" },
    });
  } catch {
    return new Response("Not found", { status: 404 });
  }
}
