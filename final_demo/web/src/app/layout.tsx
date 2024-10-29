import type { Metadata } from "next";
import { Ubuntu } from "next/font/google";
import ChakraUIProvider from "@/providers/chakra-ui";
import ReactQueryProvider from "@/providers/react-query";
import ICAgentProvider from "@/providers/ic-reactor";

const ubuntu = Ubuntu({
  weight: ["300", "400", "500", "700"],
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "Medblock",
  description: "Medblock is a blockchain-based medical records system.",
  icons: "/logo-only.png",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={ubuntu.className}>
        <ICAgentProvider>
          <ReactQueryProvider>
            <ChakraUIProvider>
              {children}
            </ChakraUIProvider>
          </ReactQueryProvider>
        </ICAgentProvider>
      </body>
    </html>
  );
}
