"use client";

import { AuthHeader } from "@/components/auth/header";
import FormHospitalRegistration from "@/components/auth/hospital-registration/form";
import { hospitalRegistration } from "@/constants/contents/auth/hospital-registration";
import { ProviderInfoRequest } from "@/declarations/provider_registry/provider_registry.did";
import { useProviderQuery } from "@/services/providers";
import { Flex, Spinner, useToast } from "@chakra-ui/react";
import { Principal } from "@dfinity/principal";
import { useUserPrincipal } from "@ic-reactor/react";
import { useRouter } from "next/navigation";
import { useEffect } from "react";

export default function HospitalRegistrationPage() {
  const principal = useUserPrincipal();
  const router = useRouter();
  const toast = useToast();

  const { success, error } = hospitalRegistration;

  const { loading: hospitalLoading, call: getProviderInfo } = useProviderQuery({
    functionName: "get_provider_info_with_principal",
    refetchOnMount: false,
  });

  const checkUserRegistration = async () => {
    if (!principal) return;

    toast.closeAll();

    const request: ProviderInfoRequest = {
      provider: Array(principal as unknown as Principal),
    };

    try {
      // @ts-ignore
      const data = await getProviderInfo([request]);

      if (!data) {
        return toast({
          title: error.title,
          description: error.description,
          status: "info",
        });
      }

      router.replace(success.redirect);

      return toast({
        title: success.title,
        description: success.description,
        status: "info",
      });
    } catch (err) {
      return toast({
        title: error.title,
        description: error.description,
        status: "info",
      });
    }
  };

  console.log("principa", principal?.toText());

  useEffect(() => {
    if (principal) {
      checkUserRegistration();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [principal]);

  // Loading state
  if (hospitalLoading) {
    return (
      <Flex align="center" justify="center" h="100dvh">
        <Spinner size="xl" />
      </Flex>
    );
  }

  return (
    <Flex direction="column" align="center" w="full" p={8} gap={5}>
      <AuthHeader />
      <Flex w="full" pt={5}>
        <FormHospitalRegistration />
      </Flex>
    </Flex>
  );
}
