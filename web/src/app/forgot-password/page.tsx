import type { Metadata } from "next";
import ForgotForm from "./ForgotForm";

export const metadata: Metadata = { title: "Forgot password · ROSE Online" };

export default function ForgotPage() {
  return <ForgotForm />;
}
