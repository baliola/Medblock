import { Provider } from '@/declarations/provider_registry/provider_registry.did';
import { create } from 'zustand';

interface ListConsent extends Provider {
  session_user: string;
  code: string;
}

interface ConsentStore {
  listUserConsent: Array<ListConsent>;
  copyListUserConsent: Array<ListConsent>;
  revokeConsent: string[];
  isOpenConfirmation: boolean;
  search: string;
}

interface ConsentAction {
  setListUserConsent: (list: Array<ListConsent>) => void;
  setRevokeConsent: (list: string[]) => void;
  onOpenConfirmation: () => void;
  onCloseConfirmation: () => void;
  setSearch: (search: string) => void;
}

export const useConsentStore = create<
  ConsentStore &
  ConsentAction
>((set) => ({
  listUserConsent: [],
  copyListUserConsent: [],
  revokeConsent: [],
  isOpenConfirmation: false,
  search: "",

  setListUserConsent: (list) => set({
    listUserConsent: list,
    copyListUserConsent: list
  }),
  setRevokeConsent: (list) => set({ revokeConsent: list }),
  onOpenConfirmation: () => set({ isOpenConfirmation: true }),
  onCloseConfirmation: () => set({ isOpenConfirmation: false }),
  setSearch: (filter) => set((state) => {
    const list = state.copyListUserConsent.filter((consent) => {
      return consent.V1.display_name
        .toLowerCase()
        .includes(filter.toLowerCase());
    });

    return {
      listUserConsent: list,
      search: filter
    }
  }),
}));
