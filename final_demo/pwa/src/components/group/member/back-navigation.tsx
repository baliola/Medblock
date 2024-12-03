"use client"

import { Flex, Icon, Text } from "@chakra-ui/react";
import { useRouter } from "next/navigation";
import { FaChevronLeft } from "react-icons/fa6";

interface IGroupListBackNavigation {
  name: string
}

export default function GroupListBackNavigation({ props }: { props: IGroupListBackNavigation }) {
  const { name } = props
  const router = useRouter();

  return (
    <Flex
      align={'center'}
      justify={'space-between'}
      gap={3}
      onClick={() => router.push("/group")}
    >
      <Icon as={FaChevronLeft} boxSize={5} color={'neutral.500'} />
      <Text
        fontSize={'lg'}
        fontWeight={'bold'}
        textTransform={"capitalize"}
      >
        {name} Group
      </Text>
      <Icon as={FaChevronLeft} boxSize={5} color={'neutral.500'} visibility={'hidden'} />
    </Flex>
  )
}