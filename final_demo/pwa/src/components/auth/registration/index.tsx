"use client";

import { useRouter } from "next/navigation";
import { useState } from "react";
import { useToast } from "@chakra-ui/react";
import { Form, Formik } from "formik";

import {
  encodeHashNIK,
  usePatientMethod,
  usePatientUpdate,
} from "@/services/patients";
import {
  GetPatientInfoResponse,
  RegisterPatientRequest,
  UpdatePatientInfoRequest,
} from "@/declarations/patient_registry/patient_registry.did";
import {
  PatientRegister,
  PatientRegistrationSchema,
} from "@/libs/yup/patients-registration";

import UserRegistrationForm from "@/components/auth/registration/form";
import {
  registrationFormAction,
  registrationInitialValues,
} from "@/constants/contents/auth/registration/form";

import UserRegistrationSubmit from "./button";

export default function UserRegistration({
  initialData,
}: {
  initialData: GetPatientInfoResponse | null | undefined;
}) {
  const toast = useToast();
  const router = useRouter();

  const [file, setFile] = useState<File | null>(null);
  console.log(initialData);
  const { call: updateInitialData, loading: loadingUpdateInitialData } =
    usePatientUpdate({
      functionName: "update_patient_info",
      onSuccess() {
        router.replace("/auth/unverified/waiting");
        return;
      },
      onError(error) {
        if (
          error?.message &&
          error.message.toLowerCase().includes("userexist")
        ) {
          console.log("User already exist, updating initial data");
          return toast({
            title: "User Already Exist",
            description:
              "User already exist, please check your NIK and try again.",
            isClosable: true,
            duration: 5000,
            position: "top-right",
            status: "error",
          });
        } else {
          console.log("Error while registering patient", error);
          toast({
            title: registrationFormAction.onError.title,
            description: registrationFormAction.onError.description,
            isClosable: true,
            duration: 5000,
            position: "top-right",
            status: "error",
          });
          return;
        }
      },
    });

  const {
    call: updateExistingInitialData,
    loading: loadingUpdateExistingInitialData,
  } = usePatientUpdate({
    functionName: "update_patient_info",
    onSuccess() {
      router.replace("/auth/unverified/waiting");
      return;
    },
    onError(error) {
      if (error?.message && error.message.toLowerCase().includes("userexist")) {
        console.log("User already exist, updating initial data");
        return toast({
          title: "User Already Exist",
          description:
            "User already exist, please check your NIK and try again.",
          isClosable: true,
          duration: 5000,
          position: "top-right",
          status: "error",
        });
      } else {
        console.log("Error while registering patient", error);
        toast({
          title: registrationFormAction.onError.title,
          description: registrationFormAction.onError.description,
          isClosable: true,
          duration: 5000,
          position: "top-right",
          status: "error",
        });
        return;
      }
    },
  });

  const { call: registerPatientNIK, loading: loadingRegisterPatientNIK } =
    usePatientMethod({
      functionName: "register_patient",
      throwOnError: true,
    });

  const onRegisterToCanister = async (variables: PatientRegister) => {
    const patientNIK: RegisterPatientRequest = {
      nik: encodeHashNIK(variables.nik),
    };
    console.log("patient nik", patientNIK.nik);

    const kycDate = () => {
      const now = new Date();
      const yyyy = now.getFullYear();
      let mm = now.getMonth() + 1;
      let dd = now.getDate();
      return (
        `${dd < 10 ? `0${dd}` : dd}` +
        "/" +
        `${mm < 10 ? `0${mm}` : mm}` +
        "/" +
        yyyy
      );
    };

    const initialPatientData: UpdatePatientInfoRequest = {
      info: {
        address: variables.address,
        date_of_birth: variables.date_of_birth,
        gender: variables.gender,
        martial_status: variables.martial_status,
        name: variables.name,
        place_of_birth: variables.place_of_birth,
        kyc_date: kycDate(),
        kyc_status: { Pending: null },
      },
    };

    // @ts-ignore
    await registerPatientNIK([patientNIK])
      .then(async (data) => {
        console.log("hmm", data, initialData);
        if (initialData !== undefined && initialData !== null) {
          console.log("update");
          // @ts-ignore
          await updateExistingInitialData([initialPatientData]);
        } else {
          console.log("initial");
          // @ts-ignore
          await updateInitialData([initialPatientData]);
        }

        return;
      })
      .catch((error) => {
        console.log("Error while registering patient", error);
        toast({
          title: registrationFormAction.onError.title,
          description: registrationFormAction.onError.description,
          isClosable: true,
          duration: 5000,
          position: "top-right",
          status: "error",
        });
        return;
      });
  };

  // const { mutate: sendKYC, isPending } = useMutation({
  //   mutationKey: ["sendKYC"],
  //   mutationFn: (data: KYC) => createKYC(data),
  //   onSuccess(data, variables, context) {
  //     if (initialData) {
  //       router.replace("/auth/unverified/waiting");
  //       return;
  //     }

  //     onRegisterToCanister({
  //       nik: variables.nik,
  //       address: variables.address,
  //       date_of_birth: variables.dateBirth,
  //       gender: variables.gender,
  //       idcard_upload: true,
  //       martial_status: variables.marital,
  //       agree: true,
  //       name: variables.fullName,
  //       place_of_birth: variables.placeBirth
  //     });
  //   },
  //   onError: (error) => {
  //     console.log(error)
  //     return toast({
  //       title: "Error while Registering",
  //       description: "Error while registering patient, please try again",
  //       status: "error",
  //       isClosable: true,
  //       duration: 5000,
  //       position: "top-right"
  //     })
  //   }
  // });

  return (
    <Formik
      initialValues={
        initialData
          ? {
              nik: "",
              address: initialData?.patient.V1.address,
              name: initialData?.patient.V1.name,
              gender: initialData?.patient.V1.gender,
              place_of_birth: initialData?.patient.V1.place_of_birth,
              date_of_birth: initialData?.patient.V1.date_of_birth,
              martial_status: initialData?.patient.V1.martial_status,
              idcard_upload: false,
              agree: false,
            }
          : registrationInitialValues
      }
      enableReinitialize={true}
      validationSchema={PatientRegistrationSchema}
      onSubmit={async (values) => {
        // console.log(values)
        if (!file) {
          return toast({
            title: "ID Card Required",
            description: "Please upload the required identification card.",
            status: "error",
            isClosable: true,
            duration: 5000,
            position: "top-right",
          });
        }

        const data: PatientRegister = {
          nik: initialData?.nik || encodeHashNIK(values.nik),
          name: values.name,
          address: values.address,
          date_of_birth: values.date_of_birth,
          gender: values.gender,
          place_of_birth: values.place_of_birth,
          martial_status: values.martial_status,
          agree: true,
          idcard_upload: true,
        };

        onRegisterToCanister(data);
      }}
    >
      {({ errors, isValid, dirty, handleSubmit }) => (
        <Form onSubmit={handleSubmit}>
          <UserRegistrationForm file={file} setFile={setFile} />
          <UserRegistrationSubmit
            loading={
              loadingRegisterPatientNIK ||
              loadingUpdateInitialData ||
              loadingUpdateExistingInitialData
            }
            disabled={!isValid || !dirty || Object.keys(errors).length > 0}
          />
        </Form>
      )}
    </Formik>
  );
}
