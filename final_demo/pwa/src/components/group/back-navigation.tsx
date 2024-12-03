"use client"

import { Flex, Icon, Text } from "@chakra-ui/react";
import { useRouter } from "next/navigation";
import { FaChevronLeft } from "react-icons/fa6";

export default function GroupBackNavigation() {
  const router = useRouter();

  return (
    <Flex
      align={'center'}
      justify={'space-between'}
      gap={3}
      onClick={() => router.push("/home")}
      position={"fixed"}
      zIndex={20}
      top={0}
      left={0}
      w="full"
      h={16}
      px={6}
      bg={"white"}
    >
      <Icon as={FaChevronLeft} boxSize={5} color={'neutral.500'} />
      <Text
        fontSize={'lg'}
        fontWeight={'bold'}
      >
        My Group List
      </Text>
      <Icon as={FaChevronLeft} boxSize={5} color={'neutral.500'} visibility={'hidden'} />
    </Flex>
  )
}