import MainLayout from "@/layouts/main";
import { Metadata } from "next";

export const metadata: Metadata = {
  title: "Medblock Passport | My Group",
  description: "List of group",
};

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  // redirect("/group");
  return (
    <MainLayout
      active="group"
      displayHeader={false}
    >
      {children}
    </MainLayout>
  )
}