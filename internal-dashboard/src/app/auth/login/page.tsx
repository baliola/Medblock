import { AuthHeader } from "@/components/auth/header";
// import FormLogin from "@/components/auth/login";
import NFIDButtonLogin from "@/components/auth/nfid";
import { Flex } from "@chakra-ui/react";

export default function LoginPage() {
  return (
    <Flex
      direction={'column'}
      align={'center'}
      w={'full'}
      p={8}
      gap={12}
    >
      <AuthHeader />
      <Flex 
        flexDirection={"column"}
        alignItems={"center"}
        gap={4}
        mt={"auto"}
      >
        <NFIDButtonLogin />
      </Flex>
      {/* <FormLogin /> */}
    </Flex>
  )
}