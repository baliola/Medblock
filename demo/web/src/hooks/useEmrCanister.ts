import { useAuth } from '@/config/agent';
import { AppAgent } from '@/config/config';
import { createCanisterError } from '@/lib/CanisterError';
import { emrCanisterId } from '@/lib/canister/emr.canister';
import { ErrorMessages } from '@/lib/constant';
import { Principal } from '@dfinity/principal';
import { createActor } from 'declarations/patient_registry';
import {
  EmrHeaderWithBody,
  ReadEmrByIdRequest,
} from 'declarations/patient_registry/patient_registry.did';

import { useRouter } from 'next/router';
import { useEffect, useState } from 'react';
import { toast } from 'react-toastify';

const useCallEMRCanister = () => {
  const { identity, authenticated } = useAuth();
  const api = createActor(emrCanisterId, { agent: AppAgent(identity) });
  const [emr, setEmr] = useState<EmrHeaderWithBody>();
  const router = useRouter();
  const { providerId, userId } = router.query;
  const params = router.query;
  const emrId = params.id;

  console.log('Params provide', router);
  console.log('Params usr', userId);

  async function GetEmrDetail(
    providerId: string,
    registryId: Principal,
    emrId: string,
  ) {
    const data: ReadEmrByIdRequest = {
      provider_id: providerId as string,
      registry_id: (await identity?.getPrincipal()) as Principal,
      emr_id: emrId as string,
    };

    console.log('get detail data request', data);
    try {
      const response = await api.read_emr_by_id(data);
      console.log('-----------------');
      console.log('RESPONSE:::: EMR DETAIL', response);
      console.log('-----------------');
      toast.success('Success Create Medical Record');
      setEmr(response.emr);
    } catch (error) {
      const canisterError = createCanisterError(error);
      console.log('CANISTER ERROR', canisterError?.message);
      if (canisterError?.message.includes(ErrorMessages.AnonimError)) {
        // router.push('/auth/login');
        // toast.error('Provider info does not exist');
      } else {
        console.log('-----------------');
        toast.error(canisterError?.message);
        console.log('ERROR::::', error);
      }
    }
  }

  // useEffect(() => {
  //   if (emrId && identity) {
  //     GetEmrDetail();
  //     console.log('Params usr', identity.getPrincipal());
  //   }
  // }, [emrId, providerId, userId, identity]);

  return {
    providerId,
    userId,
    emrId,
  };
};

export default useCallEMRCanister;
