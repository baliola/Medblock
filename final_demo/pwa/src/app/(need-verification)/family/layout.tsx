import MainLayout from "@/layouts/main";
import { Metadata } from "next";
import { redirect } from "next/navigation";

export const metadata: Metadata = {
  title: "Medblock Passport | My Family",
  description: "List of my family members for emr",
};

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  redirect("/home");
  // return (
  //   <MainLayout
  //     active="family"
  //     displayHeader={false}
  //   >
  //     {children}
  //   </MainLayout>
  // )
}