"use client"

import { Button, Flex, Icon } from "@chakra-ui/react";
import { IoFilter } from "react-icons/io5";

export default function EMRFilter() {
  return (
    <Flex align={'center'} justify={'space-between'} gap={3}>
      <Button
        variant="outline"
        size={'lg'}
        rounded={'xl'}
        borderColor={'#A1A2A6'}
        p={2}
      >
        <Icon
          as={IoFilter}
          boxSize={5}
          color={'neutral.400'}
        />
      </Button>
    </Flex>
  )
}