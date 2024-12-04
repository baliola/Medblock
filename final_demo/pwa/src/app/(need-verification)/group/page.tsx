import PatientGroupList from "@/components/group/list";
import { Flex } from "@chakra-ui/react"

export default function GroupPage() {
  return (
    <Flex
      w={"full"}
      direction={'column'}
      gap={5}
      justify={"start"}
      h={'full'}
    >
      <PatientGroupList />
    </Flex>
  )
}