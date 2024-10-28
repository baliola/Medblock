import MainLayout from "@/layouts/main";
import { Metadata } from "next";
import { redirect } from "next/navigation";

export const metadata: Metadata = {
  title: "Medblock Passport | My Insurance",
  description: "List of my insurance information",
};

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {

  redirect("/home");

  // return (
  //   <MainLayout
  //     active="insurance"
  //     displayHeader={false}
  //   >
  //     {children}
  //   </MainLayout>
  // )
}