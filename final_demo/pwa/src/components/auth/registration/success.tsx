"use client"

import { registrationSuccess } from "@/constants/contents/auth/registration/success";
import { Button, Flex, Image, Stack, Text } from "@chakra-ui/react";
import { useRouter } from "next/navigation";

export default function AuthRegistrationSuccess() {
  const router = useRouter();
  const { header, button } = registrationSuccess;

  return (
    <Flex
      w={'full'}
      direction={"column"}
      justify={'space-between'}
      align={'center'}
      gap={9}
      h={'full'}
    >
      <Stack align={'center'} pt={5}>
        <Image src={header.image} alt={header.alt} w={40} />
        <Stack spacing={3} align={'center'} textAlign={'center'}>
          <Text fontSize={'xl'} fontWeight={'bold'}>
            {header.title}
          </Text>
          <Text fontSize={'md'}>
            {header.description}
          </Text>
        </Stack>
      </Stack>
      <Button type="submit"
        colorScheme="primary"
        bg={"primary.700"}
        fontSize={'sm'}
        w={'full'}
        py={6}
        rounded={"xl"}
        onClick={() => router.replace(button.redirect)}
      >
        {button.label}
      </Button>
    </Flex>
  )
}