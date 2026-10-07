import type { Metadata } from "next";
import ResetForm from "./ResetForm";

export const metadata: Metadata = { title: "Choose a new password · ROSE Online" };

export default function ResetPage() {
  return <ResetForm />;
}
