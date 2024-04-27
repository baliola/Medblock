import { useAuth } from '@/config/agent';
import { AppAgent } from '@/config/config';
import { providerCanisterId } from '@/lib/canister/provider.canister';
import { createActor } from 'declarations/provider_registry';
import { GetProviderBatchRequest } from 'declarations/provider_registry/provider_registry.did';

const useProvider = () => {
  const { identity } = useAuth();
  const api = createActor(providerCanisterId, { agent: AppAgent(identity) });

  async function GetProviderInfo(id: string): Promise<string | undefined> {
    const data: GetProviderBatchRequest = {
      ids: [id],
    };
    try {
      const response = await api.get_provider_batch(data);
      console.log('------------------');
      console.log(
        'response provider info',
        response.providers[0].V1.display_name,
      );
      console.log('------------------');
      return response.providers[0].V1.display_name;
    } catch (error) {
      console.log('error provider info pwa', error);
    }
  }

  return { GetProviderInfo };
};

export default useProvider;
