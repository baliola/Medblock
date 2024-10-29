import { EmrHeaderWithBody, EmrHeaderWithStatus } from "@/declarations/patient_registry/patient_registry.did";
import { create } from "zustand";

interface EMRStore {
  emr: EmrHeaderWithBody | null;
}

interface EMRAction {
  setEMR: (emr: EmrHeaderWithBody | null) => void;
}

export const useEMRDetail = create<
  EMRStore &
  EMRAction
>((set) => ({
  emr: null,
  setEMR: (emr) => set({ emr }),
}));