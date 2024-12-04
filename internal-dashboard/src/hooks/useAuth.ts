"use client";

import { loginHost } from "@/config/agent";
import { ButtonNFID } from "@/constants/contents/auth/button";
import { useToast } from "@chakra-ui/react";
import { HttpAgent, Identity } from "@dfinity/agent";
import { AuthClient } from "@dfinity/auth-client";
import { useAuth } from "@ic-reactor/react";
// import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";

const makeAgent = ({ identity }: { identity: Identity }) => {
  return new HttpAgent({
    identity: identity,
    host: loginHost,
  });
};

export default function useMedblockAuth() {
  const toast = useToast();
  // const router = useRouter();

  const { signIn, signOut } = ButtonNFID;
  const [authenticated, setAuthenticated] = useState<boolean>(false);
  const [authenticating, setAuthenticating] = useState<boolean>(false);

  const {
    login,
    loginLoading,
    identity,
  } = useAuth({
    onLoginError() {
      window.location.reload();
      return;
    },
    onLoginSuccess() {
      makeAgent({
        identity: identity as unknown as Identity
      });

      window.location.href = '/';

      toast({
        title: signIn.onSuccess.title,
        description: signIn.onSuccess.description,
        status: "success",
        isClosable: true,
        position: "top-right",
        duration: 5000,
      });

      return;
    },
  });

  const onLogin = async () => {
    await login({
      identityProvider: loginHost,
    });
  };

  const onLogout = async () => {
    const client = await AuthClient.create();
    const isAuth = await client.isAuthenticated();

    if (isAuth) {
      await client.logout()
        .then(() => {
          setAuthenticated(false);

          toast({
            title: signOut.onSuccess.title,
            description: signOut.onSuccess.description,
            status: "success",
            isClosable: true,
            position: "top-right",
            duration: 5000,
          });

          /**
           * [IMPORTANT]
           * 
           * due to dfinity auth-client cache issue,
           * we need to force reload the page to clear the cache
           * 
           * if dont do this, the encryption key will be cached and
           * it will cause error when new user login to the app
           * 
           */
          window.location.href = '/auth/login';

          return; 
        })
        .catch((error) => {
          console.error(error)
        })  
    }
  };

  const initAuth = async () => {
    const authClient = await AuthClient.create({
      keyType: "Ed25519"
    });

    const isAuthenticated = await authClient.isAuthenticated();

    if (isAuthenticated) {
      setAuthenticated(true);
    }

    return;
  };

  useEffect(() => {
    const initialize = async () => {
      setAuthenticating(true)
      await initAuth();
      setAuthenticating(false)
    };

    if (typeof window !== "undefined") {
      initialize();
    }

  }, [authenticated]);

  return {
    onLogin,
    onLogout,
    loginLoading,
    authenticated,
    authenticating,
    identity,
  };
}
