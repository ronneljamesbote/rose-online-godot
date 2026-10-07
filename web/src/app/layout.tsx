import type { Metadata } from "next";
import Link from "next/link";
import "./globals.css";

export const metadata: Metadata = {
  title: "ROSE Online",
  description: "Accounts for our ROSE Online server",
  robots: { index: false, follow: false },
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>
        <header className="top">
          <Link href="/" className="brand">
            ROSE <span>Online</span>
          </Link>
        </header>
        <main>{children}</main>
      </body>
    </html>
  );
}
