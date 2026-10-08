import type { Metadata } from "next";
import VerifyEmail from "./VerifyEmail";

export const metadata: Metadata = { title: "Confirm your email · ROSE Online" };

export default function VerifyEmailPage() {
  return <VerifyEmail />;
}
