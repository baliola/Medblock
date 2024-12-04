"use client"

import { ButtonEID } from "@/constants/contents/auth/login/button";
import { Button, Image } from "@chakra-ui/react";

export default function EIDButtonLogin() {
  const { icon, label } = ButtonEID;
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
      <Image src={icon} alt={label} />
    </Button>
  )
}