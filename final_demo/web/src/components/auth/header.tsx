import { authHeader } from "@/constants/contents/auth/header";
import { Flex, Text } from "@chakra-ui/react"

export const AuthHeader = () => {
  const { title, span, description } = authHeader;
  return (
    <Flex as="header"
      direction={'column'}
      align={'center'}
      gap={0}
    >
      <Text as="h1"
        color={"neutral.700"}
        fontWeight={'bold'}
        fontSize={'6xl'}
      >
        {title}
        <Text as="span" color="accent.700">
          {span}
        </Text>
      </Text>
      <Text as="p"
        fontSize={'2xl'}
        color={"neutral.700"}
        fontWeight={'bold'}
      >
        {description}
      </Text>
    </Flex>
  )
}