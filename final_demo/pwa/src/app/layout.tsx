import { ChakraUIProvider } from "@/providers/chakra-ui";
import ICAgentProvider from "@/providers/ic-reactor";
import { ReactQueryProvider } from "@/providers/react-query";
import type { Metadata } from "next";
import { Poppins } from "next/font/google";

const poppins = Poppins({
  weight: ["300", "400", "500", "700"],
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "Medisa Passport",
  description: "Medisa Passport is a EMR based on blockchain technology",
  generator: "Next.js",
  manifest: "/manifest.json",
  icons: "/logo.png",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={poppins.className}>
        <ChakraUIProvider>
          <ReactQueryProvider>
            <ICAgentProvider>{children}</ICAgentProvider>
          </ReactQueryProvider>
        </ChakraUIProvider>
      </body>
    </html>
  );
}
