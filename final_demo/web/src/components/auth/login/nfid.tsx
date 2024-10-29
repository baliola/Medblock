"use client";

import { ButtonNFID } from "@/constants/contents/auth/login/button";
import useMedblockAuth from "@/hooks/useAuth";
import { Button } from "@chakra-ui/react";
import { Fragment } from "react";

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
          <Button colorScheme="accent"
            size="lg"
            w={"full"}
            bg={"accent.700"}
            rounded={"2xl"}
            py={7}
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
            py={7}
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
