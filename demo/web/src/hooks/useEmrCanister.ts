import { useAuth } from '@/config/agent';
import { AppAgent } from '@/config/config';
import { createCanisterError } from '@/lib/CanisterError';
import { patientCanisterId } from '@/lib/canister/patient.canister';
// import { emrCanisterId } from '@/lib/canister/emr.canister';
import { ErrorMessages } from '@/lib/constant';
import { Principal } from '@dfinity/principal';
import { createActor } from 'declarations/patient_registry';
import {
  EmrHeaderWithBody,
  EmrHeaderWithStatus,
  EmrListConsentRequest,
  ReadEmrByIdRequest,
  ReadEmrSessionRequest,
} from 'declarations/patient_registry/patient_registry.did';

import { useRouter } from 'next/router';
import { useEffect, useState } from 'react';
import { toast } from 'react-toastify';

const useCallEMRCanister = () => {
  const { identity, authenticated } = useAuth();
  const api = createActor(patientCanisterId, { agent: AppAgent(identity) });
  const router = useRouter();
  const { providerId, sessions } = router.query;
  const params = router.query;
  const emrId = params.id;
  const [isLoading, setIsLoading] = useState(false);
  const [initialValues, setInitialValues] = useState({
    location: 'denpasar',
    amnanesis: '',
    medication: '',
    oxygen: '',
    temperature: '',
    blood: '',
    doctor: '',
  });
  const [emrList, setEmrList] = useState<EmrHeaderWithStatus[]>([]);
  const [emr, setEmr] = useState<EmrHeaderWithBody>();

  console.log('Params provide', router);
  console.log('Params usr', sessions);
  async function GetEmr(session: string) {
    console.log('running add emr from med record');
    const data: EmrListConsentRequest = {
      session_id: session as string,
      page: 0,
      limit: 10,
    };
    console.log('data get emr session', data.session_id);
    try {
      // NOTES GANTI KE EMR_LIST_PATIENT_WITH_SESSIONG
      const response = await api.emr_list_with_session(data);
      console.log('-----------------');
      console.log('RESPONSE:::: EMR', response);
      console.log('-----------------');
      setEmrList(response.emr);
    } catch (error) {
      console.log('-----------------');
      console.log('ERROR:::: EMR', error);
      console.log('-----------------');
    }
  }

  async function GetEmrDetail(providerId: string, emrId: string) {
    console.log('running add emr from med record detail');

    setIsLoading(true);
    console.log('principal detail', identity?.getPrincipal());
    const arg: ReadEmrByIdRequest = {
      provider_id: providerId as string,
      registry_id: identity?.getPrincipal() as Principal,
      emr_id: emrId as string,
    };

    console.log('arg', arg);
    const data: ReadEmrSessionRequest = {
      session_id: sessions as string,
      args: arg,
    };

    console.log('get detail data request', data);
    try {
      const response = await api.read_emr_with_session(data);
      console.log('-----------------');
      console.log('RESPONSE:::: EMR DETAIL', response);
      console.log('-----------------');
      setEmr(response.emr);
      setInitialValues({
        location: response.emr.body.find(
          (fragment) => fragment.key === 'location',
        )?.value as string,
        amnanesis: response.emr.body.find(
          (fragment) => fragment.key === 'amnanesis',
        )?.value as string,
        medication: response.emr.body.find(
          (fragment) => fragment.key === 'medication',
        )?.value as string,
        oxygen: response.emr.body.find((fragment) => fragment.key === 'oxygen')
          ?.value as string,
        temperature: response.emr.body.find(
          (fragment) => fragment.key === 'temperature',
        )?.value as string,
        blood: response.emr.body.find((fragment) => fragment.key === 'blood')
          ?.value as string,
        doctor: response.emr.body.find((fragment) => fragment.key === 'doctor')
          ?.value as string,
      });
      setIsLoading(false);
    } catch (error) {
      const canisterError = createCanisterError(error);
      console.log('CANISTER ERROR ', canisterError?.message);
      if (canisterError?.message.includes(ErrorMessages.AnonimError)) {
        // router.push('/auth/login');
        // toast.error('Provider info does not exist');
      } else {
        console.log('-----------------');
        // toast.error(canisterError?.message);
        console.log('ERROR::::', error);
      }
      // setIsLoading(false);
    }
  }

  return {
    providerId,
    setIsLoading,
    sessions,
    emrId,
    emr,
    emrList,
    initialValues,
    isLoading,
    GetEmrDetail,
    GetEmr,
  };
};

export default useCallEMRCanister;
