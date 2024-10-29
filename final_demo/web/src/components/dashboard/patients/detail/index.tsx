"use client"

import { useSearchParams } from "next/navigation";
import { useToast } from "@chakra-ui/react";

import { PatientActor, usePatientQuery } from "@/services/patients";
import { EMRLoading } from "@/components/dashboard/patients/loading";
import EMRPatientInfo from "@/components/dashboard/patients/detail/patient-info";
import { patientCanisterId } from "@/config/canisters/patient.canister";

const DisplayEMR = () => {
  const params = useSearchParams();
  const patientId = params.get('id');

  const toast = useToast();

  const { data, loading } = usePatientQuery({
    functionName: "get_patient_info_with_consent",
    args: [{ session_id: patientId }] as any,
    refetchOnMount: true,
    onSuccess(data) {
      console.log(data);
    },
    onError(error) {
      toast({
        title: "Get Patient Info Error",
        description: "It looks like an invalid session or already revoked!",
        status: "error",
      });

      return null;
    },
  });

  if (!patientId) return null;

  if (loading) return <EMRLoading />;
  if (data) return <EMRPatientInfo patient={data} />;

  return null;
};

export default function EMRPreview() {
  return (
    <PatientActor
      canisterId={patientCanisterId}
    >
      <DisplayEMR />
    </PatientActor>
  );
}