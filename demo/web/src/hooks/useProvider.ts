import { useCentralStore } from '@/Store';
import { useAuth, useUserPrincipal } from '@/config/agent';
import { AppAgent } from '@/config/config';
import { RegisterProviderRequest } from '@/interface/Provider.interface';
import { createCanisterError } from '@/lib/CanisterError';
import { providerCanisterId } from '@/lib/canister/provider.canister';
import { ErrorMessages } from '@/lib/constant';
import { Principal } from '@dfinity/principal';
import { createActor } from 'declarations/provider_registry';
import {
  ProviderInfoRequest,
  RegisternewProviderRequest,
} from 'declarations/provider_registry/provider_registry.did';
import { useRouter } from 'next/router';
import { useEffect, useState } from 'react';
import { toast } from 'react-toastify';

const useProvider = () => {
  const { identity, authenticated } = useAuth();
  const { setProvider } = useCentralStore();
  const principal = useUserPrincipal();
  const [alertShow, setAlertShow] = useState(false);
  const router = useRouter();

  const api = createActor(providerCanisterId, { agent: AppAgent(identity) });

  const GetProviderInfo = async () => {
    console.log('PROVIDER PRINCIPAL', principal);
    console.log('FETCH PROVIDER RUNNING.....');
    const data: ProviderInfoRequest = {
      provider: [principal as Principal],
    };
    try {
      const response = await api.get_provider_info_with_principal(data);
      console.log('-----------------');
      console.log('PROVIDER INFO RESPONSE::::', response.providers);
      setProvider(response.providers[0]);
      console.log('-----------------');
      console.log('FETCH PROVIDER ENDED.');
    } catch (error) {
      const canisterError = createCanisterError(error);
      if (canisterError?.message.includes(ErrorMessages.ProviderDoesNotExist)) {
        router.push('/registration');
        // toast.error('Provider info does not exist');
      } else {
        console.log('-----------------');
        console.log('ERROR::::', error);
      }
    }
  };

  useEffect(() => {
    if (identity) GetProviderInfo();
  }, [identity]);

  return {
    GetProviderInfo,
    // registerProvider,
  };
};

export default useProvider;
