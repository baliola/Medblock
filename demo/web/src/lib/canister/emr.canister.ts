import { canisterId } from 'declarations/emr_registry';

export const emrCanisterIdLocal = canisterId ?? 'be2us-64aaa-aaaaa-qaabq-cai';
export const emrCanisterIdMainnet = 'tvrxx-2iaaa-aaaak-akn4a-cai';

export const emrCanisterId =
  process.env.DFX_NETWORK === 'ic' ? emrCanisterIdMainnet : emrCanisterIdLocal;
