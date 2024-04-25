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

import { useAuth } from '@/config/agent';
import { AppAgent } from '@/config/config';
import { localStorageHelper } from '@/helpers/localStorage.helpers';
import { createCanisterError } from '@/interface/CanisterError';
import { ErrorMessages } from '@/interface/constant';
import { patientCanisterId } from '@/lib/canister/patient.canister';
import { useCentralStore } from '@/Store';

// Call emr and info patient from patient canister
const useEMRPatient = () => {
  const { identity, authenticated } = useAuth();
  // const { setNik } = useCentralStore();
  const [patientInfo, setPatientInfo] = useState<Patient | null>();
  const [nik, setNik] = useState('');
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
  const emrId = params.id;
  const { providerId, userId } = router.query;
  const [isLoading, setIsLoading] = useState(false);

  const api = createActor(patientCanisterId, { agent: AppAgent(identity) });

  async function getPatientInfo() {
    console.log('FETCH PATIENT RUNNING.....');
    console.log('principal ingbok', identity?.getPrincipal().toText());
    try {
      console.log('FETCH PATIENT RUNNING 11.....');

      const response = await api.get_patient_info();
      console.log('-----------------');
      console.log('RESPONSE:::: INGBOX', response);
      console.log('-----------------');
      setPatientInfo(response.patient);
      setNik(response.nik);

      // setPatientList(response.patients);
    } catch (error) {
      const canisterError = createCanisterError(error);
      console.log('CANISTER ERROR', canisterError);
      if (canisterError?.message.includes(ErrorMessages.AnonimError)) {
        router.push('/auth/login');
        // toast.error('Provider info does not exist');
      } else {
        console.log('-----------------');
        toast.error(canisterError?.message);
        console.log('ERROR:::: ingbok', error);
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
      setEmrList(response.emrs);
    } catch (error) {
      setEmrList([]);
      console.log('-----------------');
      console.log('ERROR::::', error);
      console.log('-----------------');
    }
  }
  async function GetEmrDetail() {
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
    if (identity) {
      getPatientInfo();
      GetEmr();
    }
    if (emrId && providerId) GetEmrDetail();
  }, [identity]);

  useEffect(() => {
    if (emrId && providerId && identity) GetEmrDetail();
  }, [emrId, providerId]);

  return {
    getPatientInfo,
    GetEmrDetail,
    params,
    patientInfo,
    emrList,
    emr,
    initialValues,
    isLoading,
    nik,
  };
};

export default useEMRPatient;
