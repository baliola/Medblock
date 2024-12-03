import DashboardLayout from "@/layouts/dashboard"
import ICAgentProvider from "@/providers/ic-reactor"
import { Metadata } from "next"

export const metadata: Metadata = {
  title: "Medblock | Admin Dashboard",
  description: "Admin Management Dashboard",
}

export default function Layout({
  children
}: Readonly<{
  children: React.ReactNode
}>) {
  return (
    <DashboardLayout activeLink="admin">
      <ICAgentProvider>
        {children}
      </ICAgentProvider>
    </DashboardLayout>
  )
}