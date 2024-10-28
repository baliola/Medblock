import { ChakraUIProvider } from "@/providers/chakra-ui";
import ICAgentProvider from "@/providers/ic-reactor";
import { ReactQueryProvider } from "@/providers/react-query";
import type { Metadata } from "next";
import { Ubuntu } from "next/font/google";

const ubuntu = Ubuntu({
  weight: ["300", "400", "500", "700"],
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "Medblock Passport",
  description: "Medblock Passport is a EMR based on blockchain technology",
  generator: "Next.js",
  manifest: "/manifest.json",
  icons: "/logo.png"
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={ubuntu.className}>
        <ChakraUIProvider>
          <ReactQueryProvider>
            <ICAgentProvider>
              {children}
            </ICAgentProvider>
          </ReactQueryProvider>
        </ChakraUIProvider>
      </body>
    </html>
  )
}
