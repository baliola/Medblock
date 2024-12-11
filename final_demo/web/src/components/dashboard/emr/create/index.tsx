"use client"
import { useSearchParams } from "next/navigation";

import { Flex, useDisclosure, useToast } from "@chakra-ui/react";
import { Form, Formik } from "formik";

import { EMR, emrSchema } from "@/libs/yup/emr";
import { ProviderActor, useProviderQuery } from "@/services/providers";
import { EmrFragment } from "@/declarations/emr_registry/emr_registry.did";
import { IssueEmrRequest } from "@/declarations/provider_registry/provider_registry.did";

import EMRCreateSuccess from "./success";
import { Fragment } from "react";
import { providerCanisterId } from "@/config/canisters/providers.canister";
import { emrButton } from "@/constants/contents/dashboard/emr/button";
import EMRFormContent from "../form";

const initialValues = {
  visit_date: "",
  discharge_date: "",
  visit_time: "",
  discharge_time: "",
  medical_officer: "",
  room: "",

  blood_pressure: "",
  temperature: "",
  heart_rate: "",
  respiration: "",
  o2_saturation: "",

  circuit_reason: "",
  illness_history: "",

  pyhsical_exam: "",
  drug_allergy: "No",
  food_allergy: "No",
  other_allergy: "",

  additional_exam: "",
  primary_diagnosis: "",
  secondary_diagnosis: "",
  surgery: "",
  procedures_and_therapies: "",

  recipe: "",
  discharge_condition: "",
}

const mapValuesToEmrFragments = (values: EMR): EmrFragment[] =>
  Object.entries(values).map(([key, value]) => ({
    key, value: String(value)
  }));

const EMRForm = ({ header }: { header: React.ReactNode }) => {
  const toast = useToast();
  const params = useSearchParams();
  const user = params.get('user') || null;

  const { isOpen, onOpen } = useDisclosure();

  const {
    call: createEMR,
    loading: loadingCreateEMR
  } = useProviderQuery({
    functionName: "issue_emr",
    refetchOnMount: false,
    onSuccess(data) {
      console.log(data)
    },
    onError(err) {
      if (err instanceof Error) {
        toast({
          title: "Error!",
          description: err.message,
          status: "error",
          duration: 5000,
          isClosable: true,
          position: "top-right"
        });
      } else {
        toast({
          title: "Error!",
          description: "Something went wrong!",
          isClosable: true,
          duration: 5000,
          position: "top-right",
          status: "error"
        })
      }
      
      throw err
    }
  })

  const onSubmit = async (values: EMR, resetForm: () => void) => {
    if (!user) return;

    const emrFragments = mapValuesToEmrFragments(values);

    const request: IssueEmrRequest = {
      emr: emrFragments,
      user_id: user,
    };

    try {
      // @ts-expect-error
      await createEMR([request]);

      resetForm();
      onOpen();
    } catch (error) {
      console.error(error);
    }
  }

  return (
    <Fragment>
      <EMRCreateSuccess isOpen={isOpen} />
      <Formik
        initialValues={initialValues}
        validationSchema={emrSchema}
        validateOnChange={true}
        validateOnBlur={false}
        onSubmit={(values, { resetForm }) => {
          onSubmit(values, resetForm)
        }}
      >
        {({ handleSubmit, errors, touched, setFieldValue, values }) => (
          <Form onSubmit={handleSubmit}>
            <Flex w={'full'} gap={5} p={10}
              direction={{ base: 'column', lg: 'row' }}
            >
              <EMRFormContent props={{ 
                header, 
                loading: loadingCreateEMR, 
                label: emrButton.create.label, 
                drugAllergyValue: values.drug_allergy,
                foodAllergyValue: values.food_allergy,
                dischargeValue: values.discharge_condition,
                setFieldValue 
              }} />
            </Flex>
          </Form>
        )}
      </Formik>
    </Fragment>
  )
}

export default function EMRCreateForm({ header }: { header: React.ReactNode }) {
  return (
    <ProviderActor
      canisterId={providerCanisterId}
    >
      <EMRForm header={header} />
    </ProviderActor>
  )
}