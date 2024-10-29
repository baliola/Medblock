"use client"

import { useRouter } from "next/navigation";
import { Divider, Flex, Icon, Stack, Text } from "@chakra-ui/react";
import { FaHospital } from "react-icons/fa6";

import { useEMRStore } from "@/store/emr-store";
import { EmrHeader } from "@/declarations/patient_registry/patient_registry.did";
import { convertBigIntToTime } from "@/utils/format-time";

export default function HomeHistoryVisits() {
  const router = useRouter();

  const emrs = useEMRStore(state => state.emrs);

  const onRedirect = (detail: EmrHeader) => {
    const { emr_id, provider_id, registry_id } = detail;
    router.push(`/emr/${emr_id}/${provider_id}/${registry_id.toString()}`);
  }

  return (
    <Stack flex={1} divider={<Divider />} spacing={3}>
      {emrs.map((emr, index) => (
        <Flex key={index}
          align={'start'}
          gap={5}
          _hover={{ textDecoration: 'underline' }}
          onClick={() => onRedirect(emr.header)}
        >
          <Icon as={FaHospital} boxSize={7} color={'primary.700'} />
          <Flex direction={"column"} gap={1}>
            <Text fontSize={'xs'} color={'neutral.700'}>
              {convertBigIntToTime(emr.status.updated_at)}
            </Text>
            <Text fontSize={'lg'} fontWeight={'bold'} color={'neutral.700'}>
              {emr.hospital_name}
            </Text>
            {/* <Text fontSize={'sm'} color={'neutral.400'}>
              {emr.header.provider_id}
            </Text> */}
          </Flex>
        </Flex>
      ))}
    </Stack>
  )
}