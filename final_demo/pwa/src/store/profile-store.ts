import { GetPatientInfoResponse } from '@/declarations/patient_registry/patient_registry.did';
import { create } from 'zustand';

interface ProfileStore{
  profile: GetPatientInfoResponse | null;
}

interface ProfileStoreActions{
  setProfile: (profile: GetPatientInfoResponse) => void;
}

export const useProfileStore = create<
  ProfileStore &
  ProfileStoreActions
>((set) => ({
  profile: null,
  setProfile: (profile) => set({ profile })
}));