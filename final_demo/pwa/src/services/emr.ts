import { emr_registry, idlFactory, canisterId } from '@/declarations/emr_registry';
import { createActorContext } from '@ic-reactor/react';

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
