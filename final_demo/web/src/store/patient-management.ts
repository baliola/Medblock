import { create } from "zustand";

interface Patient {
  session_id: string;
  name: string;
}

interface PatientStore {
  patients: Patient[];
  copyPatients: Patient[];
  search: string;
}

interface PatientStoreActions {
  addPatient: (patient: Patient) => void;
  setPatient: (patients: Patient[]) => void;
  deletePatient: ({ session_id }: { session_id: string }) => void;
  searchPatient: (search: string) => void;
}

export const usePatientStore = create<PatientStore & PatientStoreActions>((set) => ({
  patients: [],
  copyPatients: [],
  search: '',

  setPatient: (patients) => set(() => ({
    patients: patients,
    copyPatients: patients
  })),

  addPatient: (patient) => set((state) => ({
    patients: [...state.patients, patient],
    copyPatients: [...state.copyPatients, patient]
  })),

  deletePatient: ({ session_id }) => set((state) => ({
    patients: state.patients.filter((patient) => patient.session_id !== session_id),
    copyPatients: state.copyPatients.filter((patient) => patient.session_id !== session_id)
  })),

  searchPatient: (search) => set((state) => ({
    search: search,
    patients: state.copyPatients.filter((patient) => patient.name
      .toLowerCase().includes(search.toLowerCase()))
  }))
}));
