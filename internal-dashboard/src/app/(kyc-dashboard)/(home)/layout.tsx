import DashboardLayout from "@/layouts/dashboard"
import ICAgentProvider from "@/providers/ic-reactor"
import { Metadata } from "next"

export const metadata: Metadata = {
  title: "Medblock | HAM Dashboard",
  description: "Hospital Access Management Dashboard",
}

export default function Layout({
  children
}: Readonly<{
  children: React.ReactNode
}>) {
  return (
    <DashboardLayout activeLink="dashboard">
      <ICAgentProvider>
        {children}
      </ICAgentProvider>
    </DashboardLayout>
  )
}