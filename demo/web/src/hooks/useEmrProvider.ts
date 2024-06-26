import { useCentralStore } from '@/Store';
import { useAuth } from '@/config/agent';
import { AppAgent } from '@/config/config';
import { emrCanisterId } from '@/lib/canister/emr.canister';
import {
  CreateEmrRequest,
  EmrFragment,
} from 'declarations/emr_registry/emr_registry.did';
import useProvider from './useProvider';
import { useRouter } from 'next/router';
import { toast } from 'react-toastify';
import { createActor } from 'declarations/provider_registry';
import { providerCanisterId } from '@/lib/canister/provider.canister';
import {
  EmrHeader,
  EmrListProviderRequest,
  IssueEmrRequest,
  UpdateEmrRequest,
} from 'declarations/provider_registry/provider_registry.did';
import { useEffect, useState } from 'react';

// CALL EMR LIST From PROVIDER canister
const useEmr = () => {
  const { identity, authenticated } = useAuth();
  const { provider } = useCentralStore();
  const { GetProviderInfo, providerName } = useProvider();

  const [initialValues, setInitialValues] = useState({
    // location: providerName ? providerName : '',
    amnanesis: '',
    medication: '',
    oxygen: '',
    temperature: '',
    blood: '',
    doctor: '',
  });

  const router = useRouter();
  const params = router.query;
  const userId = params.id;

  const api = createActor(providerCanisterId, { agent: AppAgent(identity) });

  async function createEmr(emr: EmrFragment[]) {
    const data: IssueEmrRequest = {
      emr: emr,
      user_id: userId as string,
    };
    console.log('data reques', data);
    const response = await api.issue_emr(data);
    console.log('-----------------');
    console.log('RESPONSE::::', response);
    console.log('-----------------');
    toast.success('Success Create Medical Record');
    setTimeout(() => {
      router.back();
    }, 2000);
    try {
    } catch (error) {
      console.log('-----------------');
      console.log('ERROR::::', error);
      console.log('-----------------');
    }
  }
  async function update(emr: EmrFragment[], emrHeader: EmrHeader) {
    const data: UpdateEmrRequest = {
      fields: emr,
      header: emrHeader,
    };
    console.log('data reques', data);
    const response = await api.update_emr(data);
    console.log('-----------------');
    console.log('RESPONSE::::', response);
    console.log('-----------------');
    toast.success('Success Update Medical Record');
    setTimeout(() => {
      router.back();
    }, 2000);
    try {
    } catch (error) {
      console.log('-----------------');
      console.log('ERROR::::', error);
      console.log('-----------------');
    }
  }

  async function GetEmrProvider() {
    const data: EmrListProviderRequest = {
      page: BigInt(0),
      limit: 10,
    };

    const response = await api.emr_list_provider(data);
    console.log('-----------------');
    console.log('RESPONSE:::: EMR', response);
    console.log('-----------------');
    toast.success('Success Create Medical Record');
    setTimeout(() => {
      router.back();
    }, 2000);
    try {
    } catch (error) {
      console.log('-----------------');
      console.log('ERROR::::', error);
      console.log('-----------------');
    }
  }

  return {
    createEmr,
    update,
    providerName,
    initialValues,
  };
};

export default useEmr;
