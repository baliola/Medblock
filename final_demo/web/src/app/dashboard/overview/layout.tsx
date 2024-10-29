import DashboardLayout from "@/layouts/dashboard"
import { type Metadata } from "next";

export const metadata: Metadata = {
  title: "Medblock | Overview",
  description: "Medblock is a blockchain-based medical records system.",
};

export default function Layout({
  children
}: {
  children: React.ReactNode
}) {
  return (
    <DashboardLayout activeLink="overview">
      {children}
    </DashboardLayout>
  )
}