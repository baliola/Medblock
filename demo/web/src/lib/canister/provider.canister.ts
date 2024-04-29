import { canisterId } from 'declarations/provider_registry';

export const providerCanisterIdLocal =
  canisterId ?? 'bkyz2-fmaaa-aaaaa-qaaaq-cai';
export const providerCanisterIdMainnet = 'tawg2-3aaaa-aaaak-akn7q-cai';

export const providerCanisterId =
  process.env.NEXT_PUBLIC_APP_DFX_NETWORK === 'ic'
    ? providerCanisterIdMainnet
    : providerCanisterIdLocal;
