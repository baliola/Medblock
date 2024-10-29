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
    <FormControl isRequired
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
    subjective,
    diagnosis,
    planning,
    medication
  } = emrForm.reports;
  return (
    <Flex direction={'column'} gap={8}>
      <CustomInput label={subjective.label} name={subjective.name} placeholder={subjective.placeholder} />
      <CustomInput label={diagnosis.label} name={diagnosis.name} placeholder={diagnosis.placeholder} />
      <CustomInput label={planning.label} name={planning.name} placeholder={planning.placeholder} />
      <CustomInput label={medication.label} name={medication.name} placeholder={medication.placeholder} />
    </Flex>
  )
}