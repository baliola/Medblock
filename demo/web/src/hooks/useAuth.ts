import useSWRImmutable from 'swr/immutable';
import { NFID } from '@nfid/embed';
import { useCallback, useEffect, useState } from 'react';
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
import { AuthClient } from '@dfinity/auth-client';
import { host } from '@/config/config';

const useAuth = () => {
  const [response, setResponse] = useState<any>(null);
  const [identity, setIdentitiy] = useState<Identity>();
  const [nfid, setNfid] = useState<NFID | null>(null);
  const [client, setClient] = useState<AuthClient>();
  const [signIn, setSignedIn] = useState(false);
  const [principal, setPrincipal] = useState<string>('');

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
  const initAuth = async () => {
    const client = await AuthClient.create();
    const isAuthenticated = await client.isAuthenticated();

    setClient(client);

    if (isAuthenticated) {
      const identity = client.getIdentity();
      const principal = identity.getPrincipal().toString();
      setSignedIn(true);
      setPrincipal(principal);
    }
  };

  const handleLogin = async () => {
    const APP_NAME = 'NFID example';
    const APP_LOGO = 'https://nfid.one/icons/favicon-96x96.png';
    const CONFIG_QUERY = `?applicationName=${APP_NAME}&applicationLogo=${APP_LOGO}`;
    const identityProvider = `https://nfid.one/authenticate${CONFIG_QUERY}`;

    await new Promise<void>((resolve, reject) => {
      client?.login({
        identityProvider: host,
        onSuccess: async () => {
          const identity = client.getIdentity();
          const principal = identity.getPrincipal().toString();
          //   setIsloading(false);
          //   setTimeout(() => {
          //     router.push('/');
          //   }, 3000);

          console.log('principal', principal);
          console.log('identity', identity);
          console.log('clinet', client);
          const newAgent = new HttpAgent({
            host,
            // host: 'http://127.0.0.1:4943',
            identity,
          });

          console.log('PRINCIAP', identity.getPrincipal().toText());
          setAgent(newAgent);
          setIdentitiy(identity);
          toast.success('Login successfully');
          setIsloading(false);
          setTimeout(() => {
            router.push('/');
          }, 3000);

          // Use identity and principal here if needed
          resolve();
        },
        onError: reject,
      });
    });
  };

  //   console.log('NFID:::', nfid);
  const handleAuthenticate = useCallback(async () => {
    console.log('AUTHENTICATING........');
    setIsloading(true);

    const testNewNfid = await NFIDS();
    setNfid(testNewNfid);
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

      const newAgent = new HttpAgent({
        host: 'https://identity.ic0.app',
        identity,
      });
      console.log('PRINCIAP', identity.getPrincipal().toText());
      setAgent(newAgent);
      setIdentitiy(identity);
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
    const isAuthenticated = await nfid?.isAuthenticated;
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

  useEffect(() => {
    initAuth();
  }, []);

  return {
    handleAuthenticate,
    checkAuthentication,
    signOut,
    // nfid,
    handleLogin,
    identity,
  };
};

export default useAuth;
