import { EmrHeaderWithBody, EmrHeaderWithStatus } from "@/declarations/patient_registry/patient_registry.did";
import { create } from "zustand";

interface EMRStore {
  emrs: Array<EmrHeaderWithStatus>;
  copyEmrs: Array<EmrHeaderWithStatus>;
  emr: EmrHeaderWithBody | null;
  search: string;
}

interface EMRAction {
  setEMRS: (emrs: Array<EmrHeaderWithStatus>) => void;
  setEMR: (emr: EmrHeaderWithBody | null) => void;
  setSearchByHospitalName: (filter: string) => void;
}

export const useEMRStore = create<
  EMRStore &
  EMRAction
>((set) => ({
  emrs: [],
  copyEmrs: [],
  emr: null,
  search: "",
  setEMRS: (emrs) => set({
    emrs: emrs,
    copyEmrs: emrs
  }),
  setEMR: (emr) => set({ emr }),
  setSearchByHospitalName: (filter) => set((state) => {
    const emr = state.copyEmrs.filter((emr) => {
      return emr.hospital_name
        .toLowerCase()
        .includes(filter.toLowerCase());
    });
    return {
      emrs: emr,
      search: filter
    }
  })
}));