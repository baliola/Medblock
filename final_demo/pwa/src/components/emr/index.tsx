"use client"

import { useEffect } from "react";
import { useParams } from "next/navigation";
import { Flex } from "@chakra-ui/react";

import { patientCanisterId } from "@/config/canisters/patient.canister";
import { ReadEmrByIdRequest } from "@/declarations/patient_registry/patient_registry.did";
import { PatientActor, usePatientQuery } from "@/services/patients";
import { Principal } from "@dfinity/principal";
import { useEMRStore } from "@/store/emr-store";

import EMRReport from "./report";
import EMRHeader from "./header";

const EMRData = () => {
  const setEMR = useEMRStore(state => state.setEMR);
  const params = useParams();

  const {
    emr_id,
    provider_id,
    registry_id
  } = params;

  const { call: getEmrById } = usePatientQuery({
    functionName: "read_emr_by_id",
    onSuccess(data) {
      // @ts-expect-error
      setEMR(data?.emr);
    },
    onError(error) {
      console.log(error)
    },
  });

  useEffect(() => {
    if (emr_id && provider_id && registry_id) {
      const request: ReadEmrByIdRequest = {
        emr_id: emr_id as string,
        provider_id: provider_id as string,
        registry_id: Principal.fromText(registry_id as string),
      };

      // @ts-expect-error
      getEmrById([request]);
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [emr_id, provider_id, registry_id]);

  return (
    <Flex w={"full"} direction={'column'} gap={5}>
      <EMRHeader />
      <EMRReport />
    </Flex>
  )
}

export default function EMRPatient() {
  return (
    <PatientActor
      canisterId={patientCanisterId}
    >
      <EMRData />
    </PatientActor>
  )
}