import Link from "next/link";

export default function Home() {
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
        <Link className="button secondary" href="/forgot-password">
          Forgot password?
        </Link>
      </div>
      <ol className="steps">
        <li>Create an account with your email and a password.</li>
        <li>Start the game and sign in with them.</li>
        <li>Make your character and play.</li>
      </ol>
    </div>
  );
}
