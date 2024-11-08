import { create } from "zustand"

type Status = "active" | "suspended" | "idle";

export interface HospitalStatus {
  status: Status;
}

export interface HospitalStatusStoreActions {
  setStatus: (status: HospitalStatus) => void;
}

export const useHospitalStatusStore = create<
  HospitalStatus &
  HospitalStatusStoreActions
  >((set) => ({
    status: "idle",
    setStatus: (status) => set({ 
      status: status.status
    })
}))