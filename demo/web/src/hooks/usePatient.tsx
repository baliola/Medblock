import {
  canisterId,
  createActor,
  patient_registry,
} from 'declarations/patient_registry';
import { useEffect } from 'react';

export interface EmrListPatientRequest {
  page: number;
  limit: number;
}

const usePatient = () => {
  async function fetchPatient() {
    const req: EmrListPatientRequest = {
      page: 0,
      limit: 10,
    };
    console.log('fetching patient test...');
    let ping = await patient_registry?.emr_list_patient(req);
    console.log('-------------');
    console.log('PING PATIENT REGISTRY', ping);
    console.log('-------------');
  }

  useEffect(() => {
    fetchPatient();
  }, []);

  return {
    fetchPatient,
  };
};

export default usePatient;
