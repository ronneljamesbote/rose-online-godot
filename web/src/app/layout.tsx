import type { Metadata } from "next";
import Link from "next/link";
import { currentSession } from "@/lib/session";
import "./globals.css";

export const metadata: Metadata = {
  title: "ROSE Online",
  description: "Accounts for our ROSE Online server",
  robots: { index: false, follow: false },
};

export default async function RootLayout({ children }: { children: React.ReactNode }) {
  const signedIn = await currentSession();
  return (
    <html lang="en">
      <body>
        <header className="top">
          <Link href="/" className="brand">
            ROSE <span>Online</span>
          </Link>
          <nav>
            <Link href="/wiki">Wiki</Link>
            <Link href="/online">Who&apos;s online</Link>
            {signedIn ? (
              <Link href="/account" className="nav-account">
                My account
              </Link>
            ) : (
              <>
                <Link href="/login">Sign in</Link>
                <Link href="/signup" className="nav-account">
                  Create account
                </Link>
              </>
            )}
          </nav>
        </header>
        <main>{children}</main>
      </body>
    </html>
  );
}
