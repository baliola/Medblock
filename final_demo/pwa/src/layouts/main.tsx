import BottomBar, { type Active } from "@/components/bottom-bar";
import { LOGO } from "@/constants/logo";
import { Container, Flex, Image, Text } from "@chakra-ui/react";

interface MainLayout {
  displayHeader?: boolean;
  active: Active;
  children: React.ReactNode;
}

const Header = () => {
  return (
    <Flex as="header"
      w={"full"}
      gap={3}
      px={5} py={3}
      align={'center'}
    >
      <Image src={LOGO} alt="logo" w={12} h={10} />
      <Flex direction={'column'}>
        <Text
          color={"neutral.700"}
          fontWeight={"bold"}
          fontSize={"lg"}
        >
          Med
          <Text as="span" color="accent.700">
            block
          </Text>
        </Text>
        <Text
          fontSize={"sm"}
          color={"neutral.700"}
          fontWeight={"normal"}
        >
          Passport
        </Text>
      </Flex>
    </Flex>
  )
}

export default function MainLayout({
  displayHeader = true,
  active,
  children,
}: MainLayout) {
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
        h={"full"}
        bg={'white'}
      >
        {displayHeader && <Header />}

        <Flex as="main"
          flex={1}
          direction={"column"}
          overflowY={"auto"}
          px={5}
          py={displayHeader ? 0 : 5}
        >
          {children}
        </Flex>

        <Flex as="footer"
          bg="#F5F5F5"
          roundedTop={"2xl"}
          w={"full"}
          h={20}
          py={4}
          position={"sticky"}
          bottom={0}
          zIndex={10}
        >
          <BottomBar active={active} />
        </Flex>
      </Container>
    </Flex>
  )
}
