import {
  canisterId,
  createActor,
  patient_registry,
} from 'declarations/patient_registry';
import { useEffect, useState } from 'react';
import useAuth from './useAuth';
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
  PatientListResponse,
  RegisterPatientRequest,
  UpdateInitialPatientInfoRequest,
  V1,
} from 'declarations/patient_registry/patient_registry.did';
import keccak256 from 'keccak256';

import { Patient } from '@/interface/patient.interface';
import { NFIDS } from '@/interface/nfid.interface';
import { toast } from 'react-toastify';
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
    agent,
    setAgent,
    setIdentity,
    identity,
    patientList,
    setPatientList,
  } = useCentralStore();
  const router = useRouter();
  const canister = patientCanisterIdMainnet;
  const api = createActor(canister, { agent });

  async function fetchPatient() {
    console.log('FETCH PATIENT RUNNING.....');
    const request = {
      page: BigInt(0),
      limit: 10,
    };
    // const nfid = await NFIDS();

    try {
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
  const generateRandomNumber = (): string => {
    const randomNumber = Math.floor(
      1000000000000000 + Math.random() * 9000000000000000,
    );
    return String(randomNumber);
  };

  // Function to generate hash and encode to example format
  const generateAndEncodeHash = (): string => {
    // Generate a random 16-digit number
    const randomNum = generateRandomNumber();

    // Generate the hash using Keccak
    const hashBuffer = keccak256(randomNum);

    // Encode the hash to hexadecimal string
    const encodedHash = Buffer.from(hashBuffer).toString('hex');

    return encodedHash;
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

  const generateRandomString = (): string => {
    return Math.random().toString(36).substr(2, 8);
  };
  const generateRandomV1Values = (): V1 => {
    return {
      martial_status: generateRandomString(),
      place_of_birth: generateRandomString(),
      address: generateRandomString(),
      gender: generateRandomString(),
      date_of_birth: generateRandomString(),
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
      console.log('ERROR::::', error);
      console.log('-----------------');
    }
  };

  const getAgent = () => {
    const localIdentityString = localStorage.getItem('identity');
    if (localIdentityString) {
      const localIdentity: Identity = JSON.parse(localIdentityString);
      console.log('agent local', localIdentity);
      console.log('identity', localIdentity);

      // setIdentity(localIdentity);
    } else {
      router.push('/auth/login');
    }
  };

  useEffect(() => {
    console.log('AGENT NIH BRO', identity);
    updateInfoDummyPatient();
    // if (agent) {
    fetchPatient();
    // }
  }, []);

  // useEffect(() => {}, [delegation, identity]);

  return {
    fetchPatient,
    patientList,
    createdummyConsent,
    claimConsent,
    registerDummyPatient,
    updateInfoDummyPatient,
  };
};

export default usePatient;

// export default canister;
