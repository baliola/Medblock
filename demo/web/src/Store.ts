import { HttpAgent, Identity } from '@dfinity/agent';
import { AuthClient } from '@dfinity/auth-client';
import { Patient } from 'declarations/patient_registry/patient_registry.did';
import { Provider } from 'declarations/provider_registry/provider_registry.did';
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

interface centralStore {
  activePage: pageOptions;
  setActivePage: (page: pageOptions) => void;

  sessionId: string | undefined;
  setSessionId: (session: string) => void;

  provider: Provider | undefined;
  setProvider: (provider: Provider) => void;

  patientList: Patient[] | null;
  setPatientList: (patient: Patient[]) => void;

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

export const useCentralStore = create<centralStore>((set, get) => ({
  patientList: null,
  setPatientList(patient) {
    set({ patientList: patient });
  },

  provider: undefined,
  setProvider(provider) {
    set({ provider: provider });
  },

  sessionId: undefined,
  setSessionId(session) {
    set({ sessionId: session });
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
