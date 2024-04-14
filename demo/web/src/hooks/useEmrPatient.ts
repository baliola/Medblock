import { useCentralStore } from '@/Store';
import { useAuth } from '@/config/agent';
import { AppAgent } from '@/config/config';
import { localStorageHelper } from '@/helpers/localStorage.helpers';
import { createCanisterError } from '@/lib/CanisterError';
import { patientCanisterId } from '@/lib/canister/patient.canister';
import { ErrorMessages } from '@/lib/constant';
import { createActor } from 'declarations/patient_registry';
import {
  ClaimConsentResponse,
  EmrFragment,
  EmrHeader,
  EmrListConsentRequest,
  EmrListPatientRequest,
  EmrListPatientResponse,
  Patient,
} from 'declarations/patient_registry/patient_registry.did';
import { useRouter } from 'next/router';
import { useEffect, useState } from 'react';
import { toast } from 'react-toastify';

const useEMRPatient = () => {
  const { identity, authenticated } = useAuth();
  const { setNik } = useCentralStore();
  const [patientInfo, setPatientInfo] = useState<Patient | null>();
  const [emrList, setEmrList] = useState<EmrHeader[]>([]);
  const router = useRouter();
  const params = router.query;
  const session = params.id;
  console.log('params', params);
  console.log('identity', identity);

  const api = createActor(patientCanisterId, { agent: AppAgent(identity) });

  async function getPatientInfo() {
    console.log('FETCH PATIENT RUNNING.....');
    const localSessionId = localStorageHelper.getItem('session');
    try {
      console.log('FETCH PATIENT RUNNING.....');
      const data: ClaimConsentResponse = {
        session_id: session as string,
      };
      const response = await api?.get_patient_info_with_consent(data);
      console.log('-----------------');
      console.log('RESPONSE::::', response);
      console.log('-----------------');
      setPatientInfo(response.patient);
      setNik(response.nik);

      // setPatientList(response.patients);
    } catch (error) {
      const canisterError = createCanisterError(error);
      console.log('CANISTER ERROR', canisterError?.message);
      if (canisterError?.message.includes(ErrorMessages.AnonimError)) {
        router.push('/auth/login');
        // toast.error('Provider info does not exist');
      } else {
        console.log('-----------------');
        console.log('ERROR::::', error);
      }
    }
  }

  async function GetEmr() {
    const data: EmrListPatientRequest = {
      page: 0,
      limit: 10,
    };

    try {
      const response = await api.emr_list_patient(data);
      console.log('-----------------');
      console.log('RESPONSE:::: EMR', response);
      console.log('-----------------');
      toast.success('Success Create Medical Record');
      setEmrList(response.emrs);
    } catch (error) {
      console.log('-----------------');
      console.log('ERROR::::', error);
      console.log('-----------------');
    }
  }
  useEffect(() => {
    if (session && identity) {
      getPatientInfo();
      GetEmr();
    }
  }, [session, identity]);
  return {
    getPatientInfo,
    params,
    patientInfo,
    emrList,
  };
};

export default useEMRPatient;
