import type { Metadata } from "next";
import { Ubuntu } from "next/font/google";
import ChakraUIProvider from "@/providers/chakra-ui";
// import { NextAuthProvider } from "@/providers/next-auth";
// import { getServerSession } from "next-auth";
// import { authOptions } from "@/libs/next-auth";
import { ReactQueryProvider } from "@/providers/react-query";
import ICAgentProvider from "@/providers/ic-reactor";

const ubuntu = Ubuntu({
  weight: ["300", "400", "500", "700"],
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "Medblock",
  description: "Medblock is a blockchain-based medical records system.",
  icons: "/logo-only.png"
};

export default async function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  // const session = await getServerSession(authOptions);

  return (
    <html lang="en">
      <body className={ubuntu.className}>
        <ICAgentProvider>
        {/* <NextAuthProvider session={session}> */}
          <ReactQueryProvider>
            <ChakraUIProvider>
              {children}
            </ChakraUIProvider>
          </ReactQueryProvider>
        {/* </NextAuthProvider> */}
        </ICAgentProvider>
      </body>
    </html>
  );
}
