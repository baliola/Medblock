"use client"

import { useEffect, useState } from "react";
import { useUserPrincipal } from "@ic-reactor/react";
import { useRouter } from "next/navigation";

export default function Layout({
  children
}: Readonly<{
  children: React.ReactNode
}>) {
  const router = useRouter();

  const [userHasPin, setUserHasPin] = useState<boolean>(false);
  const [loading, setLoading] = useState<boolean>(true);

  const principal = useUserPrincipal();

  useEffect(() => {
    if (!principal) return;

    const localPin = localStorage.getItem(`pin@${principal.toText()}`);

    if (localPin) {
      setUserHasPin(true);
      setLoading(false);
    }

    setLoading(false);

  }, [principal]);

  if (!loading && !userHasPin) {
    router.replace("/pin/add");
  }

  return children;
}