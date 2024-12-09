import { emrForm } from "@/constants/contents/dashboard/emr/form";
import { EMR } from "@/libs/yup/emr";
import { Flex, FormControl, FormErrorMessage, FormLabel, Input, InputProps, Text } from "@chakra-ui/react";
import { Field, useFormikContext } from "formik";

const CustomInput = ({ ...props }: InputProps) => {
  return (
    <Input
      border={'2px'}
      borderColor={'primary.700'}
      py={6}
      rounded={"lg"}
      color={'primary.800'}
      fontWeight={'bold'}
      fontSize={'sm'}
      _hover={{ borderColor: 'primary.700' }}
      {...props}
    />
  )
}

export default function EMRFormInfo() {
  const { errors, touched } = useFormikContext<EMR>();
  const { info } = emrForm;
  
  return (
    <Flex direction={"column"} rowGap={3}>
      <Flex w={'full'} align={'start'} gap={5} pt={5}>
        <FormControl isRequired
          isInvalid={!!errors.visit_date && touched.visit_date}
        >
          <FormLabel as={Text} fontWeight={'bold'}>
            {info.visit_date.label}
          </FormLabel>
          <Field as={CustomInput} type="date" name="visit_date" placeholder={info.visit_date.placeholder} />
          <FormErrorMessage>{errors.visit_date}</FormErrorMessage>
        </FormControl>
        <FormControl isRequired
          isInvalid={!!errors.discharge_date && touched.discharge_date}
        >
          <FormLabel as={Text} fontWeight={'bold'}>
            {info.discharge_date.label}
          </FormLabel>
          <Field as={CustomInput} type="date" name="discharge_date" placeholder={info.discharge_date.placeholder} />
          <FormErrorMessage>{errors.discharge_date}</FormErrorMessage>
        </FormControl>
      </Flex>
      <Flex w={'full'} align={'start'} gap={5} pt={5}>
        <FormControl isRequired
          isInvalid={!!errors.medical_officer && touched.medical_officer}
        >
          <FormLabel as={Text} fontWeight={'bold'}>
            {info.medical_officer.label}
          </FormLabel>
          <Field as={CustomInput} name="medical_officer" placeholder={info.medical_officer.placeholder} />
          <FormErrorMessage>{errors.medical_officer}</FormErrorMessage>
        </FormControl>
        <FormControl isRequired
          isInvalid={!!errors.room && touched.room}
        >
          <FormLabel as={Text} fontWeight={'bold'}>
            {info.room.label}
          </FormLabel>
          <Field as={CustomInput} name="room" placeholder={info.room.placeholder} />
          <FormErrorMessage>{errors.room}</FormErrorMessage>
        </FormControl>
      </Flex>
    </Flex>
)
}