"use client"

import { ButtonGoogle } from "@/constants/contents/auth/login/button";
import { Button, ButtonProps, Icon } from "@chakra-ui/react";

export default function GoogleButtonLogin({ ...props }: ButtonProps) {
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
      <Icon
        as={ButtonGoogle.icon}
        boxSize={6}
      />
    </Button>
  )
}