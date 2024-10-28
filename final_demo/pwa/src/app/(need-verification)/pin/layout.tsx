import { Container, Flex } from "@chakra-ui/react";
import { Metadata } from "next";

export const metadata: Metadata = {
  title: "Medblock Passport | PIN",
  description: "Handle Pin to your account",
};

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <Flex
      direction={"column"}
      w={"full"}
      h={"100dvh"}
      bg={"gray.100"}
    >
      <Container
        flex={1}
        display={"flex"}
        flexDirection={"column"}
        p={0}
        h={'full'}
        bg={'white'}
        overflowY={"auto"}
      >
        {children}
      </Container>
    </Flex>
  )
}