import { useAuth } from '@/config/agent';
import { AppAgent } from '@/config/config';
import { patientCanisterId } from '@/lib/canister/patient.canister';
import { createActor } from 'declarations/patient_registry';
import {
  Consent,
  RevokeConsentRequest,
} from 'declarations/patient_registry/patient_registry.did';
import { useState } from 'react';

const useRevoke = () => {
  const [openError, setOpenError] = useState<boolean>(false);

  const { identity } = useAuth();

  const [consenst, setConsents] = useState<Consent[]>();
  const api = createActor(patientCanisterId, { agent: AppAgent(identity) });

  async function GetConsentList() {
    try {
      const response = await api.consent_list();
      console.log('-----------------');
      console.log('RESPONSE:::: CONSENT LIST', response);
      console.log('-----------------');
      setConsents(response.consents);
    } catch (error) {
      setConsents([]);
      console.log('-----------------');
      console.log('ERROR::::', error);
      console.log('-----------------');
    }
  }
  async function RevokeAccessHostpital(codes: string[]) {
    const data: RevokeConsentRequest = {
      codes: codes,
    };
    try {
      const response = await api.revoke_consent(data);
      console.log('-----------------');
      console.log('RESPONSE:::: CONSENT LIST', response);
      console.log('-----------------');
      setOpenError(true);
      //   setConsents(response.consents);
    } catch (error) {
      //   setConsents([]);
      console.log('-----------------');
      console.log('ERROR::::', error);
      console.log('-----------------');
    }
  }

  return {
    openError,
    setOpenError,
    GetConsentList,
    RevokeAccessHostpital,
    consenst,
  };
};

export default useRevoke;
