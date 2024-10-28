import { create } from 'zustand';

interface PinStore {
  pin: string;
}

interface PinStoreActions {
  setPin: (pin: string) => void;
}

export const usePinStore = create<
  PinStore &
  PinStoreActions
>((set) => ({
  pin: "",
  setPin: (pin) => set({ pin }),
}));