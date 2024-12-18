import { emrForm } from "@/constants/contents/dashboard/emr/form";
import { EMR } from "@/libs/yup/emr";
import { Box, Flex, FormControl, FormErrorMessage, FormLabel, Grid, Input, InputProps, Radio, RadioGroup, Stack, Text, Textarea } from "@chakra-ui/react";
import { Field, FormikErrors, useFormikContext } from "formik";

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

const CustomInputText = ({ ...props }: InputProps) => {
  return (
    <Input
      border={"none"}
      rounded={"lg"}
      py={0}
      color={'primary.800'}
      fontSize="sm"
      {...props}
      sx={{
        "&:focus": {
          border: "none",
          boxShadow: "none",
        },
      }}
    />
  )
}

interface CustomRadioInputProps extends InputProps {
  name: string
  value: string
  label: string;
  setFieldValue: (field: string, value: any, shouldValidate?: boolean) => Promise<void | FormikErrors<{}>>
}

const CustomRadioInput = ({ label, name, value, setFieldValue, ...props }: CustomRadioInputProps) => {
  return (
    <FormControl as="fieldset">
      <FormLabel as="legend" fontSize="sm">{label}</FormLabel>
      <Field name={name}>
        {({ field }: { field: { value: string } }) => (
          <RadioGroup
            {...field}
            value={value}
            onChange={(value) => setFieldValue(name, value)}
          >
            <Stack direction="row" columnGap={4}>
              <Radio value="Yes" borderColor={"black"}>
                <Text fontSize="sm">Yes</Text>
              </Radio>
              <Radio value="No" borderColor={"black"}>
                <Text fontSize="sm">No</Text>
              </Radio>
            </Stack>
          </RadioGroup>
        )}
      </Field>
    </FormControl>
  )
}

interface IEMRFormHistoryProps {
  drugAllergyValue: string
  foodAllergyValue: string
  setFieldValue: (field: string, value: any, shouldValidate?: boolean) => Promise<void | FormikErrors<{}>>
}

export default function EMRFormHistory({ props }: { props: IEMRFormHistoryProps }) {
  const { drugAllergyValue, foodAllergyValue, setFieldValue } = props
  const { errors, touched } = useFormikContext<EMR>();

  const {
    circuit_reason,
    illness_history,
    pyhsical_exam
  } = emrForm.history;

  const {
    drug_allergy,
    food_allergy,
    other_allergy
  } = emrForm.history_of_allergy
  
  return (
    <Grid templateColumns="repeat(2, 1fr)" gap={6} pt={5}>
      <Flex direction={"column"} w={'full'} align={'start'} gap={5}>
        <CustomInput label={circuit_reason.label} name={circuit_reason.name} placeholder={circuit_reason.placeholder} />
        <CustomInput label={illness_history.label} name={illness_history.name} placeholder={illness_history.placeholder} />
      </Flex>
      <Flex direction={"column"} w={'full'} align={'start'} gap={5}>
        <CustomInput label={pyhsical_exam.label} name={pyhsical_exam.name} placeholder={pyhsical_exam.placeholder} />
        <Flex flexDirection={"column"} w={'full'}>
          <FormLabel as={Text} fontWeight={'bold'}>
            {'History of Allergy'}
          </FormLabel>
          <Flex
            bg={'primary.200'}
            minH={'200px'}
            rounded={'lg'}
            p={5}
            fontSize="sm"
            direction={"column"}
            justifyContent={"space-between"}
          >
            <Grid templateColumns="repeat(2, 1fr)" gap={6}>
              <CustomRadioInput name={drug_allergy.name} label={drug_allergy.label} value={drugAllergyValue} setFieldValue={setFieldValue} />
              <CustomRadioInput name={food_allergy.name} label={food_allergy.label} value={foodAllergyValue} setFieldValue={setFieldValue} />
            </Grid>
            <FormControl
              isInvalid={!!errors.other_allergy && touched.other_allergy}
            >
              <FormLabel as={Text} fontSize="sm" mb={0}>
                {other_allergy.label}
              </FormLabel>
              <Field as={CustomInputText} name="other_allergy" placeholder={other_allergy.placeholder} />
              <FormErrorMessage>{errors.other_allergy}</FormErrorMessage>
            </FormControl>
          </Flex>
        </Flex>
      </Flex>
    </Grid>
  )
}