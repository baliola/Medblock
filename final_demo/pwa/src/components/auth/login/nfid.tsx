"use client"

import { Fragment } from "react";
import { Button } from "@chakra-ui/react";
import useMedblockAuth from "@/hooks/useAuth";
import { ButtonNFID } from "@/constants/contents/auth/login/button";

export default function NFIDButtonLogin() {
  const { signIn, signOut } = ButtonNFID;

  const {
    authenticated,
    loginLoading,
    onLogin,
    onLogout
  } = useMedblockAuth();

  return (
    <Fragment>
      {authenticated
        ? (
          <Button
            colorScheme="red"
            size="lg"
            w={"full"}
            rounded={"2xl"}
            fontSize={'md'}
            onClick={onLogout}
          >
            {signOut.label}
          </Button>
        )
        : (
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
        )
      }
    </Fragment>
  )
}
