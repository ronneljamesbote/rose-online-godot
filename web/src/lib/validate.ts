// Input rules for the account forms. Passwords follow NIST SP 800-63B: a minimum length,
// a generous maximum, no composition rules, and no password that is the email itself.

export const PASSWORD_MIN = 8;
export const PASSWORD_MAX = 128;
const EMAIL_MAX = 254;

export function normalizeEmail(email: unknown): string | null {
  if (typeof email !== "string") return null;
  const value = email.trim().toLowerCase();
  if (value.length === 0 || value.length > EMAIL_MAX) return null;
  // One @, something before it, a dot in the domain, no spaces.
  if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value)) return null;
  return value;
}

/** Why the password can't be used, or null when it is fine. */
export function passwordProblem(password: unknown, email?: string): string | null {
  if (typeof password !== "string") return "Enter a password.";
  const length = [...password].length;
  if (length < PASSWORD_MIN) return `Use at least ${PASSWORD_MIN} characters.`;
  if (length > PASSWORD_MAX) return `Use at most ${PASSWORD_MAX} characters.`;
  if (email && password.trim().toLowerCase() === email) return "Don't use your email as the password.";
  if (/^(.)\1+$/.test(password)) return "That password is too easy to guess.";
  return null;
}
