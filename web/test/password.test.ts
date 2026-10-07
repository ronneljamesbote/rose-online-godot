import assert from "node:assert/strict";
import { test } from "node:test";
import { hashPassword, verifyPassword } from "../src/lib/password.ts";
import { normalizeEmail, passwordProblem } from "../src/lib/validate.ts";

test("hashes verify and differ per salt", async () => {
  const a = await hashPassword("correct horse battery");
  const b = await hashPassword("correct horse battery");
  assert.notEqual(a, b);
  assert.match(a, /^scrypt\$131072\$8\$1\$/);
  assert.equal(await verifyPassword("correct horse battery", a), true);
  assert.equal(await verifyPassword("correct horse batterY", a), false);
  assert.equal(await verifyPassword("x", "not a hash"), false);
});

test("email and password rules", () => {
  assert.equal(normalizeEmail("  Ronnel@Example.COM "), "ronnel@example.com");
  assert.equal(normalizeEmail("no-at-sign"), null);
  assert.equal(normalizeEmail("a@b"), null);
  assert.equal(passwordProblem("short"), "Use at least 8 characters.");
  assert.equal(passwordProblem("aaaaaaaaaa"), "That password is too easy to guess.");
  assert.equal(passwordProblem("me@example.com", "me@example.com"), "Don't use your email as the password.");
  assert.equal(passwordProblem("a fine long password"), null);
});
