"use client"

import { Flex, Icon, Text } from "@chakra-ui/react";
import { useRouter } from "next/navigation";
import { IoAddCircle } from "react-icons/io5";

export default function InsuranceAddButton() {
  const router = useRouter();
  return (
    <Flex
      direction={'column'}
      justify={'center'}
      align={'center'}
      bg={"#D9D9D9"}
      rounded={"2xl"}
      color={'primary.500'}
      p={16}
      gap={2}
      transition={"all .3s"}
      _hover={{
        bg: "neutral.300",
        color: "primary.600"
      }}
      onClick={() => router.push("/insurance/add")}
    >
      <Icon as={IoAddCircle} boxSize={7} />
      <Text>
        Add Insurance
      </Text>
    </Flex>
  )
}