"use client"

import { useParams, useSearchParams } from "next/navigation";
import { Fragment, useEffect, useState } from "react";
import { Button, Divider, Flex, FormControl, FormErrorMessage, FormLabel, Text, Textarea, useDisclosure } from "@chakra-ui/react";
import { Field, Form, Formik } from "formik";

import { PatientActor, usePatientQuery } from "@/services/patients";
import { ProviderActor, useProviderUpdate } from "@/services/providers";
import { Principal } from "@dfinity/principal";
import { EMR, emrSchema } from "@/libs/yup/emr";
import { EmrFragment, EmrHeaderWithBody, ReadEmrByIdResponse, ReadEmrSessionRequest } from "@/declarations/patient_registry/patient_registry.did";
import { UpdateEmrRequest } from "@/declarations/provider_registry/provider_registry.did";

import EMRUpdateSuccess from "./success";
import { providerCanisterId } from "@/config/canisters/providers.canister";
import { patientCanisterId } from "@/config/canisters/patient.canister";
import { emrButton } from "@/constants/contents/dashboard/emr/button";
import EMRFormContent from "../form";

const mapValuesToEmrFragments = (values: EMR): EmrFragment[] =>
  Object.entries(values).map(([key, value]) => ({
    key, value: String(value)
  }));

const EMRForm = ({ header }: { header: React.ReactNode }) => {
  const { isOpen, onOpen } = useDisclosure();

  const params = useSearchParams();
  const param = useParams();

  const [EMR, setEMR] = useState<EmrHeaderWithBody>();

  const { record, id } = param;

  const providerId = params.get('provider') || null;
  const registry = params.get('registry') || null;
  const user = params.get('user') || null;

  const {
    call: getDetailEMR,
    loading: loadingDetailEMR
  } = usePatientQuery({
    functionName: "read_emr_with_session",
    refetchOnMount: false,
  });

  const {
    call: updateEMR,
    loading: loadingUpdateEMR
  } = useProviderUpdate({
    functionName: "update_emr",
  });

  const fetchEmrDetails = async () => {
    if (record && providerId && registry) {
      const request: ReadEmrSessionRequest = {
        session_id: id as string,
        args: {
          emr_id: record as string,
          provider_id: providerId as string,
          registry_id: Principal.fromText(registry as string),
        },
      };

      try {
        // @ts-expect-error
        const data: ReadEmrByIdResponse = await getDetailEMR([request]);
        setEMR(data.emr);
      } catch (error) {
        console.error("Failed to fetch EMR details", error);
      }
    }
  };

  const onSubmit = async (values: EMR) => {
    if (!user) return;

    const emrFragments = mapValuesToEmrFragments(values);

    const request: UpdateEmrRequest = {
      fields: emrFragments,
      header: {
        emr_id: record as string,
        provider_id: providerId as string,
        registry_id: Principal.fromText(registry as string),
        user_id: user
      }
    };

    try {
      // @ts-expect-error
      await updateEMR([request]);
      onOpen();
    } catch (error) {
      console.log(error)
    }
  }

  useEffect(() => {
    fetchEmrDetails();

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [record, providerId, registry]);

  if (loadingDetailEMR) return <div>Loading...</div>

  return (
    <Fragment>
      <EMRUpdateSuccess isOpen={isOpen} />
      <Formik
        initialValues={{
          visit_date: EMR?.body.find(e => e.key === 'visit_date')?.value || "",
          discharge_date: EMR?.body.find(e => e.key === 'discharge_date')?.value || "",
          visit_time: EMR?.body.find(e => e.key === 'visit_time')?.value || "",
          discharge_time: EMR?.body.find(e => e.key === 'discharge_time')?.value || "",
          medical_officer: EMR?.body.find(e => e.key === 'medical_officer')?.value || "",
          room: EMR?.body.find(e => e.key === 'room')?.value || "",

          blood_pressure: EMR?.body.find(e => e.key === 'blood_pressure')?.value || "",
          temperature: EMR?.body.find(e => e.key === 'temperature')?.value || "",
          heart_rate: EMR?.body.find(e => e.key === 'heart_rate')?.value || "",
          respiration: EMR?.body.find(e => e.key === 'respiration')?.value || "",
          o2_saturation: EMR?.body.find(e => e.key === 'o2_saturation')?.value || "",

          circuit_reason: EMR?.body.find(e => e.key === 'circuit_reason')?.value || "",
          illness_history: EMR?.body.find(e => e.key === 'illness_history')?.value || "",

          pyhsical_exam: EMR?.body.find(e => e.key === 'pyhsical_exam')?.value || "",
          drug_allergy: EMR?.body.find(e => e.key === 'drug_allergy')?.value || "No",
          food_allergy: EMR?.body.find(e => e.key === 'food_allergy')?.value || "No",
          other_allergy: EMR?.body.find(e => e.key === 'other_allergy')?.value || "",

          additional_exam: EMR?.body.find(e => e.key === 'additional_exam')?.value || "",
          primary_diagnosis: EMR?.body.find(e => e.key === 'primary_diagnosis')?.value || "",
          secondary_diagnosis: EMR?.body.find(e => e.key === 'secondary_diagnosis')?.value || "",
          surgery: EMR?.body.find(e => e.key === 'surgery')?.value || "",
          procedures_and_therapies: EMR?.body.find(e => e.key === 'procedures_and_therapies')?.value || "",

          recipe: EMR?.body.find(e => e.key === 'recipe')?.value || "",
          discharge_condition: EMR?.body.find(e => e.key === 'discharge_condition')?.value || "",
        }}
        enableReinitialize={true}
        validateOnChange={true}
        validationSchema={emrSchema}
        validateOnBlur={false}
        onSubmit={(values) => { onSubmit(values) }}
      >
        {({ handleSubmit, errors, touched, values, setFieldValue }) => (
          <Form onSubmit={handleSubmit}>
            <Flex w={'full'} gap={5} p={10}>
              <EMRFormContent props={{ 
                header, 
                loading: loadingUpdateEMR, 
                label: emrButton.update.label,
                drugAllergyValue: values.drug_allergy,
                foodAllergyValue: values.food_allergy,
                dischargeValue: values.discharge_condition,
                setFieldValue
              }}/>
            </Flex>
          </Form>
        )}
      </Formik>
    </Fragment>
  )
}

export default function EMRUpdateForm({ header }: { header: React.ReactNode }) {
  return (
    <ProviderActor
      canisterId={providerCanisterId}
    >
      <PatientActor canisterId={patientCanisterId}>
        <EMRForm header={header} />
      </PatientActor>
    </ProviderActor>
  )
}