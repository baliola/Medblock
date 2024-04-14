import { useAuth } from '@/config/agent';
import { AppAgent } from '@/config/config';
import { emrCanisterId } from '@/lib/canister/emr.canister';
import { createActor } from 'declarations/emr_registry';
import {
  CreateEmrRequest,
  EmrFragment,
} from 'declarations/emr_registry/emr_registry.did';

const useEmr = () => {
  const { identity, authenticated } = useAuth();
  const api = createActor(emrCanisterId, { agent: AppAgent(identity) });

  async function createEmr(emr: EmrFragment[]) {
    const data: CreateEmrRequest = {
      emr: emr,
      provider_id: '',
      user_id: '',
    };
    const response = await api?.create_emr(data);
    console.log('-----------------');
    console.log('RESPONSE::::', response);
    console.log('-----------------');
    try {
    } catch (error) {
      console.log('-----------------');
      console.log('ERROR::::', error);
      console.log('-----------------');
    }
  }
  return {
    createEmr,
  };
};

export default useEmr;
