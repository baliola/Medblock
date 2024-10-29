"use client"

import { PatientActor } from "@/services/patients";
import { patientCanisterId } from "@/config/canisters/patient.canister";
import EMRRevokeList from "@/components/emr/revoke/list";

export default function EMRRevoke() {
  return (
    <PatientActor canisterId={patientCanisterId}>
      <EMRRevokeList />
    </PatientActor>
  )
}