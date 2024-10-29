import { create } from "zustand"

export interface Overview {
  hospital: Hospital
  patients: number
  emr: number
  doctors: number
  patient_gender: PatientGender
  patient_age: PatientAge
  polyclinics: Polyclinic[]
}

interface Hospital {
  name: string
  id: number
  image: string
}

interface PatientGender {
  male: number
  female: number
}

interface PatientAge {
  "0-18": number
  "19-30": number
  "31-45": number
  "46-60": number
  "60+": number
}

interface Polyclinic {
  id: number
  name: string
  patients: number
  doctors: number
  emr: number
}

interface OverviewStore {
  overview: Overview | null
}

interface OverviewStoreActions {
  setOverview: (overview: Overview) => void
}

export const useOverviewStore = create<
  OverviewStore &
  OverviewStoreActions
> ((set) => ({
  overview: null,
  setOverview: (overview) => set({ overview })
}))