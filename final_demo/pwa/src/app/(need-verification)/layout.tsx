"use client"

import { patientCanisterId } from "@/config/canisters/patient.canister";
import { GetPatientInfoResponse } from "@/declarations/patient_registry/patient_registry.did";
import LoadingScreen from "@/layouts/loading";
import { PatientActor, usePatientQuery } from "@/services/patients";
import { useProfileStore } from "@/store/profile-store";
import { useAuthState } from "@ic-reactor/react"
import { useRouter } from "next/navigation";

const RegistrationStatus = ({ children }: { children: React.ReactNode }) => {
  const router = useRouter();
  const { profile, setProfile } = useProfileStore()

  const { loading: loadingGetPatientData } = usePatientQuery({
    functionName: "get_patient_info",
    refetchOnMount: true,
    onSuccess(data) {
      console.log('PATIENT INFO', data)
      if (data) setProfile(data)
    },
    onError(error) {
      setProfile(null)
    },
  });


  if (loadingGetPatientData || profile === undefined) {
    return <LoadingScreen />
  }

  if (profile === null) {
    router.replace("/auth/unverified")
    return;
  }

  if (Object.keys(profile.patient.V1.kyc_status)[0].toLowerCase() === "pending") {
    router.replace("/auth/unverified/waiting");
    return;
  }

  if (Object.keys(profile.patient.V1.kyc_status)[0].toLowerCase() === "denied") {
    router.replace("/auth/unverified/rejected");
    return;
  }

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