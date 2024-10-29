import { Flex, TabPanel, TabPanels, Text } from "@chakra-ui/react";
import HeaderEMRReport from "@/components/dashboard/emr/report/header";
import EMROverview from "./overview";
import { useEffect } from "react";
import { EMRReportLoading } from "../loading";
import { useSearchParams } from "next/navigation";
import { usePatientQuery } from "@/services/patients";
import { Principal } from "@dfinity/principal";
import { ReadEmrSessionRequest } from "@/declarations/patient_registry/patient_registry.did";
import { useEMRDetail } from "@/store/patient-emr-detail";
import EMRPatientEmpty from "../empty";

interface Params {
  record: string | null;
  provider: string | null;
  registry: string | null;
}

const isParamsFullfilled = ({ record, registry, provider }: Params) => {
  return !!record && !!provider && !!registry;
};

export default function EMRReport({ id }: { id: string }) {
  const params = useSearchParams();
  const setEMR = useEMRDetail(state => state.setEMR);

  const recordId = params.get('record') || null;
  const providerId = params.get('provider') || null;
  const registry = params.get('registry') || null;

  const {
    call: getDetailEMR,
    loading,
    error
  } = usePatientQuery({
    functionName: "read_emr_with_session",
    refetchOnMount: false,
  });

  const fetchEmrDetails = async () => {
    if (recordId && providerId && registry) {
      const request: ReadEmrSessionRequest = {
        session_id: id,
        args: {
          emr_id: recordId as string,
          provider_id: providerId as string,
          registry_id: Principal.fromText(registry as string)
        }
      }

      try {
        // @ts-expect-error
        const data: ReadEmrByIdResponse = await getDetailEMR([request]);
        setEMR(data.emr);
      } catch (error) {
        console.error("Failed to fetch EMR details", error);
      }
    }
  }

  useEffect(() => {
    fetchEmrDetails();

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [recordId, providerId, registry])

  if (!isParamsFullfilled({
    record: recordId,
    provider: providerId,
    registry: registry
  })) {
    return <EMRPatientEmpty />
  }

  if (loading) return <EMRReportLoading />

  if (error) {
    return (
      <Flex
        direction="column"
        w="full"
        p={8}
        bg="primary.100"
        rounded="2xl"
        gap={10}
        h={"full"}
        align={"center"}
        justify={"center"}
      >
        <Text fontSize={'lg'} fontWeight={'bold'} color={"red.500"}>
          {error.message}
        </Text>
      </Flex>
    )
  }

  return (
    <Flex
      direction="column"
      w="full"
      p={8}
      bg="primary.100"
      rounded="xl"
      gap={10}
      h={"fit-content"}
    >
      <HeaderEMRReport />

      <Flex direction={"column"} gap={5} px={5}>
        <Text fontSize={{ base: 'md', lg: "lg" }} fontWeight={'bold'} color={'neutral.700'}>
          Visit Summary
        </Text>

        <TabPanels>
          <TabPanel p={0}>
            <EMROverview />
          </TabPanel>
          <TabPanel p={0}>Notes</TabPanel>
          <TabPanel p={0}>Document</TabPanel>
          <TabPanel p={0}>Labs</TabPanel>
          <TabPanel p={0}>Imaging</TabPanel>
          <TabPanel p={0}>Communication</TabPanel>
        </TabPanels>
      </Flex>
    </Flex>
  )
}