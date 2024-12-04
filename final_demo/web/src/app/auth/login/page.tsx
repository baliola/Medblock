import { Box, Flex, Text } from "@chakra-ui/react";
import { AuthHeader } from "@/components/auth/header";

import EIDButtonLogin from "@/components/auth/login/eid";
import GoogleButtonLogin from "@/components/auth/login/google";
import NFIDButtonLogin from "@/components/auth/login/nfid";
import PasskeyButtonLogin from "@/components/auth/login/passkey";
import { OtherLoginOptions } from "@/constants/contents/auth/login/button";

const Divider = () => (
  <Flex w={'full'} align={'center'} gap={5}>
    <Box as="span" bg="neutral.400" w={"full"} h={0.5} />
    <Text fontSize={'lg'} color={'neutral.500'} whiteSpace={'nowrap'}>
      {OtherLoginOptions.divider}
    </Text>
    <Box as="span" bg="neutral.400" w={"full"} h={0.5} />
  </Flex>
)

export default async function LoginPage() {
  return (
    <Flex
      direction={'column'}
      align={'center'}
      w={'full'}
      p={8}
      gap={5}
    >
      <AuthHeader />
      <NFIDButtonLogin />
      <Divider />

      <Flex direction={"column"} gap={5} w={'full'}>
        <Flex gap={5}>
          <GoogleButtonLogin />
          <EIDButtonLogin />
        </Flex>
        <PasskeyButtonLogin />
      </Flex>
      <Text color={"blue.200"} fontWeight={"medium"} fontSize={"lg"}>
        {OtherLoginOptions.label}
      </Text>
    </Flex>
  )
}