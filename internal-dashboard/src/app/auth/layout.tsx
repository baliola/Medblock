"use client"


import { providerCanisterId } from "@/config/canisters/providers.canister";
import { ProviderActor } from "@/services/providers";
import { useAuth } from "@ic-reactor/react";
import { useRouter } from "next/navigation";
import { useEffect } from "react";

const LayoutChecking = ({ children }: { children: React.ReactNode }) => {
  const { authenticated, authenticating } = useAuth();
  const router = useRouter();

  console.log("AUTHENTICATED AUTH", authenticated)
  console.log("AUTHENTICATING AUTH", authenticating)

  if (authenticated && !authenticating) {
    router.replace("/")
    return;
  }

  useEffect(() => {

  }, [])

  return children
}

export default function AuthLayout({
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