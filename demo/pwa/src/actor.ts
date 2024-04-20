import { createReactor } from '@ic-reactor/react';
import {
  canisterId,
  idlFactory,
  patient_registry,
} from 'declarations/patient_registry';

type Actor = typeof patient_registry;

export const { useAuth, useQueryCall, useActorState } = createReactor<Actor>({
  canisterId,
  idlFactory,
  host: 'https://localhost:4943',
});
