import type { Metadata } from "next";
import { redirect } from "next/navigation";
import { currentSession } from "@/lib/session";
import LoginForm from "./LoginForm";

export const metadata: Metadata = { title: "Sign in · ROSE Online" };

export default async function LoginPage() {
  if (await currentSession()) redirect("/account");
  return <LoginForm />;
}
