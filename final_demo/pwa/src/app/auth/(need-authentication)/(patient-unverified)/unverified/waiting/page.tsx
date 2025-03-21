"use client"

import { kycWaiting } from "@/constants/contents/auth/registration/waiting";
import { Button, Flex, Image, Stack, Text } from "@chakra-ui/react";
import { useRouter } from "next/navigation";

export default function WaitingPage() {
  const router = useRouter();
  const { title, description, image, button } = kycWaiting;

  return (
    <Flex
      w={'full'}
      direction={"column"}
      justify={'space-between'}
      align={'center'}
      gap={2}
      h={'full'}
    >
      <Stack align={'center'} pt={5} my={"auto"}>
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
      <Button type="button"
        fontSize={'sm'}
        w={'full'}
        py={6}
        rounded={"xl"}
        colorScheme="gray"
        color={'primary.700'}
        size={'sm'}
        onClick={(() => router.replace(button.refresh.redirect))}
      >
        {button.refresh.label}
      </Button>
      <Button type="button"
        colorScheme="primary"
        bg={"primary.700"}
        fontSize={'sm'}
        w={'full'}
        py={6}
        rounded={"xl"}
        onClick={() => router.replace(button.back.redirect)}
      >
        {button.back.label}
      </Button>
    </Flex>
  )
}