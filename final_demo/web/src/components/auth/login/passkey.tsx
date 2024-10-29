"use client"

import { ButtonPasskey } from "@/constants/contents/auth/login/button";
import { Button, Icon, Text } from "@chakra-ui/react";

export default function PasskeyButtonLogin() {
  const { icon, label } = ButtonPasskey;
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
      leftIcon={
        <Icon as={icon} boxSize={6} color={"#622E8A"} />
      }
    >
      <Text as={"span"} color={"neutral.700"} fontWeight={'medium'}>
        {label}
      </Text>
    </Button>
  )
}