import { Flex } from "@chakra-ui/react";

import HorizontalProfile from "@/components/profile/horizontal";
import ButtonNotifications from "@/components/notifications/button";
import HomeEMRHistory from "@/components/home";

export default function HomePage() {
  return (
    <Flex
      w={"full"}
      direction={'column'}
      gap={5}
      minH={'full'}
    >
      <HorizontalProfile
        rightIcon={
          <ButtonNotifications />
        }
      />

      <HomeEMRHistory />
    </Flex>
  )
}