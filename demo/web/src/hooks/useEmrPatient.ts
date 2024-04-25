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
} from 'declarations/patient_registry/patient_registry.did';
import { useRouter } from 'next/router';
import { useEffect, useState } from 'react';
import { toast } from 'react-toastify';

// Call emr and info patient from patient canister
const useEMRPatient = () => {
  const { identity, authenticated } = useAuth();
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
  const { providerId, userId } = router.query;
  const [isLoading, setIsLoading] = useState(false);

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
    const data: EmrListConsentRequest = {
      session_id: session as string,
      page: 0,
      limit: 10,
    };

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
    setIsLoading(true);
    console.log('principal detail', identity?.getPrincipal());
    const data: ReadEmrByIdRequest = {
      provider_id: providerId as string,
      registry_id: identity?.getPrincipal() as Principal,
      emr_id: emrId as string,
    };

    console.log('get detail data request', data);
    try {
      const response = await api.read_emr_by_id(data);
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
      console.log('CANISTER ERROR', canisterError?.message);
      if (canisterError?.message.includes(ErrorMessages.AnonimError)) {
        // router.push('/auth/login');
        // toast.error('Provider info does not exist');
      } else {
        console.log('-----------------');
        toast.error(canisterError?.message);
        console.log('ERROR::::', error);
      }
      setIsLoading(false);
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
    GetEmrDetail,
    params,
    patientInfo,
    emrList,
    emr,
    initialValues,
    isLoading,
  };
};

export default useEMRPatient;
