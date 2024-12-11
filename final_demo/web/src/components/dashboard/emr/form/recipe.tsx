import { EMR } from "@/libs/yup/emr";
import { Flex, FormControl, FormErrorMessage, FormLabel, Text, Textarea } from "@chakra-ui/react";
import { useFormikContext } from "formik";
import { emrForm } from "@/constants/contents/dashboard/emr/form";

export default function EMRFormRecipe() {
  const { errors, touched } = useFormikContext<EMR>();

  return (
    <Flex w={'full'} direction={'column'} bg={'primary.100'} p={5} rounded={"xl"} h={'fit-content'}>
      <FormControl isRequired
        isInvalid={!!errors.discharge_condition && touched.discharge_condition}
      >
        <FormLabel as={Text} fontWeight={'bold'}>
          {emrForm.discharge_condition.label}
        </FormLabel>
        <FormErrorMessage>{errors.discharge_condition}</FormErrorMessage>
      </FormControl>
    </Flex>
  )
}