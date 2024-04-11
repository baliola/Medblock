import { canisterId, patient_registry } from 'declarations/patient_registry';
import { useEffect, useState } from 'react';
import useAuth from './useAuth';
import {
  patientCanisterId,
  patientCanisterIdMainnet,
} from '@/lib/canister/patient.canister';
import { Cbor, HttpAgent } from '@dfinity/agent';
import { AuthClient } from '@dfinity/auth-client';
import { useRouter } from 'next/router';
import { useCentralStore } from '@/Store';
import { providerCanisterIdMainnet } from '@/lib/canister/provider.canister';
import { createActor } from 'declarations/provider_registry';
import { NFID } from '@nfid/embed';
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
  const { agent } = useCentralStore();
  const [patientList, setPatientList] = useState([]);

  const canister = patientCanisterIdMainnet;
  async function fetchPatient() {
    const api = createActor(providerCanisterIdMainnet, { agent });
    console.log('FETCH PATIENT RUNNING.....');
    const request = {
      page: BigInt(0),
      limit: 10,
    };
    try {
      const response = await api?.emr_list_provider(request);
      console.log('-----------------');
      console.log('RESPONSE::::', response);
      console.log('-----------------');
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

  useEffect(() => {
    console.log('AGENT NIH BRO', agent);
    if (agent) fetchPatient();
  }, []);

  // useEffect(() => {}, [delegation, identity]);

  return {
    fetchPatient,
  };
};

export default usePatient;

interface Req {
  page: number;
  limit: number;
}

// export default canister;
