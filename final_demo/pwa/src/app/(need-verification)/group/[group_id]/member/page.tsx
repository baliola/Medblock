"use client"

import PatienMemberGroupList from "@/components/group/member/list";
import { Flex } from "@chakra-ui/react";

export default function GroupMemberPage() {
  return (
    <Flex
      w={"full"}
      direction={'column'}
      gap={5}
      justify={"start"}
      h={'full'}
    >
      <PatienMemberGroupList />
    </Flex>
  )
}