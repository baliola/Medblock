import { useCentralStore } from '@/Store';
import { useAuth } from '@/config/agent';
import { AppAgent } from '@/config/config';
import { createCanisterError } from '@/lib/CanisterError';
import { patientCanisterId } from '@/lib/canister/patient.canister';
import { ErrorMessages } from '@/lib/constant';
import { DetailType } from '@/scenes/Detail/Detail.scene';
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

  const [isLoading, setIsLoading] = useState(false);

  const api = createActor(patientCanisterId, { agent: AppAgent(identity) });

  async function getPatientInfo(session: string, name: string) {
    console.log('FETCH PATIENT INFO RUNNING.....', session);
    try {
      console.log('FETCH PATIENT INFO RUNNING..... 2');
      const data: ClaimConsentResponse = {
        session_id: session as string,
        name: name as string,
      };
      const response = await api?.get_patient_info_with_consent(data);
      console.log('-----------------');
      console.log('RESPONSE:::: INFO PATIEN', response);
      console.log('-----------------');
      setPatientInfo(response.patient);
      setNik(response.nik);

      // setPatientList(response.patients);
    } catch (error) {
      const canisterError = createCanisterError(error);
      console.log('INFO PATIENT ERROR', canisterError?.message);
      if (canisterError?.message.includes(ErrorMessages.AnonimError)) {
        router.push('/auth/login');
        // toast.error('Provider info does not exist');
      } else {
        console.log('-----------------');
        console.log('ERROR:::: INFO PATIENT', error);
      }
    }
  }

  async function GetEmr(session: string) {
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

  return {
    getPatientInfo,
    // GetEmrDetail,
    patientInfo,
    emrList,
    emr,
    initialValues,
    isLoading,
    GetEmr,
    // getNotifications,
  };
};

export default useEMRPatient;
