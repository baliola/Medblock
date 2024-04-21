import { HttpAgent, Identity, SignIdentity } from '@dfinity/agent';
import { AuthClient, AuthClientLoginOptions } from '@dfinity/auth-client';
import { LogoutParameters } from '@ic-reactor/react/dist/types';
import { NFID } from '@nfid/embed';
import { useRouter } from 'next/router';
import { useCallback, useEffect, useState } from 'react';
import { toast } from 'react-toastify';

import { useAuth } from '@/config/agent';
import { host, loginHost } from '@/config/config';
import { NFIDS } from '@/interface/nfid.interface';
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
import { useCentralStore } from '@/Store';

const useAuthentication = () => {
  const [response, setResponse] = useState<any>(null);
  const [nfid, setNfid] = useState<NFID | null>(null);
  const [client, setClient] = useState<AuthClient>();
  const [signIn, setSignedIn] = useState(false);

  const { setUserPrincipal, setIdentity } = useCentralStore();

  const targetCanisterIds = [
    providerCanisterIdMainnet,
    patientCanisterIdMainnet,
  ];
  const { setAgent } = useCentralStore();
  const [loading, setIsloading] = useState(false);

  const hosts: AuthClientLoginOptions = { identityProvider: host };

  const router = useRouter();

  const { login, logout, authenticated, identity, loginError } = useAuth({
    onLoginSuccess: (principal) => {
      console.log(`Logged in as ${principal}`);
      toast.success('Login successfully');
      setIsloading(false);
      setTimeout(() => {
        router.push('/unverified');
      }, 3000);
    },
    onLoginError: (error: any) =>
      console.error(`Login failed: ${error.message}`),
  });

  const initAuth = async () => {
    const client = await AuthClient.create();
    const isAuthenticated = await client.isAuthenticated();

    setClient(client);

    if (isAuthenticated) {
      const identity = client.getIdentity();
      const principal = identity.getPrincipal().toString();
      setSignedIn(true);
      setUserPrincipal(principal);
    }
  };

  const handleLogin = async () => {
    setIsloading(true);
    await login(hosts);
    setIsloading(false);
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
      // setIdentitiy(identity);
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
    logout().then(() => {
      toast.success('Logout successfully');
      setTimeout(() => {
        router.push('/auth/login');
      }, 3000);
    });
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
    loading,
    // nfid,
    initAuth,
    authenticated,
    handleLogin,
    identity,
  };
};

export default useAuthentication;
