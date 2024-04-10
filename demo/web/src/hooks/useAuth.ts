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
import { Identity, SignIdentity } from '@dfinity/agent';
import { toast } from 'react-toastify';
import { useRouter } from 'next/router';
import { useCentralStore } from '@/Store';

const useAuth = () => {
  const [response, setResponse] = useState<any>(null);
  const [identity, setIdentitiy] = useState<Identity>();
  const targetCanisterIds = [
    providerCanisterIdMainnet,
    patientCanisterIdMainnet,
  ];
  const { data: nfid, error: nfidError } = useSWRImmutable('nfid', () =>
    NFID.init({}),
  );

  const router = useRouter();

  //   function handleSuccess() {
  //     const { setClient, setUserPrincipal } = useCentralStore();
  //     const principalId = nfid?.getIdentity().getPrincipal().toText();
  //     console.log('--------------');
  //     setUserPrincipal(principalId);
  //     toast.success('Login successfully');
  //     setTimeout(() => {
  //       router.push('/');
  //     }, 3000);
  //     console.log('--------------');
  //   }

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
      toast.success('Login successfully');
      setTimeout(() => {
        router.push('/');
      }, 3000);
    } catch (error: any) {
      setResponse({ error: error.message });
    }
  }, [nfid, targetCanisterIds, setResponse]);

  const checkAuthentication = async () => {
    const isAuthenticated = await nfid?.isAuthenticated;
    if (!isAuthenticated) {
      router.push('/auth/login');
    }
    return isAuthenticated;
  };

  const signOut = async () => {
    try {
      const principalId = nfid?.getIdentity().getPrincipal();
      const isAuthenticated = await nfid?.isAuthenticated;
      console.log('Principal', principalId);
      console.log('isAuthenticated', isAuthenticated);

      const resp = await nfid?.logout();
      console.log('LOGOUT SUCCESSS', resp);
      router.push('/auth/login');
    } catch (error) {
      console.log('ERROR', error);
    }
  };

  return {
    handleAuthenticate,
    checkAuthentication,
    signOut,
  };
};

export default useAuth;
