import { usePathname, useSearchParams, useRouter } from "next/navigation";
import { Button, Divider, Flex, Icon, Stack, Text } from "@chakra-ui/react";

import { FaCalendarAlt } from "react-icons/fa";
import { FaX } from "react-icons/fa6";
import { HiHome } from "react-icons/hi2";
import { IoDocumentAttach } from "react-icons/io5";

import HorizontalProfile from "@/components/profile/horizontal";
import { GetPatientInfoResponse } from "@/declarations/patient_registry/patient_registry.did";

interface InfoRowProps {
  icon: React.ElementType;
  title: string;
  value: string;
}

const parsingDate = (date: string) => {
  const splitDate = date.split('T')[0];
  const newDate = new Date(splitDate);
  return newDate.toLocaleDateString('id-ID', {
    day: 'numeric',
    month: 'long',
    year: 'numeric'
  });
}

const InfoRow = ({
  icon,
  title,
  value
}: InfoRowProps) => {
  return (
    <Flex align={'center'} gap={3}>
      <Icon as={icon} boxSize={{ base: 5 }} color={"neutral.700"} />
      <Flex direction={'column'} color={"neutral.700"} gap={1}>
        <Text
          fontSize={{ base: 'xs' }}
        >
          {title}
        </Text>
        <Text
          fontSize={{ base: 'sm' }}
          fontWeight={'bold'}
        >
          {value}
        </Text>
      </Flex>
    </Flex>
  );
}

export default function EMRPatientInfo({
  patient
}: {
  patient: GetPatientInfoResponse
}) {
  const params = useSearchParams();
  const pathname = usePathname();
  const router = useRouter();

  const patientId = params.get("id");

  const onClose = () => {
    const param = new URLSearchParams(params);
    param.delete('id');

    const newUrl = `${pathname}?${param.toString()}`;
    router.push(newUrl);
  }

  const onAccess = (id: string) => {
    router.push(`/dashboard/emr/${id}`);
  }

  return (
    <Flex
      w={'24vw'}
      bg={'primary.100'}
      transition={'all 0.3s'}
      direction={'column'}
      p={7}
      gap={8}
    >
      <Flex align={'center'} gap={5}>
        <Button size={'xs'} rounded={'full'} colorScheme="red" p={0}>
          <Icon as={FaX} onClick={onClose} />
        </Button>
        <Text
          fontSize={{ base: 'md', xl: 'lg' }}
          fontWeight={'bold'}
        >
          EMR Preview
        </Text>
      </Flex>

      <Flex direction={"column"} gap={5} flex={1}>
        <HorizontalProfile profile={patient} />
        <Flex direction={'column'} gap={3} ps={4}>
          <Text
            fontSize={{ base: 'sm' }}
            fontWeight={'bold'}
            color={"neutral.700"}
          >
            Personal Information
          </Text>

          <Stack divider={<Divider borderColor={'purple.200'} />} spacing={3}>
            <InfoRow
              icon={HiHome}
              title={'Home Address'}
              value={patient?.patient.V1.address}
            />

            <InfoRow
              icon={IoDocumentAttach}
              title={'Marital Status'}
              value={patient?.patient.V1.martial_status}
            />

            <InfoRow
              icon={FaCalendarAlt}
              title={'Birthdate & Place'}
              value={`
              ${patient?.patient.V1.place_of_birth}, 
              ${parsingDate(patient?.patient.V1.date_of_birth as string)}
            `}
            />
          </Stack>
        </Flex>
      </Flex>
      <Button
        colorScheme="primary"
        bg="primary.700"
        w="full"
        rounded="lg"
        fontSize={'xs'}
        onClick={() => onAccess(patientId as string)}
      >
        Access EMR
      </Button>
    </Flex>
  )
}