import { canisterId } from 'declarations/patient_registry';

export const patientCanisterIdLocal =
  canisterId ?? 'bd3sg-teaaa-aaaaa-qaaba-cai';
export const patientCanisterIdMainnet = 't3t27-byaaa-aaaak-akn5a-cai';

export const patientCanisterId =
  process.env.NEXT_PUBLIC_APP_DFX_NETWORK === 'ic'
    ? patientCanisterIdMainnet
    : patientCanisterIdLocal;
