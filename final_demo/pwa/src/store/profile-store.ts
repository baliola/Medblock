import { GetPatientInfoResponse } from '@/declarations/patient_registry/patient_registry.did';
import { create } from 'zustand';

interface ProfileStore{
  profile: GetPatientInfoResponse | null | undefined;
}

interface ProfileStoreActions{
  setProfile: (profile: GetPatientInfoResponse | null | undefined) => void;
}

export const useProfileStore = create<
  ProfileStore &
  ProfileStoreActions
>((set) => ({
  profile: undefined,
  setProfile: (profile) => set({ profile })
}));