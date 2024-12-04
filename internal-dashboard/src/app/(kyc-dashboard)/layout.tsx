"use client"

import { providerCanisterId } from "@/config/canisters/providers.canister";
import { ProviderActor } from "@/services/providers";
import { useAuth } from "@ic-reactor/react";
import { useRouter } from "next/navigation";

const LayoutChecking = ({ children }: { children: React.ReactNode }) => {
  const { authenticated, authenticating } = useAuth();
  const router = useRouter();

  console.log("AUTHENTICATED DASHBOARD", authenticated)
  console.log("AUTHENTICATING DASHBOARD", authenticating)

  if (!authenticated && !authenticating) {
    router.replace("/auth/login")
    return;
  }

  return children
}

export default function ProtectedLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <ProviderActor canisterId={providerCanisterId}>
      <LayoutChecking>
        {children}
      </LayoutChecking>
    </ProviderActor>
  )
}