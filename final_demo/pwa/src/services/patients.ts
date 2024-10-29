import { createActorContext } from '@ic-reactor/react';

import {
  canisterId,
  idlFactory,
  patient_registry,
} from '@/declarations/patient_registry';
import keccak256 from 'keccak256';

type Actor = typeof patient_registry;

export const {
  ActorProvider: PatientActor,
  useQueryCall: usePatientQuery,
  useUpdateCall: usePatientUpdate,
  useMethod: usePatientMethod
} = createActorContext<Actor>({
  canisterId,
  idlFactory: idlFactory as any
});

export const encodeHashNIK = (nik: string) => {
  const hashBuffer = keccak256(nik);
  const encodedHash = Buffer
    .from(hashBuffer)
    .toString('hex');

  return encodedHash;
};