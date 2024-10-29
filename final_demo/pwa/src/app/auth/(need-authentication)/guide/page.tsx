import { Box, Flex, Text } from "@chakra-ui/react";
import AuthGuideButton from "@/components/auth/guide/button";
import AuthGuideHeader from "@/components/auth/guide/header";

export default function AuthGuidePage() {
  return (
    <Flex
      w={'full'}
      direction={"column"}
      justify={'space-between'}
      align={'center'}
      gap={9}
      h={'full'}
    >
      <Flex direction={'column'} w={'full'} pt={8} gap={3}>
        <AuthGuideHeader />
        <Box className="custom-pagination" />
      </Flex>

      <Flex direction={"column"} gap={4} w={'full'}
        align={'center'}>
        <Text as="desc"
          textAlign={'center'}
          w={"xs"}
          lineHeight={1.5}
          fontSize={'sm'}
        >
          By using Medblock pasport, you agree to the {" "}
          <Text as="span" color={"blue.200"}>
            terms
          </Text>
          {""} and {" "}
          <Text as="span" color={"blue.200"}>
            privacy policy
          </Text>
        </Text>
        <AuthGuideButton />
      </Flex>
    </Flex>
  )
}