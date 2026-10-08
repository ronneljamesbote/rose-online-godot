// Accounts in SQLite (Node's built-in node:sqlite, so there is nothing native to install).

import fs from "node:fs";
import path from "node:path";
import { DatabaseSync } from "node:sqlite";
import { DATA_DIR } from "./config";

export interface Account {
  id: string;
  email: string;
  password_hash: string;
  created_at: number;
  password_changed_at: number;
  /** When the email was confirmed (a verification link or a password reset), or null. */
  email_verified_at: number | null;
  /** The account's SpacetimeDB identity (hex), looked up once and kept. */
  game_identity: string | null;
}

export interface Session {
  id_hash: string;
  account_id: string;
  created_at: number;
  last_seen_at: number;
  expires_at: number;
  user_agent: string;
}

const globalForDb = globalThis as unknown as { roseDb?: DatabaseSync };

export function db(): DatabaseSync {
  if (globalForDb.roseDb) return globalForDb.roseDb;
  fs.mkdirSync(DATA_DIR, { recursive: true, mode: 0o700 });
  const database = new DatabaseSync(path.join(DATA_DIR, "accounts.sqlite"));
  database.exec(`
    PRAGMA journal_mode = WAL;
    PRAGMA foreign_keys = ON;
    CREATE TABLE IF NOT EXISTS account (
      id TEXT PRIMARY KEY,
      email TEXT NOT NULL UNIQUE COLLATE NOCASE,
      password_hash TEXT NOT NULL,
      created_at INTEGER NOT NULL,
      password_changed_at INTEGER NOT NULL
    );
    CREATE TABLE IF NOT EXISTS password_reset (
      token_hash TEXT PRIMARY KEY,
      account_id TEXT NOT NULL REFERENCES account(id) ON DELETE CASCADE,
      expires_at INTEGER NOT NULL,
      used_at INTEGER
    );
    CREATE INDEX IF NOT EXISTS password_reset_account ON password_reset(account_id);
    CREATE TABLE IF NOT EXISTS email_verification (
      token_hash TEXT PRIMARY KEY,
      account_id TEXT NOT NULL REFERENCES account(id) ON DELETE CASCADE,
      expires_at INTEGER NOT NULL
    );
    CREATE INDEX IF NOT EXISTS email_verification_account ON email_verification(account_id);
    CREATE TABLE IF NOT EXISTS session (
      id_hash TEXT PRIMARY KEY,
      account_id TEXT NOT NULL REFERENCES account(id) ON DELETE CASCADE,
      created_at INTEGER NOT NULL,
      last_seen_at INTEGER NOT NULL,
      expires_at INTEGER NOT NULL,
      user_agent TEXT NOT NULL DEFAULT ''
    );
    CREATE INDEX IF NOT EXISTS session_account ON session(account_id);
  `);
  // Columns added after the first version.
  const columns = (database.prepare("PRAGMA table_info(account)").all() as { name: string }[]).map((c) => c.name);
  if (!columns.includes("email_verified_at")) database.exec("ALTER TABLE account ADD COLUMN email_verified_at INTEGER");
  if (!columns.includes("game_identity")) database.exec("ALTER TABLE account ADD COLUMN game_identity TEXT");
  globalForDb.roseDb = database;
  return database;
}

export function findAccountById(id: string): Account | undefined {
  return db().prepare("SELECT * FROM account WHERE id = ?").get(id) as Account | undefined;
}

export function findAccountByEmail(email: string): Account | undefined {
  return db().prepare("SELECT * FROM account WHERE email = ?").get(email) as Account | undefined;
}

/** False when the email is taken. */
export function createAccount(id: string, email: string, passwordHash: string): boolean {
  const now = Date.now();
  try {
    db()
      .prepare("INSERT INTO account (id, email, password_hash, created_at, password_changed_at) VALUES (?, ?, ?, ?, ?)")
      .run(id, email, passwordHash, now, now);
    return true;
  } catch (error) {
    if (error instanceof Error && /UNIQUE/.test(error.message)) return false;
    throw error;
  }
}

export function addPasswordReset(tokenHash: string, accountId: string, expiresAt: number): void {
  const database = db();
  // Only the newest link works.
  database.prepare("DELETE FROM password_reset WHERE account_id = ?").run(accountId);
  database.prepare("INSERT INTO password_reset (token_hash, account_id, expires_at) VALUES (?, ?, ?)").run(tokenHash, accountId, expiresAt);
}

