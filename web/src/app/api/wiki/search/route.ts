import { loadWiki, searchIndex } from "@/lib/wiki";

export function GET() {
  return Response.json(searchIndex(loadWiki()), { headers: { "Cache-Control": "public, max-age=300" } });
}
