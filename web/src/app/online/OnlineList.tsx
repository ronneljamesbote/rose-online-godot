"use client";

import { useEffect, useState } from "react";

export interface OnlinePlayer {
  name: string;
  className: string;
  level: number;
}

const REFRESH_MS = 15_000;

/** The list refreshes itself every 15 seconds while the page is open. */
export default function OnlineList({ initial }: { initial: OnlinePlayer[] | null }) {
  const [players, setPlayers] = useState(initial);

  useEffect(() => {
    const timer = setInterval(async () => {
      if (document.hidden) return;
      try {
        const response = await fetch("/api/online", { cache: "no-store" });
        const answer = await response.json();
        setPlayers(response.ok ? answer.players : null);
      } catch {
        setPlayers(null);
      }
    }, REFRESH_MS);
    return () => clearInterval(timer);
  }, []);

  return (
    <div className="card wide">
      <h1>Who&apos;s online</h1>
      {players === null ? (
        <p className="lead">The game server can&apos;t be reached right now. This page tries again by itself.</p>
      ) : (
        <>
          <p className="lead">
            <span className="dot" /> <strong className="count">{players.length}</strong>{" "}
            {players.length === 1 ? "player is" : "players are"} in the game right now.
          </p>
          {players.length > 0 && (
            <table className="players">
              <thead>
                <tr>
                  <th>Character</th>
                  <th>Class</th>
                  <th className="num">Level</th>
                </tr>
              </thead>
              <tbody>
                {players.map((p) => (
                  <tr key={p.name}>
                    <td>{p.name}</td>
                    <td>{p.className}</td>
                    <td className="num">{p.level}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </>
      )}
    </div>
  );
}
