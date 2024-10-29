"use client";

import { useEffect, useState } from "react";
import { AgentProvider } from "@ic-reactor/react";
import { AuthClient } from "@dfinity/auth-client";

export default function ICAgentProvider({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const [mounted, setMounted] = useState<boolean>(false);
  const [clientReady, setClientReady] = useState<boolean>(false);

  const initialAuth = async () => {
    console.log("init from ic-reactor");

    await AuthClient.create({
      keyType: "Ed25519",
    });

    setClientReady(true);
  };

  /**
   * Mount the provider after the component is mounted on the client side
   */
  useEffect(() => {
    if (typeof window !== "undefined") {
      setMounted(true);
    }
  }, []);

  useEffect(() => {
    if (mounted) {
      initialAuth();
    }
  }, [mounted]);

  /**
   * Render only when the client is ready
   */
  return clientReady ? (
    <AgentProvider
      withLocalEnv={
        process.env.NODE_ENV === "development" ? true : false
      }
    >
      {children}
    </AgentProvider>
  ) : null
}