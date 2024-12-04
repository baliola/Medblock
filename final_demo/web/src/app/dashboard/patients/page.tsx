import { Flex } from "@chakra-ui/react";

import { PatientListHeader } from "@/components/dashboard/patients/header";
import Patients from "@/components/dashboard/patients";
import EMRPreview from "@/components/dashboard/patients/detail";
import ConcentCode from "@/components/concent-code";

interface PageProps {
  searchParams: {
    currentPage?: number;
    limit?: number;
    id?: string;
    concent_input?: string;
  }
}

export default async function PatientsPage({ searchParams }: PageProps) {
  const patientId = searchParams.id || null;
  const concent_input = searchParams.concent_input === "true";

  return (
    <Flex w={'full'} flex={1}>
      <Flex w={'full'} direction={'column'} p={10} gap={8} flex={1} overflowX={'auto'}>
        <PatientListHeader />
        <Patients />
      </Flex>
      {patientId && <EMRPreview />}
      {concent_input && <ConcentCode />}
    </Flex>
  )
}