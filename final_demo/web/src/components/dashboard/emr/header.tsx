'use client'

import HorizontalProfile from "@/components/profile/horizontal";
import { patientCanisterId } from "@/config/canisters/patient.canister";
import { FinishSessionRequest, GetPatientInfoResponse } from "@/declarations/patient_registry/patient_registry.did";
import { PatientActor, usePatientQuery } from "@/services/patients";
import { Flex, Icon, Text } from "@chakra-ui/react";
import { useParams, usePathname, useRouter, useSearchParams } from "next/navigation";
import { useEffect } from "react";
import { MdEditDocument } from "react-icons/md";

const PatientProfile = () => {
  const param = useParams();
  const params = useSearchParams();
  const pathname = usePathname();
  const router = useRouter();

  const recordId = param.id || null;

  const {
    data,
    call: getProfile
  } = usePatientQuery({
    functionName: "get_patient_info_with_consent",
    refetchOnMount: false,
  });

  const fetchProfile = async () => {
    if (!recordId) return null;

    try {
      const request: FinishSessionRequest = {
        session_id: recordId as string
      };

      // @ts-expect-error
      const data: GetPatientInfoResponse = await getProfile([request]);

      const newParams = new URLSearchParams(params);
      newParams.set('user', data.nik);

      const newUrl = `${pathname}?${newParams.toString()}`;
      router.replace(newUrl);

    } catch (err) {
      console.log(err)
    }
  }

  useEffect(() => {
    fetchProfile()

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [recordId])

  if (data) return <HorizontalProfile profile={data} />

  return null
}

export default function EMRHeader({ title }: { title: string }) {
  return (
    <Flex
      w={'full'}
      justify={'space-between'}
      align={'center'}
    >
      <Flex w={"fit-content"} gap={3} align={'center'}>
        <Icon as={MdEditDocument} boxSize={5} color={"primary.500"} />
        <Text fontSize={{ base: 'md', xl: 'xl' }} fontWeight={'bold'} color={"#2b2b2b"}>
          {title}
        </Text>
      </Flex>

      <Flex w={"sm"}>
        <PatientActor canisterId={patientCanisterId}>
          <PatientProfile />
        </PatientActor>
      </Flex>
    </Flex>
  )
}