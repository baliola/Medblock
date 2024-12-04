"use client"

import { verifyButton, walletButton } from "@/constants/contents/auth/guide/button";
import { Button, Flex } from "@chakra-ui/react";
import { useAuthState } from "@ic-reactor/react";
import { useRouter } from "next/navigation";

const WalletButton = () => {
  const { register, signin } = walletButton;
  return (
    <Flex direction={"column"} w={"full"} gap={1}>
      <Button
        colorScheme="primary"
        w={"full"}
        bg={"primary.700"}
        rounded={"2xl"}
        fontSize={'sm'}
        py={6}
      >
        {register.label}
      </Button>
      <Button
        variant={"ghost"}
        colorScheme="primary"
        w={"full"}
        rounded={"2xl"}
        fontSize={'sm'}
        color={"primary.700"}
        py={6}
      >
        {signin.label}
      </Button>
    </Flex>
  )
}

const VerifyButton = () => {
  const router = useRouter();
  return (
    <Button
      colorScheme="primary"
      w={"full"}
      bg={"primary.700"}
      rounded={"2xl"}
      fontSize={'sm'}
      py={6}
      onClick={() => router.replace(verifyButton.redirect)}
    >
      {verifyButton.label}
    </Button>
  )
}

export default function AuthGuideButton() {
  const { authenticated } = useAuthState();

  return authenticated
    ? <VerifyButton />
    : <WalletButton />
}