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
// import * as CBOR from 'cbor-js'; // Make sure to import the cbor-js library

type Response = unknown; // whatever the canister method returns
enum DelegationType {
  GLOBAL = 0,
  ANONYMOUS = 1,
}
export interface EmrListPatientRequest {
  page: number;
  limit: number;
}

const usePatient = () => {
  const { agent, setAgent, setIdentity, identity } = useCentralStore();
  const [patientList, setPatientList] = useState<Patient[]>();
  const router = useRouter();

  const canister = patientCanisterId;
  async function fetchPatient() {
    const newAgent = new HttpAgent({
      host: 'http://127.0.0.1:4943',
      identity,
    });
    // console.log(
    //   'identity in fetch patient',
    //   identity?.getPrincipal()?.toText(),
    // );

    const api = createActor(canister, { agent });
    console.log('FETCH PATIENT RUNNING.....');
    const request = {
      page: BigInt(0),
      limit: 10,
    };
    // const nfid = await NFIDS();

    try {
      const response = await api?.patient_list();
      // const response: Response = await nfid.requestCanisterCall({
      //   canisterId: patientCanisterIdMainnet, // the canister id which will be called
      //   method: 'patient_list', // the method on the canister which will be called
      //   parameters: undefined, // the parameters passed to the method on the canister
      // });
      console.log('-----------------');
      console.log('RESPONSE::::', response);
      console.log('-----------------');

      setPatientList(response.patients);
    } catch (error) {
      console.log('-----------------');
      console.log('ERROR::::', error);
      console.log('-----------------');
    }

    // const encodeParams = Cbor.encode(req);
    // const decoder = new TextDecoder('utf-8');
    // const paramsString = decoder.decode(new Uint8Array(encodeParams));
    // // const reqString = JSON.stringify(req);

    // console.log('encode', encodeParams);
    // console.log('param', paramsString);

    // if (!nfid) return alert('NFID is not initialized');

    // await nfid
    //   ?.requestCanisterCall({
    //     canisterId: canister, // the canister id which will be called
    //     method: 'emr_list_patient', // the method on the canister which will be called
    //     parameters: '', // the parameters passed to the method on the canister
    //   })
    //   .then((ping) => {
    //     console.log('-------------');
    //     console.log('PING PATIENT REGISTRY', ping);
    //     console.log('-------------');
    //   });

    // console.log('fetching patient test...');
    // let ping = await patient_registry?.emr_list_patient(req);
    // let ping = await api.emr_list_patient(req);
    // console.log('-------------');
    // console.log('PING PATIENT REGISTRY', ping);
    // console.log('-------------');
  }
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
  };
};

export default usePatient;

interface Req {
  page: number;
  limit: number;
}

// export default canister;
