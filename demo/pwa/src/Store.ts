import { HttpAgent, Identity } from '@dfinity/agent';
import { AuthClient } from '@dfinity/auth-client';
import {
  Patient,
  PatientWithNikAndSession,
} from 'declarations/patient_registry/patient_registry.did';
import { create } from 'zustand';

type pageOptions =
  | 'DASHBOARD'
  | 'INTEGRATIONS'
  | 'SETTINGS'
  | 'CALENDAR'
  | 'TIMEOFF'
  | 'PROJECTS'
  | 'TEAMS'
  | 'BENEFITS'
  | 'DOCUMENTS'
  | 'SUPPORT';

interface CentralStore {
  activePage: pageOptions;
  setActivePage: (page: pageOptions) => void;

  patientList: PatientWithNikAndSession[] | null;
  setPatientList: (patient: PatientWithNikAndSession[]) => void;

  // Principal State
  userPrincipal: string | undefined;
  setUserPrincipal: (id: string | undefined) => void;

  // Client State
  client: AuthClient | null;
  setClient: (client: AuthClient) => void;

  // Agent State
  agent: HttpAgent | undefined;
  setAgent: (userAgent: HttpAgent) => void;

  // Identity State
  identity: Identity | undefined;
  setIdentity: (activeIdentity: Identity) => void;

  // Sidebar State
  isSidebarOpen: boolean;
  toggleSidebar: () => void;
  setIsSidebarOpen: (isOpen: boolean) => void;
}

export const useCentralStore = create<CentralStore>((set, get) => ({
  patientList: null,
  setPatientList(patient) {
    set({ patientList: patient });
  },

  client: null,
  setClient(client) {
    set({
      client: client,
    });
  },

  identity: undefined,
  setIdentity(activeIdentity) {
    set({ identity: activeIdentity });
  },

  agent: undefined,
  setAgent(userAgent) {
    set({ agent: userAgent });
  },

  userPrincipal: undefined,
  setUserPrincipal(id) {
    set({ userPrincipal: id });
  },

  activePage: 'DASHBOARD',
  setActivePage: (page) => set({ activePage: page }),

  isSidebarOpen: false,
  toggleSidebar: () => set({ isSidebarOpen: !get().isSidebarOpen }),
  setIsSidebarOpen: (isOpen) => set({ isSidebarOpen: isOpen }),
}));
