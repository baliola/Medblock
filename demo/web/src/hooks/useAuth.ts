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
import {
  providerCanisterId,
  providerCanisterIdMainnet,
} from '@/lib/canister/provider.canister';
import { HttpAgent, Identity, SignIdentity } from '@dfinity/agent';
import { toast } from 'react-toastify';
import { useRouter } from 'next/router';
import { useCentralStore } from '@/Store';
import { NFIDS } from '@/interface/nfid.interface';

const useAuth = () => {
  const [response, setResponse] = useState<any>(null);
  const [identity, setIdentitiy] = useState<Identity>();
  const targetCanisterIds = [
    providerCanisterIdMainnet,
    patientCanisterIdMainnet,
  ];
  const { setAgent } = useCentralStore();
  const [loading, setIsloading] = useState(false);

  const router = useRouter();
  const updateDelegation = async (nfid: NFID) => {
    try {
      const delegationIdentity: Identity = await nfid.updateGlobalDelegation({
        targets: targetCanisterIds,
      });
      console.log('delegate response update', delegationIdentity);
    } catch (error) {
      console.log('ERROR', error);
    }
  };

  //   console.log('NFID:::', nfid);
  const handleAuthenticate = useCallback(async () => {
    console.log('AUTHENTICATING........');
    setIsloading(true);

    const testNewNfid = await NFIDS();
    try {
      const identity = await testNewNfid.getDelegation(
        targetCanisterIds.length
          ? {
              targets: targetCanisterIds,
              maxTimeToLive: BigInt(8) * BigInt(3_600_000_000_000),
              //   derivationOrigin: `https://bw4dl-smaaa-aaaaa-qaacq-cai.icp0.io`,
            }
          : undefined,
      );
      console.log('AUTHENTICATION SUCCESS:::', identity);

      const newAgent = new HttpAgent({ host: 'https://ic0.app', identity });
      setAgent(newAgent);

      toast.success('Login successfully');
      setIsloading(false);
      setTimeout(() => {
        router.push('/');
      }, 3000);
    } catch (error: any) {
      setIsloading(false);
      setResponse({ error: error.message });
    }
  }, [targetCanisterIds, setResponse]);

  const checkAuthentication = async () => {
    // const isAuthenticated = await nfid?.isAuthenticated;
    // if (!isAuthenticated) {
    //   router.push('/auth/login');
    // }
    // return isAuthenticated;
  };

  const signOut = async () => {
    // try {
    //   const principalId = nfid?.getIdentity().getPrincipal();
    //   const isAuthenticated = await nfid?.isAuthenticated;
    //   console.log('Principal', principalId);
    //   console.log('isAuthenticated', isAuthenticated);
    //   const resp = await nfid?.logout();
    //   console.log('LOGOUT SUCCESSS', resp);
    //   router.push('/auth/login');
    // } catch (error) {
    //   console.log('ERROR', error);
    // }
  };

  // const delegationIdentity: Identity = await nfid.updateGlobalDelegation({
  //   targets: [
  //     'YOUR_CANISTER_ID_1',
  //     'YOUR_CANISTER_ID_2',
  //     'YOUR_CANISTER_ID_USER_SPECIFIC',
  //   ],
  // });

  return {
    handleAuthenticate,
    checkAuthentication,
    signOut,
    // nfid,
    identity,
  };
};

export default useAuth;
