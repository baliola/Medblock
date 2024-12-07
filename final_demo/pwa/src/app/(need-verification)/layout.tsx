"use client"

import { patientCanisterId } from "@/config/canisters/patient.canister";
import LoadingScreen from "@/layouts/loading";
import { getKYCStatus } from "@/libs/api/kyc";
import { PatientActor, usePatientQuery } from "@/services/patients";
import { useProfileStore } from "@/store/profile-store";
import { useAuthState } from "@ic-reactor/react"
import { useQuery } from "@tanstack/react-query";
import { useRouter } from "next/navigation";

const RegistrationStatus = ({ children }: { children: React.ReactNode }) => {
  const router = useRouter();
  const { setProfile } = useProfileStore()

  const { data: patientData, loading } = usePatientQuery({
    functionName: "get_patient_info",
    refetchOnMount: true,
    onSuccess(data) {
      if (data) setProfile(data)
    },
    onError(error) {
      // router.replace("/auth/unverified")
    }
  });

  // const { data: status, isLoading } = useQuery({
  //   queryKey: ['registration-status'],
  //   refetchOnWindowFocus: true,
  //   queryFn: async () => {
  //     if (patientData) {
  //       // @ts-ignore
  //       const response = await getKYCStatus(patientData?.nik);
  //       return response;
  //     }
  //   },
  //   enabled: !!patientData,
  // });

  // if (isLoading || loading) {
  //   return <LoadingScreen />
  // }

  // if (status?.verification === "pending") {
  //   router.replace("/auth/unverified/waiting");
  //   return;
  // }

  // if (status?.verification === "rejected") {
  //   router.replace("/auth/unverified/rejected");
  //   return;
  // }

  return children;
}

export default function ProtectedLayout({
  children
}: {
  children: React.ReactNode
}) {
  const { authenticated, authenticating } = useAuthState();
  const router = useRouter();

  if (
    !authenticating &&
    authenticated
  ) {
    return (
      <PatientActor canisterId={patientCanisterId}>
        <RegistrationStatus>
          {children}
        </RegistrationStatus>
      </PatientActor>
    )
  }

  if (
    !authenticating &&
    !authenticated
  ) {
    router.replace("/auth/login");
    return;
  }

  return <LoadingScreen />
}