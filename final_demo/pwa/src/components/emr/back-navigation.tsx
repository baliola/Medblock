"use client"

import { emrBackNavigation } from "@/constants/contents/emr/back-navigation";
import { Flex, Icon, Text } from "@chakra-ui/react";
import { useRouter } from "next/navigation";
import { FaChevronLeft } from "react-icons/fa6";

export default function EMRBackNavigation() {
  const router = useRouter();
  const { detail } = emrBackNavigation;

  return (
    <Flex
      align={'center'}
      w={'fit-content'}
      gap={3}
      onClick={() => router.push(detail.redirect)}
    >
      <Icon as={FaChevronLeft} boxSize={5} color={'neutral.500'} />
      <Text
        fontSize={'lg'}
        fontWeight={'bold'}
      >
        {detail.label}
      </Text>
    </Flex>
  )
}