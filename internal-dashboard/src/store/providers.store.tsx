import { Provider } from "@/canister/declarations/provider_registry/provider_registry.did";
import { create } from "zustand";

interface ProviderStore {
  providers: Array<Provider> | undefined;
  provider: Provider | null | undefined;
}

interface ProviderAction {
  setProviders: (providers: Array<Provider> | undefined) => void;
  setProvider: (provider: Provider | null | undefined) => void;
}

export const useProviderStore = create<ProviderStore & ProviderAction>(
  (set) => ({
    providers: undefined,
    provider: undefined,
    setProviders: (providers) =>
      set({
        providers: providers,
      }),
    setProvider: (provider) => set({ provider }),
  })
);
