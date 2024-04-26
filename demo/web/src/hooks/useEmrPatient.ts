import { useCentralStore } from '@/Store';
import { useAuth } from '@/config/agent';
import { AppAgent } from '@/config/config';
import { localStorageHelper } from '@/helpers/localStorage.helpers';
import { createCanisterError } from '@/lib/CanisterError';
import { patientCanisterId } from '@/lib/canister/patient.canister';
import { ErrorMessages } from '@/lib/constant';
import { Principal } from '@dfinity/principal';
import { createActor } from 'declarations/patient_registry';
import {
  ClaimConsentResponse,
  EmrFragment,
  EmrHeader,
  EmrHeaderWithBody,
  EmrHeaderWithStatus,
  EmrListConsentRequest,
  EmrListPatientRequest,
  EmrListPatientResponse,
  Patient,
  ReadEmrByIdRequest,
  ReadEmrSessionRequest,
} from 'declarations/patient_registry/patient_registry.did';
import { useRouter } from 'next/router';
import { useEffect, useState } from 'react';
import { toast } from 'react-toastify';

// Call emr and info patient from patient canister
const useEMRPatient = () => {
  const { identity, authenticated } = useAuth();
  const { setSessionId } = useCentralStore();
  const { setNik } = useCentralStore();
  const [patientInfo, setPatientInfo] = useState<Patient | null>();
  const [emrList, setEmrList] = useState<EmrHeaderWithStatus[]>([]);
  const [emr, setEmr] = useState<EmrHeaderWithBody>();
  const [initialValues, setInitialValues] = useState({
    location: 'denpasar',
    amnanesis: '',
    medication: '',
    oxygen: '',
    temperature: '',
    blood: '',
    doctor: '',
  });

  const router = useRouter();
  const params = router.query;
  const session = params.id;
  const { name, sessions } = router.query;
  const [isLoading, setIsLoading] = useState(false);

  const api = createActor(patientCanisterId, { agent: AppAgent(identity) });

  async function getPatientInfo() {
    console.log('FETCH PATIENT RUNNING.....');
    const localSessionId = localStorageHelper.getItem('session');
    try {
      console.log('FETCH PATIENT RUNNING.....');
      const data: ClaimConsentResponse = {
        session_id: session as string,
        name: name as string,
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
      console.log('RESPONSE:::: EMR', response.emr);
      console.log('-----------------');
      setEmrList(response.emr);
    } catch (error) {
      console.log('-----------------');
      console.log('ERROR:::: EMR', error);
      console.log('-----------------');
    }
  }

  useEffect(() => {
    setIsLoading(true);
    console.log('sessions', session);
    if (session && identity) {
      console.log('sessions run', session);
      setSessionId(session as string);
      if (!router.asPath.includes('edit')) {
        getPatientInfo();
      }
      GetEmr();
      setIsLoading(false);
    }
  }, [session, identity]);
  return {
    getPatientInfo,
    // GetEmrDetail,
    params,
    patientInfo,
    emrList,
    emr,
    initialValues,
    isLoading,
    session,
  };
};

export default useEMRPatient;
