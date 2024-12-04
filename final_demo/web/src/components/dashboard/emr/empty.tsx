import { assets } from "@/constants/assets";
import { emrEmpty } from "@/constants/contents/dashboard/emr/empty";
import { Flex, Image, Stack, Text } from "@chakra-ui/react";

export default function EMRPatientEmpty() {
  const { title, description } = emrEmpty;
  return (
    <Flex
      w={'full'}
      justify={'center'}
      align={'center'}
      h={'85dvh'}
      bg={"primary.100"}
      rounded={"xl"}
      direction={'column'}
      gap={5}
    >
      <Image
        src={assets.emr_empty}
        alt="EMR Empty"
        w={{ base: '24', xl: 40 }}
      />

      <Stack gap={1} align={'center'}>
        <Text fontSize={{ base: 'md', xl: 'lg' }} fontWeight={'bold'}>
          {title}
        </Text>
        <Text fontSize={{ base: 'xs', xl: 'sm' }} color={"color.500"}>
          {description}
        </Text>
      </Stack>
    </Flex>
  )
}