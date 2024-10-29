"use client"

import { ButtonGoogle } from "@/constants/contents/auth/login/button";
import { Button, Icon } from "@chakra-ui/react";

export default function GoogleButtonLogin() {
  return (
    <Button
      variant={"outline"}
      colorScheme="primary"
      size="lg"
      w={"full"}
      rounded={"2xl"}
      py={7}
      border={"2px"}
      _hover={{ bg: 'primary.100' }}
    >
      <Icon as={ButtonGoogle.icon} boxSize={8} />
    </Button>
  )
}