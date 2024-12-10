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
    <Flex direction={"column"} rowGap={4} pt={5}>
      <Flex w={'full'} align={'start'} gap={5}>
        <FormControl
          isInvalid={!!errors.visit_date && touched.visit_date}
        >
          <FormLabel as={Text} fontWeight={'bold'}>
            {info.visit_date.label}
          </FormLabel>
          <Field as={CustomInput} type="date" name={info.visit_date.name} placeholder={info.visit_date.placeholder} />
          <FormErrorMessage>{errors.visit_date}</FormErrorMessage>
        </FormControl>
        <FormControl
          isInvalid={!!errors.discharge_date && touched.discharge_date}
        >
          <FormLabel as={Text} fontWeight={'bold'}>
            {info.discharge_date.label}
          </FormLabel>
          <Field as={CustomInput} type="date" name={info.discharge_date.name} placeholder={info.discharge_date.placeholder} />
          <FormErrorMessage>{errors.discharge_date}</FormErrorMessage>
        </FormControl>
      </Flex>
      <Flex w={'full'} align={'start'} gap={5}>
        <FormControl
          isInvalid={!!errors.visit_time && touched.visit_time}
        >
          <FormLabel as={Text} fontWeight={'bold'}>{info.visit_time.label}</FormLabel>
          <Field as={CustomInput} type="time" name={info.visit_time.label} placeholder={info.visit_time.placeholder} />
          <FormErrorMessage>{errors.visit_time}</FormErrorMessage>
        </FormControl>
        <FormControl
          isInvalid={!!errors.discharge_time && touched.discharge_time}
        >
          <FormLabel as={Text} fontWeight={'bold'}>{info.discharge_time.label}</FormLabel>
          <Field as={CustomInput} type="time" name={info.discharge_time.label} placeholder={info.discharge_time.placeholder} />
          <FormErrorMessage>{errors.discharge_time}</FormErrorMessage>
        </FormControl>
      </Flex>
      <Flex w={'full'} align={'start'} gap={5}>
        <FormControl
          isInvalid={!!errors.medical_officer && touched.medical_officer}
        >
          <FormLabel as={Text} fontWeight={'bold'}>
            {info.medical_officer.label}
          </FormLabel>
          <Field as={CustomInput} name="medical_officer" placeholder={info.medical_officer.placeholder} />
          <FormErrorMessage>{errors.medical_officer}</FormErrorMessage>
        </FormControl>
        <FormControl
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