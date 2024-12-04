import { canisterId } from '@/declarations/patient_registry';

const patientCanisterIdLocal =
  canisterId ?? process.env.NEXT_PUBLIC_LOCAL_CANISTER_PATIENT;
const patientCanisterIdMainnet = 't3t27-byaaa-aaaak-akn5a-cai';

const dfxNetwork =
  process.env.NODE_ENV === 'development' ? 'local' : 'ic';

export const patientCanisterId =
  dfxNetwork === 'ic'
    ? patientCanisterIdMainnet
    : patientCanisterIdLocal;
