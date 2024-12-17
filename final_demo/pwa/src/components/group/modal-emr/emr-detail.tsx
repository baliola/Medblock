"use client"

import EMRVitalSigns from "@/components/emr/vital-sign";
import { patientCanisterId } from "@/config/canisters/patient.canister";
import { emrDetailHeader } from "@/constants/contents/emr/detail/header";
import { emrDetailReports } from "@/constants/contents/emr/detail/reports";
import { EmrHeaderWithBody, EmrHeaderWithStatus, ReadGroupMembersEmrInfoRequest } from "@/declarations/patient_registry/patient_registry.did";
import { PatientActor, usePatientQuery } from "@/services/patients";
import { convertBigIntToTime } from "@/utils/format-time";
import { 
  Button,
  Flex,
  Grid,
  Icon,
  Modal,
  ModalContent,
  ModalOverlay,
  Text,
  useDisclosure,
} from "@chakra-ui/react";
import { Principal } from "@dfinity/principal";
import { useState } from "react";
import { FaHospital } from "react-icons/fa6";

interface IEMRDetailProps {
  emr: EmrHeaderWithStatus
  nik: string
  group_id: string
}

function EMRDetail({ props }: { props: IEMRDetailProps }) {
  const {
    emr,
    nik,
    group_id
  } = props

  const [emrDetail, setEmrDetail] = useState<EmrHeaderWithBody | null | undefined>(undefined);
  const { isOpen, onOpen, onClose } = useDisclosure()
  
  const {
    call: readGroupMemberEmr,
    loading: loadingReadGroupMemberEmr,
  } = usePatientQuery({
    functionName: "read_group_members_emr_info",
    args: [{
      member_nik: nik,
      emr_id: emr.header.emr_id as string,
      provider_id: emr.header.provider_id as string,
      registry_id: Principal.fromText(emr.header.registry_id.toString() as string),
      group_id
    } as ReadGroupMembersEmrInfoRequest] as any,
    onSuccess(data) {
      const { Ok, Err }: any = data;
      if (Ok) {
        setEmrDetail(Ok.emr);
      } else if (Err) {
        setEmrDetail(null);
      }
    },
    onError(error) {
      setEmrDetail(null);
      console.error(error);
    },
  });

  if (emrDetail === undefined || loadingReadGroupMemberEmr) return <Text>Please wait ...</Text>
  if (emrDetail === null) return <Text>No Data</Text>

  const emrData = emrDetail.body.reduce((acc, item) => {
    acc[item.key] = item.value;
    return acc;
  }, {} as Record<string, string>);

  const vitalSigns = {
    blood_pressure: emrData["blood_pressure"] || "",
    heart_rate: emrData["heart_rate"] || "",
    respiration: emrData["respiration"] || "",
    temperature: emrData["temperature"] || "",
    oxygen_saturation: emrData["o2_saturation"] || "",
  };

  const SectionVisitSummary = ({ keys, title }: { keys: string; title: string }) => {
    const value = emrData[keys] || '';
    // if (!value) return null;
    return (
      <Flex direction="column" gap={2}>
        <Text color="neutral.500">{title}</Text>
        <Text color="neutral.700" whiteSpace={'pre-line'}>
          {value === '' || !value ? "-" : value}
        </Text>
      </Flex>
    );
  };
  
  return (
    <>
      <Flex>
        <Button 
          type="button"
          display={"flex"}
          w={"full"}
          height={"fit-content"}
          gap={5}
          justifyContent={"start"}
          _hover={{ textDecoration: 'underline' }}
          onClick={onOpen}
          py={3}
          px={0}
          background={"white"}
          textAlign={"left"}
        >
          <Icon as={FaHospital} boxSize={10} color={'primary.700'} />
          <Flex direction={"column"} gap={1.5}>
            <Text fontSize={'base'} color={'neutral.700'}>
              {convertBigIntToTime(emr.status.updated_at)}
            </Text>
            <Text fontSize={'xl'} fontWeight={'bold'} color={'neutral.700'} textTransform={"uppercase"}>
              {emr.hospital_name}
            </Text>
            {/* <Text fontSize={'sm'} color={'neutral.400'}>
              {emr.header.provider_id}
            </Text> */}
          </Flex>
        </Button>
      </Flex>
      <Modal
        isOpen={isOpen}
        onClose={onClose}
        size={{ base: 'full' }}
      >
        <ModalOverlay 
          onClick={onClose}
          background={"transparent"}
        />
        <ModalContent
          border={"none"}
          shadow={"none"}
          marginX={0}
          marginY={"auto"}
          w={"full"}
          h={"full"}
          display={"flex"}
          onClick={onClose}
          background={"white"}
          p={8}
          overflowY={"auto"}
        >
          <Flex direction="column" gap={6}>
            <Text fontSize="lg" fontWeight="bold" color="neutral.700" px={1}>
              Report Detail
            </Text>

            <Grid templateColumns="repeat(2, 1fr)" gap={6} px={1} mb={4}>
              {emrDetailHeader.report.map((section, index) => (
                <SectionVisitSummary key={index}
                  title={section.title}
                  keys={section.key}
                />
              ))}
            </Grid>

            <EMRVitalSigns vitalSign={vitalSigns} />

            <Text fontSize="lg" fontWeight="bold" color="neutral.700" px={1} mt={4}>
              {emrDetailReports.header.report.title}
            </Text>

            <Grid templateColumns="repeat(2, 1fr)" gap={6} px={1}>
              {emrDetailReports.history.map((section, index) => (
                <SectionVisitSummary key={index}
                  title={section.title}
                  keys={section.key}
                />
              ))}
            </Grid>
            
            <Grid templateColumns="repeat(2, 1fr)" gap={6} px={1}>
              {emrDetailReports.allergies.map((section, index) => (
                <SectionVisitSummary key={index}
                  title={section.title}
                  keys={section.key}
                />
              ))}
            </Grid>
            
            <Flex direction={"column"} px={1} gap={6}>
              {emrDetailReports.result.map((section, index) => (
                <SectionVisitSummary key={index}
                  title={section.title}
                  keys={section.key}
                />
              ))}
            </Flex>
          </Flex>
        </ModalContent>
      </Modal>
    </>
  )
}

export default function EMRMemberDetail({ props }: { props: IEMRDetailProps }) {
  return (
    <PatientActor
      canisterId={patientCanisterId}
    >
      <EMRDetail props={props} />
    </PatientActor>
  )
}