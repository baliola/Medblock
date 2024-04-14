import { useAuth } from '@/config/agent';
import { AppAgent } from '@/config/config';
import { localStorageHelper } from '@/helpers/localStorage.helpers';
import { patientCanisterId } from '@/lib/canister/patient.canister';
import { createActor } from 'declarations/patient_registry';
import {
  ClaimConsentResponse,
  EmrListConsentRequest,
  Patient,
} from 'declarations/patient_registry/patient_registry.did';
import { useRouter } from 'next/router';
import { useEffect, useState } from 'react';

const useEMRPatient = () => {
  const { identity, authenticated } = useAuth();
  const [patientInfo, setPatientInfo] = useState<Patient | null>();
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

      // setPatientList(response.patients);
    } catch (error) {
      console.log('-----------------');
      console.log('ERROR::::', error);
      console.log('-----------------');
    }
  }
  useEffect(() => {
    if (session && identity) getPatientInfo();
  }, [session, identity]);
  return {
    getPatientInfo,
    params,
    patientInfo,
  };
};

export default useEMRPatient;
