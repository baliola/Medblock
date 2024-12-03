import EMRRevokeHeader from "@/components/emr/revoke/header";
import EMRRevoke from "@/components/emr/revoke";
import { Flex } from "@chakra-ui/react";

export default function EMRPage() {
  return (
    <Flex
      direction={'column'}
      gap={5}
      h={'full'}
      w={'full'}
    >
      <EMRRevokeHeader />
      <Flex direction={'column'} gap={5} flex={1}>
        <EMRRevoke />
      </Flex>
    </Flex>
  )
}