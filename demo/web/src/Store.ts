import { AuthClient } from '@dfinity/auth-client';
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

  userPrincipal: string | undefined;
  setUserPrincipal: (id: string | undefined) => void;

  client: AuthClient | null;
  setClient: (client: AuthClient) => void;

  isSidebarOpen: boolean;
  toggleSidebar: () => void;
  setIsSidebarOpen: (isOpen: boolean) => void;
}

export const useCentralStore = create<centralStore>((set, get) => ({
  activePage: 'DASHBOARD',
  client: null,
  userPrincipal: undefined,
  setUserPrincipal(id) {
    set({ userPrincipal: id });
  },
  setActivePage: (page) => set({ activePage: page }),
  setClient(client) {
    set({
      client: client,
    });
  },
  isSidebarOpen: false,
  toggleSidebar: () => set({ isSidebarOpen: !get().isSidebarOpen }),
  setIsSidebarOpen: (isOpen) => set({ isSidebarOpen: isOpen }),
}));
