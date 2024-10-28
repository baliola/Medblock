"use client"

import { ButtonEID } from "@/constants/contents/auth/login/button";
import { Button, ButtonProps, Image } from "@chakra-ui/react";

export default function EIDButtonLogin({ ...props }: ButtonProps) {
  return (
    <Button
      variant={"outline"}
      colorScheme="primary"
      size="lg"
      w={"full"}
      rounded={"2xl"}
      border={"2px"}
      _hover={{ bg: 'primary.100' }}
      {...props}
    >
      <Image
        src={ButtonEID.icon}
        alt={ButtonEID.label}
        w={10}
      />
    </Button>
  )
}