import AuthLayout from "@/layouts/auth";
import ICAgentProvider from "@/providers/ic-reactor";

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <AuthLayout>
      <ICAgentProvider>
        {children}
      </ICAgentProvider>
    </AuthLayout>
  )
}