import { createActorContext } from '@ic-reactor/react';

import {
  canisterId,
  idlFactory,
  emr_registry,
} from '@/declarations/emr_registry';

type Actor = typeof emr_registry;

export const {
  ActorProvider: EMRActor,
  useQueryCall: useEMRQuery,
  useUpdateCall: useEMRUpdate,
  useMethod: useEMRMethod
} = createActorContext<Actor>({
  canisterId,
  idlFactory: idlFactory as any
});
