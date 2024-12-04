"use client"

import { providerCanisterId } from "@/config/canisters/providers.canister";
import AuthLayout from "@/layouts/auth";
import { ProviderActor } from "@/services/providers";
import { useAuthState } from "@ic-reactor/react";
import { useRouter } from "next/navigation";

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const router = useRouter();

  const { authenticated, authenticating } = useAuthState();

  if (
    !authenticating &&
    !authenticated
  ) {
    router.replace("/auth/login");
  }

  return (
    <AuthLayout>
      <ProviderActor
        canisterId={providerCanisterId}
      >
        {children}
      </ProviderActor>
    </AuthLayout>
  )
}