/** Uses up a reset link and sets the new password. False if the link is unknown, used or expired. */
export function resetPassword(tokenHash: string, passwordHash: string): boolean {
  const database = db();
  const now = Date.now();
  database.exec("BEGIN IMMEDIATE");
  try {
    const row = database
      .prepare("SELECT account_id FROM password_reset WHERE token_hash = ? AND used_at IS NULL AND expires_at > ?")
      .get(tokenHash, now) as { account_id: string } | undefined;
    if (!row) {
      database.exec("ROLLBACK");
      return false;
    }
    // The link came to the account's email, so that proves the email too. Everyone signed
    // in with the old password is signed out.
    database
      .prepare("UPDATE account SET password_hash = ?, password_changed_at = ?, email_verified_at = COALESCE(email_verified_at, ?) WHERE id = ?")
      .run(passwordHash, now, now, row.account_id);
    database.prepare("DELETE FROM password_reset WHERE account_id = ?").run(row.account_id);
    database.prepare("DELETE FROM session WHERE account_id = ?").run(row.account_id);
    database.exec("COMMIT");
    return true;
  } catch (error) {
    database.exec("ROLLBACK");
    throw error;
  }
}

export function deleteExpiredResets(): void {
  db().prepare("DELETE FROM password_reset WHERE expires_at <= ?").run(Date.now());
}

export function addEmailVerification(tokenHash: string, accountId: string, expiresAt: number): void {
  const database = db();
  database.prepare("DELETE FROM email_verification WHERE account_id = ? OR expires_at <= ?").run(accountId, Date.now());
  database.prepare("INSERT INTO email_verification (token_hash, account_id, expires_at) VALUES (?, ?, ?)").run(tokenHash, accountId, expiresAt);
}

/** Confirms the email the link was sent to. False if the link is unknown or expired. */
export function verifyEmail(tokenHash: string): boolean {
  const database = db();
  const now = Date.now();
  const row = database
    .prepare("SELECT account_id FROM email_verification WHERE token_hash = ? AND expires_at > ?")
    .get(tokenHash, now) as { account_id: string } | undefined;
  if (!row) return false;
  database.prepare("UPDATE account SET email_verified_at = COALESCE(email_verified_at, ?) WHERE id = ?").run(now, row.account_id);
  database.prepare("DELETE FROM email_verification WHERE account_id = ?").run(row.account_id);
  return true;
}

/** Sets a new password and signs out every other session. */
export function changePassword(accountId: string, passwordHash: string, keepSessionHash: string): void {
  const database = db();
  database.prepare("UPDATE account SET password_hash = ?, password_changed_at = ? WHERE id = ?").run(passwordHash, Date.now(), accountId);
  database.prepare("DELETE FROM session WHERE account_id = ? AND id_hash != ?").run(accountId, keepSessionHash);
}

export function setGameIdentity(accountId: string, identity: string): void {
  db().prepare("UPDATE account SET game_identity = ? WHERE id = ?").run(identity, accountId);
}

export function addSession(idHash: string, accountId: string, expiresAt: number, userAgent: string): void {
  const now = Date.now();
  db()
    .prepare("INSERT INTO session (id_hash, account_id, created_at, last_seen_at, expires_at, user_agent) VALUES (?, ?, ?, ?, ?, ?)")
    .run(idHash, accountId, now, now, expiresAt, userAgent.slice(0, 200));
}

/** The session and its account, if the session is still valid. */
export function findSession(idHash: string): { session: Session; account: Account } | undefined {
  const database = db();
  const session = database.prepare("SELECT * FROM session WHERE id_hash = ? AND expires_at > ?").get(idHash, Date.now()) as
    | Session
    | undefined;
  if (!session) return undefined;
  const account = findAccountById(session.account_id);
  return account ? { session, account } : undefined;
}

/** Records a visit and pushes the expiry out, at most once a minute. */
export function touchSession(idHash: string, expiresAt: number): void {
  const now = Date.now();
  db()
    .prepare("UPDATE session SET last_seen_at = ?, expires_at = ? WHERE id_hash = ? AND last_seen_at < ?")
    .run(now, expiresAt, idHash, now - 60_000);
}

export function listSessions(accountId: string): Session[] {
  return db()
    .prepare("SELECT * FROM session WHERE account_id = ? AND expires_at > ? ORDER BY last_seen_at DESC")
    .all(accountId, Date.now()) as unknown as Session[];
}

export function deleteSession(idHash: string): void {
  db().prepare("DELETE FROM session WHERE id_hash = ?").run(idHash);
}

export function deleteOtherSessions(accountId: string, keepSessionHash: string): number {
  const result = db().prepare("DELETE FROM session WHERE account_id = ? AND id_hash != ?").run(accountId, keepSessionHash);
  return Number(result.changes);
}

export function deleteExpiredSessions(): void {
  db().prepare("DELETE FROM session WHERE expires_at <= ?").run(Date.now());
}
