"use client"

import { useRouter, useSearchParams } from "next/navigation";
import { Button, ButtonProps, Flex, Icon, Stack, Tab, TabList, useToast } from "@chakra-ui/react";
import { IoAddCircle } from "react-icons/io5";
import { IoMdDocument } from "react-icons/io";

import { EMRProfileLoading } from "./loading";
import { usePatientQuery } from "@/services/patients";
import { FinishSessionRequest, GetPatientInfoResponse } from "@/declarations/patient_registry/patient_registry.did";
import VerticalProfile from "@/components/profile/vertical";
import { Fragment, useEffect } from "react";

const tabMenus = [
  { label: "Overview" },
  { label: "Notes" },
  { label: "Document" },
  { label: "Labs" },
  { label: "Imaging" },
  { label: "Communication" },
]

const CustomButton = ({ children, ...props }: ButtonProps) => {
  return (
    <Button
      size="md"
      fontSize="sm"
      colorScheme="primary"
      bg="primary.600"
      rounded="lg"
      fontWeight="medium"
      py={6}
      justifyContent="start"
      {...props}
    >
      {children}
    </Button>
  )
}

const PatientInfo = ({ id, patient }: { id: string; patient: GetPatientInfoResponse }) => {
  const router = useRouter();
  const params = useSearchParams();

  const record = params.get("record") || null;
  const provider = params.get("provider") || null;
  const registry = params.get("registry") || null;

  const onUpdate = () => router.push(`/dashboard/emr/${id}/update/${record}?provider=${provider}&registry=${registry}`);
  const onCreate = () => router.push(`/dashboard/emr/${id}/create`);

  return (
    <Flex direction="column" gap={4} pos={"sticky"} top={0} w={{ base: 'full', lg: "18vw" }}>
      <VerticalProfile profile={patient} />
      <CustomButton
        leftIcon={
          <Icon as={IoAddCircle} boxSize={4} />
        }
        onClick={onCreate}
      >
        Create EMR
      </CustomButton>
      {record && (
        <Fragment>
          <CustomButton
            leftIcon={
              <Icon as={IoMdDocument} boxSize={4} />
            }
            onClick={onUpdate}
            variant={'outline'}
            bg="none"
            _hover={{ bg: 'primary.600', color: 'white' }}
          >
            Update EMR
          </CustomButton>

        </Fragment>
      )}

      {record && (
        <TabList mt={4}>
          <Stack bg="primary.100" spacing={0} w="full" rounded="2xl">
            {tabMenus.map((menu, index) => (
              <Button key={index}
                as={Tab}
                roundedTop={index === 0 ? "xl" : "none"}
                roundedBottom={index === tabMenus.length - 1 ? "xl" : "none"}
                rounded="none"
                p={6}
                fontSize={'sm'}
                _selected={{ bg: 'primary.300' }}
              >
                {menu.label}
              </Button>
            ))}
          </Stack>
        </TabList>
      )}
    </Flex>
  )
}

export default function EMRProfile({ id }: { id: string }) {
  const toast = useToast();
  const router = useRouter();

  const {
    data,
    loading,
    call: getPatientProfile
  } = usePatientQuery({
    functionName: "get_patient_info_with_consent",
    refetchOnMount: false,
  });

  const fetchPatientProfile = async () => {
    try {
      const request: FinishSessionRequest = {
        session_id: id
      };
      // @ts-ignore
      await getPatientProfile([request]);
    } catch (error) {
      toast.closeAll();
      toast({
        title: "Cannot Access EMR",
        description: "Patient not found or session is wrong, try again!",
        status: "error",
      })

      router.replace("/dashboard/patients");

      return;
    }
  }

  useEffect(() => {
    fetchPatientProfile()

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [id]);

  if (loading) return <EMRProfileLoading />
  if (data) return <PatientInfo id={id} patient={data} />
}