import { EmrListConsentResponse } from "@/declarations/patient_registry/patient_registry.did";
import { create } from "zustand";

interface EMRStore {
  userHasEMR: boolean;
  emrs: EmrListConsentResponse | null;
  copyEmrs: EmrListConsentResponse | null;
  loading: boolean;
  search: string;
}

interface EMRStoreActions {
  setUserHasEMR: (hasEMR: boolean) => void;
  setEMRS: (emrs: EmrListConsentResponse) => void;
  setLoading: (loading: boolean) => void;
  searchEMR: (search: string) => void;
}

export const useEMRStore = create<EMRStore & EMRStoreActions>((set) => ({
  userHasEMR: false,
  emrs: null,
  copyEmrs: null,
  loading: false,
  search: '',

  setUserHasEMR: (hasEMR) => {
    set({ userHasEMR: hasEMR });
  },

  setEMRS: (emrs) => {
    set({
      emrs: emrs,
      copyEmrs: emrs,
    });
  },

  setLoading: (loading) => {
    set({ loading });
  },

  searchEMR: (search) => {
    set((state) => {
      if (!state.copyEmrs || !state.copyEmrs.emr) {
        return { search, emrs: null };
      }

      const filteredEmrs = state.copyEmrs.emr.filter((emr) =>
        emr.hospital_name.toLowerCase().includes(search.toLowerCase())
      );

      return {
        search,
        emrs: { ...state.copyEmrs, emr: filteredEmrs },
      };
    });
  },
}));