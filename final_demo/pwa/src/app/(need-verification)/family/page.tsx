import FamilyBackNavigation from "@/components/family/back-navigation";
import NoFamilyView from "@/components/family/no-family";
import { Flex } from "@chakra-ui/react"

export default function FamilyPage() {
  const isHasFamily = false;

  if (!isHasFamily) return <NoFamilyView />

  return (
    <Flex
      w={"full"}
      direction={'column'}
      gap={5}
      justify={'space-between'}
      h={'full'}
    >
      <FamilyBackNavigation />
      TEST
    </Flex>
  )
}