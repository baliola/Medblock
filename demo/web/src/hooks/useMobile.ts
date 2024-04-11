import { NFIDS } from '@/interface/nfid.interface';
import { patientCanisterIdMainnet } from '@/lib/canister/patient.canister';
import { providerCanisterIdMainnet } from '@/lib/canister/provider.canister';
import { AuthClient } from '@dfinity/auth-client';
import { useCallback, useEffect, useState } from 'react';
import { toast } from 'react-toastify';

const useMobile = () => {
  const [client, setClient] = useState<AuthClient>();
  const [signIn, setSignedIn] = useState(false);
  const [principal, setPrincipal] = useState<string>('');
  const targetCanisterIds = [
    providerCanisterIdMainnet,
    patientCanisterIdMainnet,
  ];
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
    await new Promise<void>((resolve, reject) => {
      const redirectUri = 'https://www.google.com'; // enter redirect uri
      client?.login({
        onSuccess: async (resp) => {
          const identity = client.getIdentity();
          const principal = identity.getPrincipal().toString();
          try {
            const delegationIdentity = (await NFIDS()).getDelegation(
              targetCanisterIds.length
                ? {
                    targets: targetCanisterIds,
                    maxTimeToLive: BigInt(8) * BigInt(3_600_000_000_000),
                  }
                : undefined,
            );
            console.log('delegation identity', delegationIdentity);
            var delegationString = JSON.stringify(await delegationIdentity); // deleagation string

            console.log('delegation string ', delegationString);
            const encodedDelegation = encodeURIComponent(delegationString); // encode delegation
            const url = `${redirectUri}/redirect?delegation=${encodedDelegation}`; // uri
            console.log(`Redirecting to ${url}`);
            window.open(url, '_self');
          } catch (error) {
            console.log('hello');
          }
          //   });
        },
        onError: reject,
      });
    });
  };

  useEffect(() => {
    initAuth();
  }, []);

  return {
    handleLogin,
  };
};

export default useMobile;
