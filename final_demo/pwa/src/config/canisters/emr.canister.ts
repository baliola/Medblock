import { canisterId } from '@/declarations/emr_registry';

const emrCanisterIdLocal =
  canisterId ?? process.env.NEXT_PUBLIC_LOCAL_CANISTER_EMR ;
const emrCanisterIdMainnet = 'tvrxx-2iaaa-aaaak-akn4a-cai';

const dfxNetwork =
  process.env.NODE_ENV === 'development' ? 'local' : 'ic';

export const emrCanisterId =
  dfxNetwork === 'ic'
    ? emrCanisterIdMainnet
    : emrCanisterIdLocal;