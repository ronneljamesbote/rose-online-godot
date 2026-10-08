import type { Metadata } from "next";
import { onlinePlayers } from "@/lib/game";
import OnlineList, { type OnlinePlayer } from "./OnlineList";

export const metadata: Metadata = { title: "Who's online · ROSE Online" };
export const dynamic = "force-dynamic";

export default async function OnlinePage() {
  const players = await onlinePlayers();
  const initial: OnlinePlayer[] | null = players && players.map(({ name, className, level }) => ({ name, className, level }));
  return <OnlineList initial={initial} />;
}
