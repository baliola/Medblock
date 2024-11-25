import { canisterId } from "@/canister/declarations/provider_registry";

const providerCanisterIdLocal =
  canisterId ?? process.env.NEXT_PUBLIC_LOCAL_CANISTER_PROVIDER;
const providerCanisterIdMainnet = 'tawg2-3aaaa-aaaak-akn7q-cai';

const dfxNetwork = 
  process.env.NODE_ENV === 'development' ? 'local' : 'ic'

export const providerCanisterId =
  dfxNetwork === 'ic'
    ? providerCanisterIdMainnet
    : providerCanisterIdLocal;
