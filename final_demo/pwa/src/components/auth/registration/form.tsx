"use client"

import { Flex, FormControl, FormErrorMessage } from "@chakra-ui/react";
import { Field, useFormikContext } from "formik";

import { registrationForm } from "@/constants/contents/auth/registration/form";
import { PatientRegister } from "@/libs/yup/patients-registration";

import IDCardInput from "@/components/input/id-card";
import FormCLabel from "@/components/input/label";
import CSelect from "@/components/input/select";
import CInput from "@/components/input/text";
import CTextArea from "@/components/input/textarea";

interface UserRegistrationFormProps {
  file: File | null;
  setFile: (file: File | null) => void;
}

export default function UserRegistrationForm({
  file, setFile
}: UserRegistrationFormProps) {
  const {
    errors,
    touched,
  } = useFormikContext<PatientRegister>();

  const ageMinimumInYears = 17;
  const currentYear = new Date().getFullYear();

  const maxSelectableDate = new Date(
    currentYear - ageMinimumInYears,
    new Date().getMonth(),
    new Date().getDate()
  ).toISOString().split('T')[0];

  return (
    <Flex
      w={'full'}
      direction={"column"}
      gap={4}
    >
      <FormControl isRequired
        isInvalid={!!errors.name && touched.name}
      >
        <FormCLabel>
          {registrationForm.name.label}
        </FormCLabel>
        <Field
          as={CInput}
          name="name"
          placeholder={registrationForm.name.placeholder}
        />
        <FormErrorMessage>{errors.name}</FormErrorMessage>
      </FormControl>

      <FormControl isRequired
        isInvalid={!!errors.nik && touched.nik}
      >
        <FormCLabel>
          {registrationForm.nik.label}
        </FormCLabel>
        <Field
          as={CInput}
          type="number"
          name="nik"
          maxLength={16}
          placeholder={registrationForm.nik.placeholder}
        />
        <FormErrorMessage>{errors.nik}</FormErrorMessage>
      </FormControl>

      <FormControl isRequired
        isInvalid={!!errors.address && touched.address}
      >
        <FormCLabel>
          {registrationForm.address.label}
        </FormCLabel>
        <Field
          as={CTextArea}
          name="address"
          placeholder={registrationForm.address.placeholder}
        />
        <FormErrorMessage>{errors.address}</FormErrorMessage>
      </FormControl>

      <FormControl isRequired
        isInvalid={!!errors.gender && touched.gender}
      >
        <FormCLabel>
          {registrationForm.gender.label}
        </FormCLabel>
        <Field
          as={CSelect}
          name="gender"
          placeholder={registrationForm.gender.placeholder}
        >
          {registrationForm.gender.options.map((item, index) => (
            <option key={index} value={item.value}>
              {item.label}
            </option>
          ))}
        </Field>
        <FormErrorMessage>{errors.gender}</FormErrorMessage>
      </FormControl>

      <FormControl isRequired
        isInvalid={!!errors.place_of_birth && touched.place_of_birth}
      >
        <FormCLabel>
          {registrationForm.place_of_birth.label}
        </FormCLabel>
        <Field
          as={CInput}
          name="place_of_birth"
          placeholder={registrationForm.place_of_birth.placeholder}
        />
        <FormErrorMessage>{errors.place_of_birth}</FormErrorMessage>
      </FormControl>

      <FormControl isRequired
        isInvalid={!!errors.date_of_birth && touched.date_of_birth}
      >
        <FormCLabel>
          {registrationForm.date_of_birth.label}
        </FormCLabel>
        <Field
          as={CInput}
          type="date"
          name="date_of_birth"
          /**
           * Enable in case need validation
           */
          // max={maxSelectableDate}
          placeholder={registrationForm.date_of_birth.placeholder}
        />
        <FormErrorMessage>{errors.date_of_birth}</FormErrorMessage>
      </FormControl>

      <FormControl isRequired
        isInvalid={!!errors.martial_status && touched.martial_status}
      >
        <FormCLabel>
          {registrationForm.martial_status.label}
        </FormCLabel>
        <Field
          as={CSelect}
          name="martial_status"
          placeholder={registrationForm.martial_status.placeholder}
        >
          {registrationForm.martial_status.options.map((item, index) => (
            <option key={index} value={item.value}>
              {item.label}
            </option>
          ))}
        </Field>
        <FormErrorMessage>{errors.martial_status}</FormErrorMessage>
      </FormControl>

      <FormControl
        isRequired
        isInvalid={!!errors.idcard_upload && touched.idcard_upload}
      >
        <IDCardInput file={file} setFile={setFile} />
        <FormErrorMessage>
          {errors.idcard_upload}
        </FormErrorMessage>
      </FormControl>
    </Flex>
  )
}