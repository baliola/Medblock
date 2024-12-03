import { LOGO } from "@/constants/logo";
import { Container, Flex, Image, Spinner } from "@chakra-ui/react";

export default function LoadingScreen() {
  return (
    <Flex
      w={"full"}
      bg={"gray.100"}
      h={"100dvh"}
      maxH={"100dvh"}
    >
      <Container
        w={"full"}
        bg={"white"}
        overflow={"auto"}
        display={"flex"}
        flexDirection={"column"}
      >
        <Flex
          direction={'column'}
          flex={1}
          align={'center'}
          justify={'center'}
          gap={10}
        >
          <Image src={LOGO} alt="logo" w={24} />
          <Spinner size={"md"} colorScheme="primary" color="primary.700" />
        </Flex>
      </Container>
    </Flex>
  )
}