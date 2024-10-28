import { registrationFormButton, registrationFormTerms } from "@/constants/contents/auth/registration/form";
import { PatientRegister } from "@/libs/yup/patients-registration";
import { Button, Checkbox, FormControl, FormErrorMessage, Stack, Text } from "@chakra-ui/react";
import { Field, FieldInputProps, useFormikContext } from "formik";

interface UserRegistrationSubmitProps {
  loading: boolean;
  disabled: boolean;
}

export default function UserRegistrationSubmit({
  loading, disabled
}: UserRegistrationSubmitProps) {
  const { errors, touched, setFieldValue } = useFormikContext<PatientRegister>();

  return (
    <Stack spacing={4} pt={8}>
      <FormControl isRequired isInvalid={!!errors.agree && touched.agree}>
        <Field name="agree">
          {({ field }: { field: FieldInputProps<boolean> }) => (
            <Checkbox
              isChecked={field.value}
              onChange={(e) => setFieldValue("agree", e.target.checked)}
              colorScheme="blue"
              borderColor={"blue.200"}
              rounded={"xl"}
              spacing={4}
              onBlur={field.onBlur}
            >
              <Text as="span" fontSize={'sm'} textDecoration={'underline'}>
                {registrationFormTerms.label}
              </Text>
            </Checkbox>
          )}
        </Field>
        <FormErrorMessage>{errors.agree}</FormErrorMessage>
      </FormControl>

      <Button type="submit"
        colorScheme="primary"
        bg={"primary.700"}
        fontSize={'sm'}
        w={'full'}
        py={6}
        rounded={"xl"}
        isLoading={loading}
        isDisabled={disabled}
      >
        {registrationFormButton.label}
      </Button>
    </Stack>
  );
}
