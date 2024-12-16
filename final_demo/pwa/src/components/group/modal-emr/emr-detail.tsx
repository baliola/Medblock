"use client"

import { patientCanisterId } from "@/config/canisters/patient.canister";
import { EmrHeaderWithBody, ReadEmrByIdRequest } from "@/declarations/patient_registry/patient_registry.did";
import { PatientActor, usePatientQuery } from "@/services/patients";
import { 
  Flex,
  Text,
  useDisclosure,
} from "@chakra-ui/react";
import { Principal } from "@dfinity/principal";
import { useEffect, useState } from "react";

interface IEMRDetailProps {
  provider_id: string
  emr_id: string
  registry_id: Principal
}

function EMRDetail({ props }: { props: IEMRDetailProps }) {
  const { provider_id, emr_id, registry_id } = props
  const { isOpen, onOpen, onClose } = useDisclosure();

  const [emrDetail, setEmrDetail] = useState<EmrHeaderWithBody | null | undefined>(undefined);
  
  const {
    call: readEmrById,
    loading: loadingReadEmrById,
  } = usePatientQuery({
    functionName: "read_emr_by_id",
    onSuccess(data) {
      console.log(data);
      setEmrDetail(data);
    },
    onError(error) {
      setEmrDetail(null);
      console.error(error);
    },
  });

  useEffect(()=>{
    if (provider_id && emr_id && registry_id){
     const registry = registry_id.toText()
      const request: ReadEmrByIdRequest = {
        emr_id: emr_id as string,
        provider_id: provider_id as string,
        registry_id: Principal.fromText(registry as string),
      };

      // @ts-expect-error
      readEmrById([request])
    }
  }, [provider_id, emr_id, registry_id])

  if (emrDetail === undefined) return <Text>Please wait ...</Text>
  if (emrDetail === null) return <Text>No Data</Text>
  
  return (
    <Flex>
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