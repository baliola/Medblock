import { canisterId } from 'declarations/patient_registry';

export const patientCanisterIdLocal =
  canisterId ?? 'bd3sg-teaaa-aaaaa-qaaba-cai';
export const patientCanisterIdMainnet = 't3t27-byaaa-aaaak-akn5a-cai';

export const patientCanisterId =
  process.env.DFX_NETWORK === 'ic'
    ? patientCanisterIdMainnet
    : patientCanisterIdLocal;
