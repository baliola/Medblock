import { Actor, Cbor, HttpAgent, Identity } from '@dfinity/agent';
import { AuthClient } from '@dfinity/auth-client';
import { NFID } from '@nfid/embed';
import { rejects } from 'assert';
import {
  canisterId,
  createActor,
  patient_registry,
} from 'declarations/patient_registry';
import {
  EmrListConsentRequest,
  PatientListResponse,
  RegisterPatientRequest,
  UpdateInitialPatientInfoRequest,
  V1,
} from 'declarations/patient_registry/patient_registry.did';
import keccak256 from 'keccak256';
import { useRouter } from 'next/router';
import { useEffect, useState } from 'react';
import { toast } from 'react-toastify';

import { useAuth } from '@/config/agent';
import { AppAgent } from '@/config/config';
import { localStorageHelper } from '@/helpers/localStorage.helpers';
import {
  patientCanisterId,
  patientCanisterIdMainnet,
} from '@/lib/canister/patient.canister';
import { providerCanisterIdMainnet } from '@/lib/canister/provider.canister';
import { useCentralStore } from '@/Store';
// import * as CBOR from 'cbor-js'; // Make sure to import the cbor-js library

type Response = unknown; // whatever the canister method returns
enum DelegationType {
  GLOBAL = 0,
  ANONYMOUS = 1,
}
export interface ClaimConsentRequest {
  code: string;
}

export interface EmrListPatientRequest {
  page: number;
  limit: number;
}

const usePatient = () => {
  const { patientList, setPatientList } = useCentralStore();
  const { identity, authenticated } = useAuth();
  const router = useRouter();
  const [loading, setLoading] = useState<boolean>(false);
  const canister = patientCanisterId;
  const api = createActor(canister, { agent: AppAgent(identity) });
  // const [sessionId, setSessionId] = useState<string | undefined>();

  const [showModal, setShowModal] = useState<boolean>(false);
  const [showModalSession, setShowModalSession] = useState<boolean>(false);

  const toggleModal = () => {
    setShowModal(!showModal);
  };
  const toggleModalSession = () => {
    setShowModalSession(!showModalSession);
  };

  async function fetchPatient() {
    console.log('FETCH PATIENT RUNNING.....');
    try {
      console.log('FETCH PATIENT RUNNING.....');

      const response = await api?.patient_list();

      console.log('-----------------');
      console.log('RESPONSE::::', response);
      console.log('-----------------');

      setPatientList(response.patients);
    } catch (error) {
      console.log('-----------------');
      console.log('ERROR::::', error);
      console.log('-----------------');
    }
  }

  const shareConsetCode = async () => {
    setLoading(true);
    try {
      const response = await api?.create_consent();
      const consent = response?.code;
      console.log('-----------------');
      console.log('RESPONSE conscentt::::', response.code);
      console.log('-----------------');
      toast.success('Successfully Share consent code');
      setTimeout(() => {
        router.push({
          pathname: `/home/consent-code`,
          query: {
            consent: consent,
          },
        });
      }, 1000);
      setLoading(false);
    } catch (error) {
      setLoading(false);
      console.log('-----------------');
      console.log('ERROR::::', error);
      console.log('-----------------');
    }
  };

  const claimConsent = async (code: ClaimConsentRequest) => {
    try {
      const response = await api?.claim_consent(code);

      console.log('-----------------');
      console.log('RESPONSE conscentt::::', response);
      console.log('-----------------');

      fetchPatient();

      // setPatientList(response.code);
    } catch (error) {
      console.log('-----------------');
      console.log('ERROR::::', error);
      console.log('-----------------');
    }
  };

  const claimConsentToGetSession = async (code: ClaimConsentRequest) => {
    try {
      const response = await api?.claim_consent(code);
      const session = await response?.session_id;
      console.log('-----------------');
      console.log('RESPONSE conscentt::::', session);
      console.log('-----------------');
      // if (sessionId) {
      // setSessionId(session);
      localStorageHelper.setItem('session', session);
      // }
      // setPatientList(response.code);
    } catch (error) {
      console.log('-----------------');
      console.log('ERROR::::', error);
      console.log('-----------------');
    }
  };

  // Function to generate hash and encode to example format
  const generateAndEncodeHash = (nik: string): string => {
    // Generate the hash using Keccak
    const hashBuffer = keccak256(nik);

    // Encode the hash to hexadecimal string
    const encodedHash = Buffer.from(hashBuffer).toString('hex');

    return encodedHash;
  };
  const registerPatient = async (request: RegisterRequest) => {
    const nik = generateAndEncodeHash(request.nik);
    console.log('nik generated', nik);
    const data: RegisterPatientRequest = {
      nik: nik,
    };
    try {
      const response = await api?.register_patient(data);

      console.log('-----------------');
      console.log('RESPONSE REGISTER::::', response);
      console.log('-----------------');
      updateInfoPatient(request);

      // setPatientList(response.code);
    } catch (error) {
      console.log('-----------------');
      console.log('ERROR::::', error);
      console.log('-----------------');
    }
  };

  const updateInfoPatient = async (request: RegisterRequest) => {
    const data: UpdateInitialPatientInfoRequest = {
      info: {
        address: request.address,
        date_of_birth: request.date_of_birth,
        gender: request.gender,
        martial_status: request.martial_status,
        name: request.name,
        place_of_birth: request.place_of_birth,
      },
    };
    try {
      const response = await api?.update_initial_patient_info(data);

      console.log('-----------------');
      console.log('RESPONSE UPDATE PATIENT INFO::::', response);
      console.log('-----------------');
      toast.success('SUCCESS REGISTRATION');
      setTimeout(() => {
        router.push('/verified');
      }, 3000);

      // setPatientList(response.code);
    } catch (error) {
      console.log('-----------------');
      console.log('ERROR UPDATE PATIENT DUMMY::::', error);
      console.log('-----------------');
    }
  };

  useEffect(() => {
    if (identity) fetchPatient();
  }, [identity]);

  return {
    fetchPatient,
    patientList,
    shareConsetCode,
    claimConsent,
    registerPatient,
    loading,
    updateInfoPatient,
    claimConsentToGetSession,
    toggleModal,
    setShowModal,
    showModal,
    toggleModalSession,
    setShowModalSession,
    showModalSession,
  };
};

export default usePatient;

// export default canister;
