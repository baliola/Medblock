import { homeBanner } from "@/constants/contents/home/banner";
import { Flex, Image, Text } from "@chakra-ui/react";

export default function HomeBanner() {
  const { image, alt, header } = homeBanner;

  return (
    <Flex
      w={"full"}
      justify={'space-between'}
      align={'end'}
      gap={4}
      bgGradient="linear(to-r, #242DA9, #E2B2BD)"
      px={4}
      rounded={"2xl"}
    >
      <Image src={image} alt={alt} />
      <Flex
        direction={'column'}
        py={7}
        color={'neutral.50'}
        gap={1}
      >
        <Text fontSize={'md'} fontWeight={'bold'}>
          {header.title}
        </Text>
        <Text fontSize={'xs'}>
          {header.description}
        </Text>
      </Flex>
    </Flex>
  )
}