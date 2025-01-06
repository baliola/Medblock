"use client"

import { useRouter } from "next/navigation";
import { Button, Flex, Image, Stack, Text } from "@chakra-ui/react";
import { kycRejected } from "@/constants/contents/auth/registration/reject";
import useMedblockAuth from "@/hooks/useAuth";
import { ButtonNFID } from "@/constants/contents/auth/login/button";

export default function RejectedPage() {
  const router = useRouter();
  const { title, description, image, button } = kycRejected;
  const { signOut } = ButtonNFID;

  const {
    authenticated,
    onLogout
  } = useMedblockAuth();

  return (
    <Flex
      w={'full'}
      direction={"column"}
      justify={'space-between'}
      align={'center'}
      gap={0}
      h={'full'}
    >
      <Stack align={'center'} pt={8} my={"auto"}>
        <Image src={image} alt={title} w={40} />
        <Stack spacing={0} align={'center'} textAlign={'center'}>
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

      {
        authenticated &&
        <Button
          colorScheme="danger"
          w={"full"}
          rounded={"2xl"}
          fontSize={'sm'}
          py={6}
          mt={2}
          onClick={onLogout}
        >
          {signOut.label}
        </Button>
      }
    </Flex>
  )
}