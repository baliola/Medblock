import { Metadata } from "next";

export const metadata: Metadata = {
  title: "Medblock Passport | Group Member",
  description: "List of my group members",
};

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  // redirect("/group");
  return (
    <>{children}</>
  )
}