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
import EMRFormVitalSign from "../form/vital-sign";
import EMRFormInfo from "../form/info";
import EMRFormReport from "../form/report";
import EMRFormRecipe from "../form/recipe";

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
          medical_officer: EMR?.body.find(e => e.key === 'medical_officer')?.value || "",
          room: EMR?.body.find(e => e.key === 'room')?.value || "",
          blood_pressure: EMR?.body.find(e => e.key === 'blood_pressure')?.value || "",
          temperature: EMR?.body.find(e => e.key === 'temperature')?.value || "",
          heart_rate: EMR?.body.find(e => e.key === 'heart_rate')?.value || "",
          respiration: EMR?.body.find(e => e.key === 'respiration')?.value || "",
          o2_saturation: EMR?.body.find(e => e.key === 'o2_saturation')?.value || "",
          subjective: EMR?.body.find(e => e.key === 'subjective')?.value || "",
          diagnosis: EMR?.body.find(e => e.key === 'diagnosis')?.value || "",
          planning: EMR?.body.find(e => e.key === 'planning')?.value || "",
          medication: EMR?.body.find(e => e.key === 'medication')?.value || "",
          recipe: EMR?.body.find(e => e.key === 'recipe')?.value || "",
        }}
        enableReinitialize={true}
        validationSchema={emrSchema}
        onSubmit={(values) => { onSubmit(values) }}
      >
        {({ handleSubmit, errors, touched }) => (
          <Form onSubmit={handleSubmit}>
            <Flex w={'full'} gap={5} p={10}>
              <Flex w={'full'} bg={"primary.100"} p={5} rounded={"xl"} direction={'column'}>
                {header}
                <Divider py={3} borderColor={'primary.300'} />
                <Flex direction={'column'} gap={7}>
                  <EMRFormInfo />
                  <EMRFormVitalSign />
                  <EMRFormReport />
                </Flex>
              </Flex>
              <Flex w={"lg"} direction={'column'} gap={5}>
                <EMRFormRecipe />
                <Button
                  type="submit"
                  colorScheme="primary"
                  bg={'primary.700'}
                  py={6}
                  rounded={'xl'}
                  isDisabled={
                    Object.keys(errors).length > 0 ||
                    Object.keys(touched).length === 0
                  }
                  isLoading={loadingUpdateEMR}
                >
                  {emrButton.update.label}
                </Button>
              </Flex>
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