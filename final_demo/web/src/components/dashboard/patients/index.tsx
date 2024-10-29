"use client"
import { Flex, Text } from "@chakra-ui/react";

import { PatientActor, usePatientQuery } from "@/services/patients";
import { usePatientStore } from "@/store/patient-management";
import { PatientListResponse } from "@/declarations/patient_registry/patient_registry.did";

import { PatientsLoading } from "./loading";
import TableOverview from "./table";
import { patientCanisterId } from "@/config/canisters/patient.canister";
import { useEffect } from "react";

const PatientList = () => {
  const setPatient = usePatientStore(state => state.setPatient)
  const patientData = usePatientStore(state => state.patients);

  const {
    loading,
    error,
    call: fetchPatientList
  } = usePatientQuery({
    functionName: "patient_list",
    refetchOnMount: false,
    onSuccess(data) {
      if (!data) return;

      const { patients } = data as PatientListResponse;

      if (patientData.length < 1) {
        const transformedPatients = patients.map(patient => ({
          session_id: patient.session_id,
          name: patient.info.V1.name
        }));

        setPatient(transformedPatients)
      }
    },
    onError(error) {
      console.log(error)
    },
  });

  useEffect(() => {
    fetchPatientList()

    // eslint-disable-next-line
  }, []);

  if (loading) {
    return <PatientsLoading />
  }

  if (error) {
    return (
      <Flex w={'full'} justify={'center'} align={'center'}>
        <Text>
          No Patient List Here, please kindly add patient!
        </Text>
      </Flex>
    )
  }

  return (
    <Flex w={'full'} direction={'column'} gap={8}>
      <TableOverview />
      <Text fontSize={'sm'}>
        Total Patient: <b>{patientData.length}</b>
      </Text>
    </Flex>
  )
}

export default function Patients() {
  return (
    <PatientActor
      canisterId={patientCanisterId}
    >
      <PatientList />
    </PatientActor>
  )
}