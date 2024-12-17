"use client"

import { patientCanisterId } from "@/config/canisters/patient.canister";
import { EmrHeaderWithBody, ReadEmrByIdResponse, ReadGroupMembersEmrInfoRequest } from "@/declarations/patient_registry/patient_registry.did";
import { PatientActor, usePatientQuery } from "@/services/patients";
import { 
  Flex,
  Text,
} from "@chakra-ui/react";
import { Principal } from "@dfinity/principal";
import { useEffect, useState } from "react";

interface IEMRDetailProps {
  provider_id: string
  emr_id: string
  registry_id: Principal
  nik: string
  group_id: string
}

function EMRDetail({ props }: { props: IEMRDetailProps }) {
  const { 
    provider_id,
    emr_id, 
    registry_id,
    nik,
    group_id
  } = props

  const [emrDetail, setEmrDetail] = useState<EmrHeaderWithBody | null | undefined>(undefined);
  
  const {
    call: readGroupMemberEmr,
    loading: loadingReadGroupMemberEmr,
  } = usePatientQuery({
    functionName: "read_group_members_emr_info",
    args: [{
      member_nik: nik,
      emr_id: emr_id as string,
      provider_id: provider_id as string,
      registry_id: Principal.fromText(registry_id.toString() as string),
      group_id
    } as ReadGroupMembersEmrInfoRequest] as any,
    onSuccess(data) {
      const { Ok }: any = data;
      if (Ok) setEmrDetail(Ok.emr);
      else setEmrDetail(null);
    },
    onError(error) {
      setEmrDetail(null);
      console.error(error);
    },
  });

  if (emrDetail === undefined) return <Text>Please wait ...</Text>
  if (emrDetail === null) return <Text>No Data</Text>
  
  return (
    <Flex direction={"column"}>
      {
        emrDetail.body.map((item, index) =>
          <Flex key={index}>
            <Text>{item.key}</Text>: <Text>{item.value ?? '-'}</Text>
          </Flex>
        )
      }
    </Flex>
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