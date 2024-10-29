import { LogResponse } from "@/declarations/patient_registry/patient_registry.did";
import { AgentError } from "@dfinity/agent/lib/cjs/errors";
import { create } from "zustand";

type Status = "authenticated" | "unauthenticated" | "loading";

interface NotificationStore {
  notification: LogResponse | null;
  error: AgentError | null;
  isError: boolean;
  loading: boolean;
}

interface NotificationStoreActions {
  setNotification: (notification: LogResponse  | null) => void;
  setError: (error: AgentError | null) => void;
  setIsError: (isError: boolean) => void;
  setLoading: (loading: boolean) => void;
}

export const useNotificationStore = create<
  NotificationStore &
  NotificationStoreActions
>((set) => ({
  notification: null,
  setNotification: (notification) => set({ notification }),
  
  error: null,
  setError: (error) => set({ error }),
  
  isError: false,
  setIsError: (isError) => set({ isError }),

  loading: false,
  setLoading: (loading) => set({ loading }),
}))