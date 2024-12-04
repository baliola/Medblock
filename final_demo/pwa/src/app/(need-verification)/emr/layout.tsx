import MainLayout from "@/layouts/main";
import { Metadata } from "next";

export const metadata: Metadata = {
  title: "Medblock Passport | My Emr",
  description: "List of my medical records",
};

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <MainLayout
      active="emr"
      displayHeader={false}
    >
      {children}
    </MainLayout>
  )
}