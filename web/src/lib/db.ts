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
  `);
  globalForDb.roseDb = database;
  return database;
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
    database.prepare("UPDATE account SET password_hash = ?, password_changed_at = ? WHERE id = ?").run(passwordHash, now, row.account_id);
    database.prepare("DELETE FROM password_reset WHERE account_id = ?").run(row.account_id);
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
