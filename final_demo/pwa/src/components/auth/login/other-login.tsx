"use client"

import { Box, Flex, Text } from "@chakra-ui/react";
import GoogleButtonLogin from "@/components/auth/login/google";
import EIDButtonLogin from "@/components/auth/login/eid";
import PasskeyButtonLogin from "@/components/auth/login/passkey";
import { OtherLoginOptions } from "@/constants/contents/auth/login/button";
import useMedblockAuth from "@/hooks/useAuth";

const Divider = () => (
  <Flex w={'full'} align={'center'} gap={5}>
    <Box as="span" bg="neutral.400" w={"full"} h={0.5} />
    <Text fontSize={'sm'} color={'neutral.500'} whiteSpace={'nowrap'}>
      {OtherLoginOptions.divider}
    </Text>
    <Box as="span" bg="neutral.400" w={"full"} h={0.5} />
  </Flex>
)

export default function AuthOtherLoginOptions() {
  const { authenticated } = useMedblockAuth();

  return (
    !authenticated
      ? (
        <Flex
          direction={'column'}
          align={'center'}
          w={'full'}
          gap={5}
        >
          <Divider />
          <Flex direction={"column"} gap={5} w={'full'}>
            <Flex gap={5}>
              <GoogleButtonLogin />
              <EIDButtonLogin />
            </Flex>
            <PasskeyButtonLogin />
          </Flex>
          <Text color={"blue.200"} fontWeight={"medium"} fontSize={"sm"}>
            {OtherLoginOptions.label}
          </Text>
        </Flex>
      ) : null
  )
}