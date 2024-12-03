import MainLayout from "@/layouts/main";
import { Metadata } from "next";

export const metadata: Metadata = {
  title: "Medblock Passport | My Setting",
  description: "My setting information",
};

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <MainLayout
      active="setting"
      displayHeader={false}
    >
      {children}
    </MainLayout>
  )
}