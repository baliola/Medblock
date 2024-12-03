"use client"

import { ButtonPasskey } from "@/constants/contents/auth/login/button";
import { Button, ButtonProps, Icon, Text } from "@chakra-ui/react";

export default function PasskeyButtonLogin({ ...props }: ButtonProps) {
  return (
    <Button
      variant={"outline"}
      colorScheme="primary"
      size="lg"
      w={"full"}
      rounded={"2xl"}
      border={"2px"}
      _hover={{ bg: 'primary.100' }}
      fontSize={'sm'}
      leftIcon={
        <Icon
          as={ButtonPasskey.icon}
          boxSize={6}
          color={"#622E8A"}
        />
      }
      {...props}
    >
      <Text as={"span"} color={"neutral.700"} fontWeight={'medium'}>
        {ButtonPasskey.label}
      </Text>
    </Button>
  )
}