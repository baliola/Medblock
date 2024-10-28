"use client"

import { Flex, Icon, Text } from "@chakra-ui/react";
import { useRouter } from "next/navigation";
import { FaChevronLeft } from "react-icons/fa6";

export default function InsuranceBackNavigation() {
  const router = useRouter();

  return (
    <Flex
      align={'center'}
      justify={'space-between'}
      gap={3}
      onClick={() => router.push("/insurance")}
    >
      <Icon as={FaChevronLeft} boxSize={5} color={'neutral.500'} />
      <Text
        fontSize={'lg'}
        fontWeight={'bold'}
      >
        Select Insurance Company
      </Text>
      <Icon as={FaChevronLeft} boxSize={5} color={'neutral.500'} visibility={'hidden'} />
    </Flex>
  )
}