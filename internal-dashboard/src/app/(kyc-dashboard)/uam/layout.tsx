import DashboardLayout from "@/layouts/dashboard"
import { Metadata } from "next"

export const metadata: Metadata = {
  title: "Medblock | UAM Dashboard",
  description: "User Access Management Dashboard",
}

export default function Layout({
  children
}: Readonly<{
  children: React.ReactNode
}>) {
  return (
    <DashboardLayout activeLink="uam">
      {children}
    </DashboardLayout>
  )
}