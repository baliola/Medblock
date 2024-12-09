"use client"
import { useSearchParams } from "next/navigation";

import { Button, Divider, Flex, useDisclosure } from "@chakra-ui/react";
import { Form, Formik } from "formik";

import { EMR, emrSchema } from "@/libs/yup/emr";
import { ProviderActor, useProviderQuery } from "@/services/providers";
import { EmrFragment } from "@/declarations/emr_registry/emr_registry.did";
import { IssueEmrRequest } from "@/declarations/provider_registry/provider_registry.did";

import EMRFormInfo from "@/components/dashboard/emr/form/info";
import EMRFormVitalSign from "@/components/dashboard/emr/form/vital-sign";
import EMRFormReport from "@/components/dashboard/emr/form/report";
import EMRCreateSuccess from "./success";
import { Fragment } from "react";
import { providerCanisterId } from "@/config/canisters/providers.canister";
import { emrButton } from "@/constants/contents/dashboard/emr/button";
import EMRFormRecipe from "../form/recipe";

const initialValues = {
  visit_date: "",
  discharge_date: "",
  medical_officer: "",
  room: "",
  
  blood_pressure: "",
  temperature: "",
  heart_rate: "",
  respiration: "",
  o2_saturation: "",
  subjective: "",
  diagnosis: "",
  planning: "",
  medication: "",
  recipe: "",
}

const mapValuesToEmrFragments = (values: EMR): EmrFragment[] =>
  Object.entries(values).map(([key, value]) => ({
    key, value: String(value)
  }));

const EMRForm = ({ header }: { header: React.ReactNode }) => {
  const params = useSearchParams();
  const user = params.get('user') || null;

  const { isOpen, onOpen } = useDisclosure();

  const {
    call: createEMR,
    loading: loadingCreateEMR
  } = useProviderQuery({
    functionName: "issue_emr",
    refetchOnMount: false,
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
        onSubmit={(values, { resetForm }) => {
          onSubmit(values, resetForm)
        }}
      >
        {({ handleSubmit, errors, touched }) => (
          <Form onSubmit={handleSubmit}>
            <Flex w={'full'} gap={5} p={10}
              direction={{ base: 'column', lg: 'row' }}
            >
              <Flex w={'full'} bg={"primary.100"} p={5} rounded={"xl"} direction={'column'}>
                {header}
                <Divider py={3} borderColor={'primary.300'} />
                <Flex direction={'column'} gap={7}>
                  <EMRFormInfo />
                  <EMRFormVitalSign />
                  <EMRFormReport />
                </Flex>
              </Flex>
              <Flex w={{ lg: "lg" }} direction={'column'} gap={5}>
                <EMRFormRecipe />
                <Button
                  type="submit"
                  colorScheme="primary"
                  bg={'primary.700'}
                  py={5}
                  fontSize={'xs'}
                  rounded={'lg'}
                  isDisabled={
                    Object.keys(errors).length > 0 ||
                    Object.keys(touched).length === 0
                  }
                  isLoading={loadingCreateEMR}
                >
                  {emrButton.create.label}
                </Button>
              </Flex>
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