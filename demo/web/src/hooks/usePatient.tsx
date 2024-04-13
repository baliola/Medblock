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
import { PatientListResponse } from 'declarations/patient_registry/patient_registry.did';
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
  const { agent, setAgent, setIdentity, identity } = useCentralStore();
  const [patientList, setPatientList] = useState<Patient[]>();
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
  };
};

export default usePatient;

// export default canister;
