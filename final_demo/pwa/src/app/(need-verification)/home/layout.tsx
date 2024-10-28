import MainLayout from "@/layouts/main";
import { Metadata } from "next";

export const metadata: Metadata = {
  title: "Medblock Passport | Home",
  description: "Login to your account",
};

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <MainLayout
      active="home"
      displayHeader={false}
    >
      {children}
    </MainLayout>
  )
}