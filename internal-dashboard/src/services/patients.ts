import {   
  canisterId,
  idlFactory,
  patient_registry 
} from "@/canister/declarations/patient_registry";
import { createActorContext } from "@ic-reactor/react";

type Actor = typeof patient_registry;

export const {
  ActorProvider: PatientActor,
  useQueryCall: usePatientQuery,
  useUpdateCall: usePatientUpdate,
  useMethod: usePatientMethod,
} = createActorContext<Actor>({
  canisterId,
  idlFactory: idlFactory,
});