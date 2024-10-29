"use client"

import { usePathname, useSearchParams, useRouter } from "next/navigation";
import { Button, Flex, Icon, Td, Text, Tr, useToast } from "@chakra-ui/react";
import { VscSettings } from "react-icons/vsc";

import { FinishSessionRequest } from "@/declarations/patient_registry/patient_registry.did";
import { usePatientUpdate } from "@/services/patients";
import { usePatientStore } from "@/store/patient-management";

import ITable, { TD } from "@/components/table";
import ITableBody from "@/components/table/body";
import ITableHeader from "@/components/table/header";

const ITableAction = ({ id }: { id: string }) => {
  const toast = useToast();

  const params = useSearchParams();
  const pathname = usePathname();
  const router = useRouter();

  const deletePatient = usePatientStore(state => state.deletePatient);

  const {
    call: revokePatient,
    loading
  } = usePatientUpdate({
    functionName: "finish_session",
    onSuccess() {
      deletePatient({ session_id: id });

      const param = new URLSearchParams(params);
      param.delete("id");

      const newUrl = `${pathname}?${param.toString()}`;
      router.push(newUrl);

      return;
    },
    onError(error) {
      console.log(error)

      toast({
        title: "Error while revoke access",
        description: "An error occured while revoke patient access, try again!",
        status: "error",
      })

      return;
    },
  });

  const onRevoke = async () => {
    const data: FinishSessionRequest = {
      session_id: id
    }
    // @ts-ignore
    await revokePatient([data])
  }

  return (
    <Flex align={'center'} gap={5}>
      <Button
        colorScheme="primary"
        bg="primary.700"
        rounded={"lg"}
        size={'sm'}
        fontSize={'xs'}
        leftIcon={
          <Icon as={VscSettings} boxSize={4} />
        }
        onClick={onRevoke}
        isLoading={loading}
      >
        Revoke
      </Button>
    </Flex>
  )
}

export default function TablePatients() {
  const router = useRouter();
  const pathname = usePathname();
  const searchParams = useSearchParams();

  const patients = usePatientStore(state => state.patients);

  const onClickPatient = async (id: string) => {
    const params = new URLSearchParams(searchParams);
    params.set('id', id.toString());
    params.delete('concent_input');

    const newUrl = `${pathname}?${params.toString()}`;
    router.push(newUrl);
  }

  if (patients.length > 0) {
    return (
      <ITable>
        <ITableHeader headers={['No', 'Name', 'Session ID', 'Actions']} />
        <ITableBody>
          {patients?.map((patient, index: number) => (
            <Tr key={index}
              _hover={{ bg: 'primary.100' }}
            >
              <TD roundedLeft={'xl'}
                onClick={() => onClickPatient(patient.session_id)}
              >
                {index + 1}
              </TD>
              <TD onClick={() => onClickPatient(patient.session_id)}>
                {patient.name}
              </TD>
              <TD
                _hover={{
                  textDecoration: "underline",
                  cursor: "pointer"
                }}
                onClick={() => onClickPatient(patient.session_id)}
              >
                {patient.session_id}
              </TD>
              <TD roundedRight={'xl'}>
                <ITableAction id={patient.session_id} />
              </TD>
            </Tr>
          ))}
        </ITableBody>
      </ITable>
    )
  }

  return (
    <Flex w={'full'} justify={'center'} align={'center'}>
      <Text fontSize={'sm'}>
        No Patient List Here, please kindly add patient!
      </Text>
    </Flex>
  )
}