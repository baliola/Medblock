import { PatientWithNik } from "@/canister/declarations/patient_registry/patient_registry.did";
import { create } from "zustand";

interface PatientStore {
  patients: Array<PatientWithNik> | undefined;
  patient: PatientWithNik | null | undefined;
}

interface PatientAction {
  setPatients: (patients: Array<PatientWithNik> | undefined) => void;
  setPatient: (patient: PatientWithNik | null | undefined) => void;
}

export const usePatientStore = create<PatientStore & PatientAction>(
  (set) => ({
    patients: undefined,
    patient: undefined,
    setPatients: (patients) =>
      set({
        patients: patients,
      }),
    setPatient: (patient) => set({ patient }),
  })
);
