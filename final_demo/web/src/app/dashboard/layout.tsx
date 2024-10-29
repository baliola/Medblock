"use client"

import { providerCanisterId } from "@/config/canisters/providers.canister";
import { ProviderActor, useProviderQuery } from "@/services/providers";
import { useToast } from "@chakra-ui/react";
import { Principal } from "@dfinity/principal";
import { useAuthState, useUserPrincipal } from "@ic-reactor/react";
import { useRouter } from "next/navigation";

const LayoutChecking = ({ children }: { children: React.ReactNode }) => {
  const { authenticated, authenticating } = useAuthState();
  const toast = useToast();
  const principal = useUserPrincipal();
  const router = useRouter();

  const {
    loading: hospitalLoading,
    error,
    call: getProviderInfo,
  } = useProviderQuery({
    functionName: "get_provider_info_with_principal",
    args: [{
      provider: [principal as Principal]
    }] as any,
    refetchOnMount: true
  });

  const checkProvider = async () => {
    if (principal) {
      // @ts-ignore
      await getProviderInfo([{
        provider: [principal as Principal]
      }])
        .catch((error) => {
          toast.closeAll();
          toast({
            title: "Unauthorized!",
            description: "Please register your hospital first to access all feature!",
            status: "error"
          });
          router.replace("/auth/hospital-registration");

          return null;
        })
    }
  }

  if (
    !authenticating &&
    !authenticated
  ) {
    router.replace("/auth/login")
    return;
  }

  if (error) {
    toast({
      title: "Unauthorized!",
      description: "Please register your hospital first to access all feature!",
      status: "info"
    });
    router.replace("/auth/hospital-registration");
    return;
  }

  if (!error && !hospitalLoading) {
    return children
  }
}

export default function ProtectedLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <ProviderActor
      canisterId={providerCanisterId}
    >
      <LayoutChecking>
        {children}
      </LayoutChecking>
    </ProviderActor>
  )
}