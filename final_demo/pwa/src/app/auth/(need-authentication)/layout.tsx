"use client"

import LoadingScreen from "@/layouts/loading";
import { useAuthState } from "@ic-reactor/react";
import { useRouter } from "next/navigation";

export default function ProtectedLayout({
  children
}: {
  children: React.ReactNode
}) {
  const { authenticating, authenticated } = useAuthState();
  const router = useRouter();

  if (
    !authenticating &&
    authenticated
  ) {
    return children
  }

  if (
    !authenticating &&
    !authenticated
  ) {
    router.replace("/auth/login");
    return;
  }

  return <LoadingScreen />
}