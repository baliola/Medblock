import { usePathname, useRouter, useSearchParams } from "next/navigation";
import { Flex, Icon, Text } from "@chakra-ui/react";
import { FaHospital } from "react-icons/fa";

import { EmrHeader, EmrListConsentResponse } from "@/declarations/patient_registry/patient_registry.did";
import { useEMRStore } from "@/store/patient-emr";
import { convertBigIntToTime } from "@/utils/format-time";

import { EMRHistoryLoading } from "@/components/dashboard/emr/loading";
import EMRFilter from "@/components/dashboard/emr/filter";
import EMRHistoryPagination from "@/components/dashboard/emr/pagination";

const EMRRecord = ({ record }: { record: EmrListConsentResponse['emr'][0] }) => {
  const router = useRouter();
  const params = useSearchParams();
  const pathname = usePathname();

  const onHrefRecord = (header: EmrHeader) => {
    if (!header) return;

    const param = new URLSearchParams(params);
    param.set('record', header.emr_id);
    param.set('provider', header.provider_id);
    param.set('registry', header.registry_id.toText());

    const href = `${pathname}?${param.toString()}`;
    router.push(href);
  }

  return (
    <Flex
      align="center"
      gap={4}
      cursor="pointer"
      _hover={{ textDecoration: "underline" }}
      onClick={() => onHrefRecord(record.header)}
    >
      <Icon as={FaHospital} boxSize={5} color="primary.700" />
      <Flex direction="column" w="full">
        <Text fontSize={{ base: "xs" }} color="neutral.600" fontWeight="medium">
          {convertBigIntToTime(record.status.updated_at)}
        </Text>
        <Text fontWeight="bold" fontSize={{ base: 'md' }} color="neutral.700">
          {record.hospital_name}
        </Text>
      </Flex>
    </Flex>
  )
}

const EMRNotFound = () => (
  <Text color="gray.500" textAlign="center" py={5} fontSize={'sm'}>
    No EMR Record Found
  </Text>
);

export default function EMRHistory() {
  const loading = useEMRStore(state => state.loading);
  const emrs = useEMRStore(state => state.emrs);

  if (loading) return <EMRHistoryLoading />

  return (
    <Flex
      w={{ lg: "35vw" }}
      direction="column"
      px={5} pb={5}
      bg="primary.100"
      rounded="xl"
      maxH={"80dvh"}
      overflowY={'auto'}
      pos={"sticky"}
      top={0}
    >
      <Flex pos={'sticky'} top={0} bg={"primary.100"} py={5} direction={"column"} gap={3}>
        <Text fontSize={{ base: 'md', lg: "lg" }} fontWeight={'bold'}>
          History Record
        </Text>
        <EMRFilter />
      </Flex>

      <Flex flex={1} direction={"column"} gap={3} pt={2} maxH={'full'} overflowY={'auto'}>
        {emrs && emrs.emr.length === 0
          ? <EMRNotFound />
          : emrs?.emr.map((record, index) => (
            <EMRRecord key={index} record={record} />
          ))
        }
      </Flex>

      <EMRHistoryPagination />
    </Flex>
  )
}