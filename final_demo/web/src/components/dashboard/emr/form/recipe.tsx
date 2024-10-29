import { EMR } from "@/libs/yup/emr";
import { Flex, FormControl, FormErrorMessage, FormLabel, Text, Textarea } from "@chakra-ui/react";
import { Field, useFormikContext } from "formik";

import { emrForm } from "@/constants/contents/dashboard/emr/form";

export default function EMRFormRecipe() {
  const { errors, touched } = useFormikContext<EMR>();

  return (
    <Flex w={'full'} direction={'column'} bg={'primary.100'} p={5} rounded={"xl"} h={'fit-content'}>
      <FormControl isRequired
        isInvalid={!!errors.recipe && touched.recipe}
      >
        <FormLabel as={Text} fontWeight={'bold'}>
          {emrForm.recipe.label}
        </FormLabel>
        <Field as={Textarea}
          bg={'primary.200'}
          minH={'200px'}
          rounded={'xl'}
          p={5}
          fontSize="sm"
          name={emrForm.recipe.name}
          placeholder={emrForm.recipe.placeholder}
        />
        <FormErrorMessage>{errors.recipe}</FormErrorMessage>
      </FormControl>
    </Flex>
  )
}