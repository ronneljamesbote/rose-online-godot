import Link from "next/link";
import { onlinePlayers } from "@/lib/game";

export const dynamic = "force-dynamic";

export default async function Home() {
  const players = await onlinePlayers();
  return (
    <div className="card wide">
      <h1>Welcome to ROSE</h1>
      <p className="lead">
        Make an account here, then sign in with the same email and password in the game. Each account has its own
        character.
      </p>
      <div className="actions">
        <Link className="button" href="/signup">
          Create an account
        </Link>
        <Link className="button secondary" href="/login">
          Sign in
        </Link>
      </div>
      <ol className="steps">
        <li>Create an account with your email and a password.</li>
        <li>Confirm your email with the link we send you.</li>
        <li>Start the game, sign in, make your character and play.</li>
      </ol>
      {players && (
        <p className="online-line">
          <span className="dot" /> {players.length} {players.length === 1 ? "player" : "players"} online.{" "}
          <Link href="/online">See who</Link>
        </p>
      )}
      <div className="links">
        <Link href="/forgot-password">Forgot password?</Link>
      </div>
    </div>
  );
}
