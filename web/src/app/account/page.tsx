import type { Metadata } from "next";
import { redirect } from "next/navigation";
import { REQUIRE_VERIFIED_EMAIL } from "@/lib/config";
import { listSessions } from "@/lib/db";
import { deviceName, formatDate, timeAgo } from "@/lib/format";
import { characterOf } from "@/lib/game";
import { currentSession } from "@/lib/session";
import { ChangePasswordForm, ResendButton, SignOutButton, SignOutOthersButton } from "./AccountForms";

export const metadata: Metadata = { title: "My account · ROSE Online" };
export const dynamic = "force-dynamic";

export default async function AccountPage() {
  const signedIn = await currentSession();
  if (!signedIn) redirect("/login");
  const { account, session } = signedIn;
  const [character, sessions] = [await characterOf(account), listSessions(account.id)];

  return (
    <div className="stack">
      <section className="card wide">
        <div className="card-head">
          <h1>My account</h1>
          <SignOutButton />
        </div>
        <dl className="facts">
          <dt>Email</dt>
          <dd>
            {account.email}{" "}
            {account.email_verified_at ? (
              <span className="badge ok">Confirmed</span>
            ) : (
              <span className="badge warn">Not confirmed</span>
            )}
          </dd>
          <dt>Member since</dt>
          <dd>{formatDate(account.created_at)}</dd>
          <dt>Password changed</dt>
          <dd>{formatDate(account.password_changed_at)}</dd>
        </dl>
        {!account.email_verified_at && (
          <div className="message warn">
            <p>
              {REQUIRE_VERIFIED_EMAIL
                ? "Confirm your email to play: open the link we sent you."
                : "Confirm your email so you can always get back into your account."}{" "}
              No email? Check the spam folder, or send a new link.
            </p>
            <ResendButton />
          </div>
        )}
      </section>

      <section className="card wide">
        <h2>My character</h2>
        {character === "unavailable" ? (
          <p className="muted">The game server can&apos;t be reached right now, so your character can&apos;t be shown.</p>
        ) : character ? (
          <div className="character">
            <div className={`portrait ${character.gender}`} aria-hidden>
              {character.name.slice(0, 1)}
            </div>
            <div>
              <div className="character-name">
                {character.name}{" "}
                {character.online ? <span className="badge ok">In game</span> : <span className="badge">Offline</span>}
              </div>
              <div className="muted">
                Level {character.level} {character.className}
              </div>
            </div>
          </div>
        ) : (
          <p className="muted">No character yet. Start the game and sign in to make one.</p>
        )}
      </section>

      <section className="card wide">
        <h2>Change password</h2>
        <ChangePasswordForm />
      </section>

      <section className="card wide">
        <div className="card-head">
          <h2>Signed-in devices</h2>
          {sessions.length > 1 && <SignOutOthersButton />}
        </div>
        <ul className="devices">
          {sessions.map((s, i) => (
            <li key={`${s.created_at}-${i}`}>
              <span>{deviceName(s.user_agent)}</span>
              <span className="muted">
                {s.id_hash === session.id_hash ? "This device" : `Last seen ${timeAgo(s.last_seen_at)}`} · since{" "}
                {formatDate(s.created_at)}
              </span>
            </li>
          ))}
        </ul>
        <p className="hint">The game itself doesn&apos;t stay signed in: it asks for your password each time.</p>
      </section>
    </div>
  );
}
