// Sends email through SMTP when SMTP_HOST is set. Without it (local testing) the message is
// written to the website's log instead, so the reset link can be copied from there.

import nodemailer from "nodemailer";

export async function sendMail(to: string, subject: string, text: string): Promise<void> {
  const host = process.env.SMTP_HOST;
  if (!host) {
    console.warn(`[mail] SMTP_HOST is not set, so this email was not sent. To ${to}: ${subject}\n${text}`);
    return;
  }
  const port = Number(process.env.SMTP_PORT ?? 587);
  const transport = nodemailer.createTransport({
    host,
    port,
    secure: port === 465,
    requireTLS: port !== 465 && process.env.SMTP_REQUIRE_TLS !== "0",
    auth: process.env.SMTP_USER ? { user: process.env.SMTP_USER, pass: process.env.SMTP_PASS ?? "" } : undefined,
  });
  await transport.sendMail({ from: process.env.MAIL_FROM ?? "ROSE <no-reply@localhost>", to, subject, text });
}
