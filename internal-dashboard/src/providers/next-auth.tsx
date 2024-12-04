"use client";

import { SessionProvider } from "next-auth/react";
import { Session } from "next-auth";
import { ReactNode } from "react";

interface NextAuthProviderProps {
  session: Session | null;
  children: ReactNode;
}

export const NextAuthProvider = ({
  session,
  children,
}: NextAuthProviderProps) => {
  return <SessionProvider session={session}>{children}</SessionProvider>;
};
