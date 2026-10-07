import type { Metadata } from "next";
import SignupForm from "./SignupForm";

export const metadata: Metadata = { title: "Create an account · ROSE Online" };

export default function SignupPage() {
  return <SignupForm />;
}
