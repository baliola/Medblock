import { emrForm } from "@/constants/contents/dashboard/emr/form";
import { EMR } from "@/libs/yup/emr";
import { Flex, FormControl, FormErrorMessage, FormLabel, InputProps, Text, Textarea } from "@chakra-ui/react";
import { Field, useFormikContext } from "formik";

interface CustomInputProps extends InputProps {
  label: string;
}

const CustomInput = ({ label, name, ...props }: CustomInputProps) => {
  const { errors, touched } = useFormikContext<EMR>();
  const valueName = name as keyof EMR;

  return (
    <FormControl
      isInvalid={!!errors[valueName] && touched[valueName]}
    >
      <FormLabel as={Text} fontWeight={'bold'}>
        {label}
      </FormLabel>
      <Field as={Textarea}
        bg={'primary.200'}
        minH={'200px'}
        rounded={'lg'}
        p={5}
        name={name}
        fontSize="sm"
        {...props}
      />
      <FormErrorMessage>{errors[valueName]}</FormErrorMessage>
    </FormControl>
  )
}

export default function EMRFormReport() {
  const {
    additional_exam,
    primary_diagnosis,
    secondary_diagnosis,
    surgery,
    procedures_and_therapies
  } = emrForm.reports;
  
  return (
    <Flex direction={'column'} gap={8}>
      <CustomInput label={additional_exam.label} name={additional_exam.name} placeholder={additional_exam.placeholder} />
      <CustomInput label={primary_diagnosis.label} name={primary_diagnosis.name} placeholder={primary_diagnosis.placeholder} />
      <CustomInput label={secondary_diagnosis.label} name={secondary_diagnosis.name} placeholder={secondary_diagnosis.placeholder} />
      <CustomInput label={surgery.label} name={surgery.name} placeholder={surgery.placeholder} />
      <CustomInput label={procedures_and_therapies.label} name={procedures_and_therapies.name} placeholder={procedures_and_therapies.placeholder} />
    </Flex>
  )
}