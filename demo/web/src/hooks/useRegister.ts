import { useAuth, useUserPrincipal } from '@/config/agent';
import { AppAgent } from '@/config/config';
import { RegisterProviderRequest } from '@/interface/Provider.interface';
import { createCanisterError } from '@/lib/CanisterError';
import { providerCanisterId } from '@/lib/canister/provider.canister';
import { ErrorMessages } from '@/lib/constant';
import { Principal } from '@dfinity/principal';
import { createActor } from 'declarations/provider_registry';
import { RegisternewProviderRequest } from 'declarations/provider_registry/provider_registry.did';
import { useRouter } from 'next/router';
import { toast } from 'react-toastify';

const useRegister = () => {
  const { identity, authenticated } = useAuth();
  const principal = useUserPrincipal();
  const router = useRouter();

  const api = createActor(providerCanisterId, {
    agent: AppAgent(identity),
  });

  const registerProvider = async (req: RegisterProviderRequest) => {
    console.log('FETCH PROVIDER REGISTRATIONS RUNNING.....');
    try {
      const data: RegisternewProviderRequest = {
        provider_principal: principal as Principal,
        display_name: req.displayName,
        address: req.address,
      };
      const response = await api.register_new_provider(data);
      console.log('FETCH PROVIDER REGISTRATION ENDED. STATUS:: SUCCESS');
      console.log('-----------------');
      console.log('REGISTRATSTION PROVIDER INFO RESPONSE::::', response);
      console.log('-----------------');
      toast.success('Registrations Success');
      setTimeout(() => {
        router.push('/home');
      }, 3000);
    } catch (error) {
      const canisterError = createCanisterError(error);
      if (canisterError?.message.includes(ErrorMessages.AnonymusCaller)) {
        console.log('-----------------');
        console.log('CANISTER ERROR::::', canisterError.message);
        router.push('/');
      } else {
        console.log('-----------------');
        console.log('ERROR::::', error);
      }
    }
    console.log('FETCH PROVIDER REGISTRATION ENDED.');
  };
  return {
    registerProvider,
    // registerProvider,
  };
};

export default useRegister;
