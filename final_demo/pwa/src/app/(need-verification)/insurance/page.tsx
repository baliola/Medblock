import InsuranceAddButton from "@/components/insurance/button";
import ButtonNotifications from "@/components/notifications/button"
import HorizontalProfile from "@/components/profile/horizontal"
import { Flex, Text } from "@chakra-ui/react"

export default function InsurancePage() {
  const isHasInsurance = false;

  return (
    <Flex
      w={"full"}
      direction={'column'}
      gap={5}
    >
      <HorizontalProfile
        rightIcon={
          <ButtonNotifications />
        }
      />

      {isHasInsurance
        ? (
          <Text fontSize={'xl'} fontWeight={'bold'}>
            List of my insurance information
          </Text>
        )
        : <InsuranceAddButton />
      }
    </Flex>
  )
}