import useSWRImmutable from 'swr/immutable';
import { NFID } from '@nfid/embed';
import { useCallback, useState } from 'react';
import {
  emrCanisterId,
  emrCanisterIdMainnet,
} from '@/lib/canister/emr.canister';
import {
  patientCanisterId,
  patientCanisterIdMainnet,
} from '@/lib/canister/patient.canister';
import { providerCanisterId } from '@/lib/canister/provider.canister';
import { Identity, SignIdentity } from '@dfinity/agent';

const useAuth = () => {
  const [response, setResponse] = useState<any>(null);
  const [identity, setIdentitiy] = useState<Identity>();
  const targetCanisterIds = [emrCanisterIdMainnet, patientCanisterIdMainnet];
  const { data: nfid, error: nfidError } = useSWRImmutable('nfid', () =>
    NFID.init({}),
  );

  console.log('NFID:::', nfid);
  const handleAuthenticate = useCallback(async () => {
    console.log('AUTHENTICATING........');

    if (!nfid) return alert('NFID is not initialized');

    try {
      const identity = await nfid.getDelegation(
        targetCanisterIds.length
          ? {
              targets: targetCanisterIds,
              maxTimeToLive: BigInt(8) * BigInt(3_600_000_000_000),
              //   derivationOrigin: `https://bw4dl-smaaa-aaaaa-qaacq-cai.icp0.io`,
            }
          : undefined,
      );
      console.log('AUTHENTICATION SUCCESS:::', identity);
      setIdentitiy(identity);
      setResponse(identity);
    } catch (error: any) {
      setResponse({ error: error.message });
    }
  }, [nfid, targetCanisterIds, setResponse]);

  return {
    handleAuthenticate,
  };
};

export default useAuth;
