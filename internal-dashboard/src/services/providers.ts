import {
  canisterId,
  idlFactory,
  provider_registry,
} from "@/canister/declarations/provider_registry";
import { createActorContext } from "@ic-reactor/react";

type Actor = typeof provider_registry;

export const {
  ActorProvider: ProviderActor,
  useQueryCall: useProviderQuery,
  useUpdateCall: useProviderUpdate,
  useMethod: useProviderMethod,
} = createActorContext<Actor>({
  canisterId,
  idlFactory: idlFactory,
});
