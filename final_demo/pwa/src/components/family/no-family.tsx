"use client"

import { Button, Flex, Icon, Image, Text } from "@chakra-ui/react";
import FamilyBackNavigation from "./back-navigation";
import { FaUserPlus } from "react-icons/fa6";

export default function NoFamilyView() {
  return (
    <Flex
      w={"full"}
      direction={'column'}
      gap={5}
      justify={'space-between'}
      h={'full'}
    >
      <FamilyBackNavigation />
      <Flex flex={1} direction={'column'} align={'center'} justify={'center'} gap={5}>
        <Image src="/assets/female-doctor.png" alt="No Family" />
        <Flex direction={'column'} align={'center'} gap={1}>
          <Text fontSize={'lg'} fontWeight={'bold'} color={'neutral.700'}>
            You Don’t Have Family List
          </Text>
          <Text
            fontSize={'sm'}
            color={'neutral.500'}
            textAlign={'center'}
            lineHeight={'1.7'}
          >
            Click “Create Family” To <br />
            Make a Family List
          </Text>
        </Flex>
      </Flex>
      <Button
        colorScheme="primary"
        w={"full"}
        bg={"primary.700"}
        rounded={"2xl"}
        fontSize={'sm'}
        py={6}
        gap={2}
        leftIcon={
          <Icon as={FaUserPlus} boxSize={5} />
        }
      >
        Create Family
      </Button>
    </Flex>
  )
}