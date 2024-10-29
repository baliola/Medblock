import { createActorContext } from '@ic-reactor/react';

import {
  canisterId,
  idlFactory,
  patient_registry,
} from '@/declarations/patient_registry';

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
