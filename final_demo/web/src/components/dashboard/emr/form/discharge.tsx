import { EMR } from "@/libs/yup/emr";
import { Flex, FormControl, FormErrorMessage, FormLabel, InputProps, Radio, RadioGroup, Stack, Text } from "@chakra-ui/react";
import { Field, FormikErrors, useFormikContext } from "formik";
import { emrForm } from "@/constants/contents/dashboard/emr/form";

interface CustomRadioInputProps extends InputProps {
  name: string
  value: string
  label: string;
  options: string[];
  setFieldValue: (field: string, value: any, shouldValidate?: boolean) => Promise<void | FormikErrors<{}>>
}

const CustomRadioInput = ({ label, name, value, options, setFieldValue, ...props }: CustomRadioInputProps) => {
  return (
    <>
      <FormLabel as="legend" fontSize="sm">{label}</FormLabel>
      <Field name={name}>
        {({ field }: { field: { value: string } }) => (
          <RadioGroup
            {...field}
            value={value}
            onChange={(value) => setFieldValue(name, value)}
          >
            <Stack direction="row" columnGap={4} display={"flex"} flexWrap={"wrap"}>
              {
                options.map((option, index) =>
                  <Radio key={index} value={option} borderColor={"black"}>
                    <Text fontSize="sm">{option}</Text>
                  </Radio>
                )
              }
            </Stack>
          </RadioGroup>
        )}
      </Field>
    </>
  )
}

interface IEMRFormDischargeProps {
  value: string
  setFieldValue: (field: string, value: any, shouldValidate?: boolean) => Promise<void | FormikErrors<{}>>
}


export default function EMRFormDischarge({ props }: { props: IEMRFormDischargeProps }) {
  const { value, setFieldValue } = props
  const { errors, touched } = useFormikContext<EMR>();

  return (
    <Flex w={'full'} direction={'column'} bg={'primary.100'} p={5} rounded={"xl"} h={'fit-content'}>
      <FormControl
        isInvalid={!!errors.discharge_condition && touched.discharge_condition}
      >
        <CustomRadioInput 
          label={emrForm.discharge_condition.label}
          name={emrForm.discharge_condition.name}
          value={value}
          setFieldValue={setFieldValue}
          options={[
            'Healed',
            'Moved to another Hospital',
            'Going better',
            'Patient chooses to leave the hospital before the treating physician recommends to discharge',
            'Died more than 48 hours',
            'Died less than 48 hours'
          ]}
        />
        <FormErrorMessage fontSize={'xs'} mt={3}>{errors.discharge_condition}</FormErrorMessage>
      </FormControl>
    </Flex>
  )
}