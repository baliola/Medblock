"use client"

import { Flex, Text } from "@chakra-ui/react";
import { useProfileStore } from "@/store/profile-store";
import { RegistrationHeader } from "@/constants/contents/auth/registration/header";

import { AuthHeader } from "@/components/auth/header";
import UserRegistration from "@/components/auth/registration";
import { useUserPrincipal } from "@ic-reactor/react";
import { useEffect } from "react";

export default function AuthRegistrationPage() {
  const patientData = useProfileStore((state) => state.profile);
  const principal = useUserPrincipal()

  useEffect(() => {
    console.log("PRINCIPAL Text", principal?.toText());
  }, [principal]);

  return (
    <Flex
      w={'full'}
      direction={"column"}
      h={'full'}
      pt={5}
    >
      <AuthHeader size="xs" />
      <Flex
        direction={"column"}
        gap={4}
        w={'full'}
        flex={1}
      >
        <Text fontSize={'sm'}>
          {RegistrationHeader.title}
        </Text>
        <UserRegistration initialData={patientData} />
      </Flex>
    </Flex>
  )
}