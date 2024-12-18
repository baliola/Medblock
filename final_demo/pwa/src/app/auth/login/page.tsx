"use client"
import dynamic from "next/dynamic";
import { Flex, Text } from "@chakra-ui/react";

import { AuthHeader } from "@/components/auth/header";
import AuthOtherLoginOptions from "@/components/auth/login/other-login";

const NFIDButtonLogin = dynamic(
  () => import('@/components/auth/login/nfid'), {
  ssr: false
});

export default function LoginPage() {
  return (
    <Flex
      direction={'column'}
      align={'center'}
      w={'full'}
      gap={5}
    >
      <Text>Ini Text Tesiting</Text>
      <AuthHeader size="lg" />
      <NFIDButtonLogin />
      <AuthOtherLoginOptions />
    </Flex>
  )
}