import AuthLayout from "@/layouts/auth";
import { Metadata } from "next";

export const metadata: Metadata = {
  title: "Medblock Passport | Guide",
  description: "Guid to your account",
};

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <AuthLayout>
      {children}
    </AuthLayout>
  )
}