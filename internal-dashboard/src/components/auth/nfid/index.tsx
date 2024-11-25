"use client"

import { Fragment } from "react";
import { Button } from "@chakra-ui/react";
import useMedblockAuth from "@/hooks/useAuth";
import { ButtonNFID } from "@/constants/contents/auth/button";

export default function NFIDButtonLogin() {
  const { signIn } = ButtonNFID;

  const {
    loginLoading,
    onLogin
  } = useMedblockAuth();

  return (
    <Fragment>
      <Button
        colorScheme="primary"
        size="lg"
        w={"full"}
        bg={"primary.700"}
        rounded={"2xl"}
        fontSize={'md'}
        onClick={onLogin}
        isLoading={loginLoading}
      >
        {signIn.label}
      </Button>
    </Fragment>
  )
}
