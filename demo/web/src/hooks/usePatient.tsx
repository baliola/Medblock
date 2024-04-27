import {
  canisterId,
  createActor,
  patient_registry,
} from 'declarations/patient_registry';
import { useEffect, useState } from 'react';
import {
  patientCanisterId,
  patientCanisterIdMainnet,
} from '@/lib/canister/patient.canister';
import { Actor, Cbor, HttpAgent, Identity } from '@dfinity/agent';
import { AuthClient } from '@dfinity/auth-client';
import { useRouter } from 'next/router';
import { useCentralStore } from '@/Store';
import { providerCanisterIdMainnet } from '@/lib/canister/provider.canister';
import { NFID } from '@nfid/embed';
import {
  EmrListConsentRequest,
  PatientListResponse,
  PatientWithNikAndSession,
  RegisterPatientRequest,
  SearchPatientRequest,
  UpdateInitialPatientInfoRequest,
  V1,
} from 'declarations/patient_registry/patient_registry.did';
import keccak256 from 'keccak256';
import { AppAgent } from '@/config/config';
import { useAuth } from '@/config/agent';
import { localStorageHelper } from '@/helpers/localStorage.helpers';
import { rejects } from 'assert';
import { createCanisterError } from '@/lib/CanisterError';
import { toast } from 'react-toastify';
import { ErrorMessages } from '@/lib/constant';
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
  const {
    patientList,
    setPatientList,
    sessionId,
    setSessionId,
    searchResult,
    setSearchResult,
    isLoading,
    setIsloading,
    setPatientName,
  } = useCentralStore();
  const { identity, authenticated } = useAuth();
  const router = useRouter();
  const canister = patientCanisterId;
  const api = createActor(canister, { agent: AppAgent(identity) });
  const [searchQuery, setSearchQuery] = useState('');
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
      const canisterError = createCanisterError(error);
      console.log('-----------------');
      console.log('CANISTER ERROR::::', canisterError?.message);
      if (canisterError?.message.includes(ErrorMessages.ProviderDoesNotExist)) {
        router.push('/registration');
      }
      console.log('-----------------');
      console.log('-----------------');
      console.log('ERROR::::', error);
      console.log('-----------------');
    }
  }

  const createdummyConsent = async () => {
    try {
      const response = await api?.create_consent();

      console.log('-----------------');
      console.log('RESPONSE conscentt::::', response.code);
      console.log('-----------------');

      // setPatientList(response.code);
    } catch (error) {
      console.log('-----------------');
      console.log('ERROR::::', error);
      console.log('-----------------');
    }
  };

  const claimConsent = async (code: ClaimConsentRequest) => {
    try {
      const response = await api?.claim_consent(code);

      console.log('-----------------');
      console.log('RESPONSE conscentt Hello::::', response);
      console.log('-----------------');
      localStorageHelper.setItem('session', response.session_id);
      setPatientName(response.name);
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
      setSessionId(session);
      localStorageHelper.setItem('session', session);
      // }
      // setPatientList(response.code);
    } catch (error) {
      console.log('-----------------');
      console.log('ERROR::::', error);
      console.log('-----------------');
    }
  };
  const generateRandomNumber = (): string => {
    const randomNumber = Math.floor(
      1000000000000000 + Math.random() * 9000000000000000,
    );
    return String(randomNumber);
  };

  // Function to generate hash and encode to example format
  const generateAndEncodeHash = (nik?: string): string => {
    // Generate a random 16-digit number
    const value = nik ?? generateRandomNumber();
    console.log('nik not generated', generateRandomNumber());

    // Generate the hash using Keccak
    const hashBuffer = keccak256(value);

    // Encode the hash to hexadecimal string
    const encodeNik = Buffer.from(hashBuffer).toString('hex');

    return encodeNik;
  };
  const registerDummyPatient = async () => {
    const nik = generateAndEncodeHash();
    console.log('nik generated', nik);
    const data: RegisterPatientRequest = {
      nik: nik,
    };
    try {
      const response = await api?.register_patient(data);

      console.log('-----------------');
      console.log('RESPONSE conscentt::::', response);
      console.log('-----------------');
      updateInfoDummyPatient();

      // setPatientList(response.code);
    } catch (error) {
      console.log('-----------------');
      console.log('ERROR::::', error);
      console.log('-----------------');
    }
  };

  const getRandomName = (): string => {
    const names = [
      'John',
      'Emma',
      'Michael',
      'Sophia',
      'William',
      'Olivia',
      'James',
      'Ava',
      'Alexander',
      'Isabella',
    ];
    return names[Math.floor(Math.random() * names.length)];
  };

  const getRandomMartialStatus = (): string => {
    const statuses = ['Single', 'Married', 'Divorced', 'Widowed'];
    return statuses[Math.floor(Math.random() * statuses.length)];
  };

  const getRandomPlaceOfBirth = (): string => {
    const places = [
      'New York',
      'Los Angeles',
      'London',
      'Paris',
      'Tokyo',
      'Sydney',
      'Berlin',
      'Rome',
      'Moscow',
    ];
    return places[Math.floor(Math.random() * places.length)];
  };

  const getRandomAddress = (): string => {
    const addresses = [
      '123 Main St',
      '456 Elm St',
      '789 Oak Ave',
      '101 Pine Ln',
      '555 Maple St',
      '777 Cedar Dr',
    ];
    return addresses[Math.floor(Math.random() * addresses.length)];
  };

  const getRandomGender = (): string => {
    const genders = ['Male', 'Female', 'Non-binary', 'Other'];
    return genders[Math.floor(Math.random() * genders.length)];
  };

  const getRandomDateOfBirth = (): string => {
    // Generating a random date between 1950 and 2000 for simplicity
    const year = Math.floor(Math.random() * (2000 - 1950 + 1)) + 1950;
    const month = Math.floor(Math.random() * 12) + 1;
    const day = Math.floor(Math.random() * 28) + 1; // Assuming all months have max 28 days
    return `${year}-${month.toString().padStart(2, '0')}-${day
      .toString()
      .padStart(2, '0')}`;
  };

  const generateRandomV1Values = (): V1 => {
    return {
      name: getRandomName(),
      martial_status: getRandomMartialStatus(),
      place_of_birth: getRandomPlaceOfBirth(),
      address: getRandomAddress(),
      gender: getRandomGender(),
      date_of_birth: getRandomDateOfBirth(),
    };
  };
  const updateInfoDummyPatient = async () => {
    const randomV1Values = generateRandomV1Values();
    const data: UpdateInitialPatientInfoRequest = {
      info: randomV1Values,
    };
    try {
      const response = await api?.update_initial_patient_info(data);

      console.log('-----------------');
      console.log('RESPONSE conscentt::::', response);
      console.log('-----------------');

      // setPatientList(response.code);
    } catch (error) {
      console.log('-----------------');
      console.log('ERROR UPDATE PATIENT DUMMY::::', error);
      console.log('-----------------');
    }
  };

  const searchPatient = async (value: string) => {
    setIsloading(true);
    console.log('search value', value);
    const nik = generateAndEncodeHash(value);
    console.log('search value generated', nik);
    const data: SearchPatientRequest = {
      nik: nik,
    };
    await api
      .search_patient(data)
      .then((resp) => {
        console.log('Search Response', resp);
        const updatedSearchResult = [resp.patient_info];
        setSearchResult(resp.patient_info);
        setIsloading(false);
      })
      .catch((e) => {
        setIsloading(false);
        setSearchResult(null);
        console.log('------------------');
        console.log('Search not found', e);
        console.log('------------------');
      });
  };

  useEffect(() => {
    if (identity) fetchPatient();
  }, [identity]);

  return {
    fetchPatient,
    patientList,
    createdummyConsent,
    claimConsent,
    registerDummyPatient,
    updateInfoDummyPatient,
    claimConsentToGetSession,
    sessionId,
    toggleModal,
    setShowModal,
    showModal,
    toggleModalSession,
    setShowModalSession,
    showModalSession,
    searchPatient,
    searchQuery,
    setSearchQuery,
    searchResult,
  };
};

export default usePatient;

// export default canister;
