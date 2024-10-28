"use client"

import { Avatar, Button, Flex, Stack, Text } from "@chakra-ui/react";
import { AuthHeader } from "@/components/auth/header";
import { useRouter } from "next/navigation";
import { usePatientQuery } from "@/services/patients";
import { GetPatientInfoResponse } from "@/declarations/patient_registry/patient_registry.did";
import LoadingScreen from "@/layouts/loading";
import { useState } from "react";

const PatientInfo = ({ data }: { data: GetPatientInfoResponse | undefined }) => {
  return (
    <Flex direction={"column"} gap={4} w={'full'} align={'center'}>
      <Text fontWeight={'bold'} fontSize={'lg'}>
        Welcome
      </Text>
      <Avatar size={'xl'} rounded={"xl"} />
      <Text fontWeight={"bold"} fontSize={"xl"}>
        {data?.patient.V1.name || "Guest"}
      </Text>
    </Flex>
  )
}

export default function UnverifiedPage() {
  const router = useRouter();
  const onRegistration = () => router.push("/auth/unverified/registration");
  const onHome = () => router.replace("/home");

  const [patientData, setPatientData] = useState<GetPatientInfoResponse>();

  const {
    loading: patientLoading,
  } = usePatientQuery({
    functionName: "get_patient_info",
    refetchOnMount: true,
    onSuccess(data) {
      setPatientData(data);
    },
  });


  if (patientLoading) {
    return <LoadingScreen />
  }

  return (
    <Flex
      w={'full'}
      direction={"column"}
      justify={'space-between'}
      align={'center'}
      h={'full'}
      pt={8}
    >
      <Stack spacing={10}>
        <AuthHeader size="sm" />
        <PatientInfo data={patientData} />
      </Stack>

      {patientData ? (
        <Button
          colorScheme="primary"
          w={"full"}
          bg={"primary.700"}
          rounded={"2xl"}
          fontSize={'sm'}
          py={6}
          onClick={onHome}
        >
          Go to App
        </Button>
      ) : (
        <Button
          colorScheme="primary"
          w={"full"}
          bg={"primary.700"}
          rounded={"2xl"}
          fontSize={'sm'}
          py={6}
          onClick={onRegistration}
        >
          Verify your ID
        </Button>
      )}
    </Flex>
  )
}