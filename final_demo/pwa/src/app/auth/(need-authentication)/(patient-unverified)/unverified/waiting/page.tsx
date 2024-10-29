"use client"

import { kycWaiting } from "@/constants/contents/auth/registration/waiting";
import { Button, Flex, Image, Stack, Text } from "@chakra-ui/react";
import { redirect, useRouter } from "next/navigation";

export default function WaitingPage() {

  redirect("/home");

  const router = useRouter();
  const { title, description, image, button } = kycWaiting;

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
        <Image src={image} alt={title} w={40} />
        <Stack spacing={3} align={'center'} textAlign={'center'} px={5}>
          <Text fontSize={'xl'} fontWeight={'bold'}>
            {title}
          </Text>
          <Text fontSize={'md'}>
            {description}
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