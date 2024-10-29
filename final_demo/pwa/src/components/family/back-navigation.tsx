"use client"

import { Flex, Icon, Text } from "@chakra-ui/react";
import { useRouter } from "next/navigation";
import { FaChevronLeft } from "react-icons/fa6";

export default function FamilyBackNavigation() {
  const router = useRouter();

  return (
    <Flex
      align={'center'}
      justify={'space-between'}
      gap={3}
      onClick={() => router.push("/home")}
    >
      <Icon as={FaChevronLeft} boxSize={5} color={'neutral.500'} />
      <Text
        fontSize={'lg'}
        fontWeight={'bold'}
      >
        My Family List
      </Text>
      <Icon as={FaChevronLeft} boxSize={5} color={'neutral.500'} visibility={'hidden'} />
    </Flex>
  )
}