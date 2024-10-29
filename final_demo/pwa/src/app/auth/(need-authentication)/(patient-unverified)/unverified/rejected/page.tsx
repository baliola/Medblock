"use client"

import { redirect, useRouter } from "next/navigation";
import { Button, Flex, Image, Stack, Text } from "@chakra-ui/react";
import { useQuery } from "@tanstack/react-query";
import LoadingScreen from "@/layouts/loading";
import { getKYCStatus } from "@/libs/api/kyc";
import { useProfileStore } from "@/store/profile-store";
import { kycRejected } from "@/constants/contents/auth/registration/reject";

export default function RejectedPage() {
  redirect("/home");

  const router = useRouter();
  const patientData = useProfileStore((state) => state.profile);

  const { title, image, button } = kycRejected;

  const { data: status, isLoading } = useQuery({
    queryKey: ['registration-status'],
    queryFn: () => getKYCStatus(patientData?.nik as string),
    enabled: !!patientData,
  });

  if (isLoading) {
    return <LoadingScreen />
  }

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
        <Stack spacing={3} align={'center'} textAlign={'center'}>
          <Text fontSize={'xl'} fontWeight={'bold'}>
            {title}
          </Text>
          <Text fontSize={'md'}>
            {status?.verificationHistory[0].message}
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