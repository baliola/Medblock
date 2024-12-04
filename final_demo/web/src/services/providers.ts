import { createActorContext } from '@ic-reactor/react';

import {
  canisterId,
  idlFactory,
  provider_registry,
} from '@/declarations/provider_registry';

type Actor = typeof provider_registry;

export const {
  ActorProvider: ProviderActor,
  useQueryCall: useProviderQuery,
  useUpdateCall: useProviderUpdate,
  useMethod: useProviderMethod
} = createActorContext<Actor>({
  canisterId,
  idlFactory: idlFactory as any
});