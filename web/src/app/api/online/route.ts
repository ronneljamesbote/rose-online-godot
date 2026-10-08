import { onlinePlayers } from "@/lib/game";
import { json } from "@/lib/request";

export const dynamic = "force-dynamic";

export async function GET() {
  const players = await onlinePlayers();
  if (!players) return json({ error: "The game server can't be reached right now." }, 503);
  return json({ count: players.length, players: players.map(({ name, className, level }) => ({ name, className, level })) });
}
