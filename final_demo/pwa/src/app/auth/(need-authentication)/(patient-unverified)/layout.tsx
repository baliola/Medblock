"use client"

import { patientCanisterId } from "@/config/canisters/patient.canister";
import LoadingScreen from "@/layouts/loading";
import { PatientActor } from "@/services/patients";

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <PatientActor
      canisterId={patientCanisterId}
      loadingComponent={<LoadingScreen />}
    >
      {children}
    </PatientActor>
  )
}