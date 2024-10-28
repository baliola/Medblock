import { Flex } from "@chakra-ui/react";
import InsuranceSubmit from "@/components/insurance/add/button";
import InsuranceAddList from "@/components/insurance/add/insurance-list";
import InsuranceBackNavigation from "@/components/insurance/add/back-navigation";

export default function InsuranceAddPage() {
  return (
    <Flex
      w={"full"}
      direction={'column'}
      gap={5}
      justify={'space-between'}
      h={'full'}
    >
      <InsuranceBackNavigation />

      <Flex direction={'column'} flex={1}>
        <InsuranceAddList />
      </Flex>

      <InsuranceSubmit />
    </Flex>
  )
}