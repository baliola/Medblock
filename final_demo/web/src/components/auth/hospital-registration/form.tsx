"use client"

import { Button, Flex, FormControl, FormErrorMessage, FormLabel, Input, InputProps, useToast } from '@chakra-ui/react';
import { Field, Form, Formik } from 'formik';
import { HospitalRegistrationSchema } from '@/libs/yup/hospital-registration-schema';
import { useProviderMethod } from '@/services/providers';
import { RegisternewProviderRequest } from '@/declarations/provider_registry/provider_registry.did';
import { useUserPrincipal } from '@ic-reactor/react';
import { Principal } from '@dfinity/principal';
import { useRouter } from 'next/navigation';
import { hospitalRegistrationForm } from '@/constants/contents/auth/hospital-registration/form';

const CustomInput = ({ ...props }: InputProps) => {
  return (
    <Input
      bg={"#E7E7E7"}
      w={"full"}
      rounded={"2xl"}
      py={6}
      color={"neutral.600"}
      {...props}
    />
  )
}

export default function FormHospitalRegisration() {
  const principal = useUserPrincipal();
  const toast = useToast();
  const router = useRouter();

  const { contents, button, success, error } = hospitalRegistrationForm;

  const {
    call: registerHospital,
    loading: registerHospitalLoading
  } = useProviderMethod({
    functionName: "register_new_provider",
    refetchOnMount: false,
    onSuccess() {
      toast({
        title: success.title,
        description: success.description,
        status: "success"
      });

      router.replace(success.redirect);

      return;
    },
    onError(err) {
      console.log(err);
      toast({
        title: error.title,
        description: error.description,
        status: "error"
      });

      return;
    },
  });

  const onRegisterHospital = async (values: { name: string; address: string }) => {
    const data: RegisternewProviderRequest = {
      address: values.address,
      display_name: values.name,
      provider_principal: principal as Principal,
    };

    // @ts-expect-error
    await registerHospital([data]);
  }

  return (
    <Formik
      initialValues={{
        name: '',
        address: ''
      }}
      validationSchema={HospitalRegistrationSchema}
      onSubmit={(values) => onRegisterHospital(values)}
    >
      {({ errors, touched, handleSubmit }) => (
        <Form onSubmit={handleSubmit}>
          <Flex direction={"column"} w={"sm"} gap={5}>
            <FormControl
              isRequired
              isInvalid={!!errors.name && touched.name}
              gap={0}
            >
              <FormLabel color={"neutral.700"} fontSize={'sm'}>
                {contents.hospital_name.label}
              </FormLabel>
              <Field as={CustomInput} name="name" placeholder={contents.hospital_name.placeholder} />
              <FormErrorMessage>{errors.name}</FormErrorMessage>
            </FormControl>
            <FormControl
              isRequired
              isInvalid={!!errors.address && touched.address}
              gap={0}
            >
              <FormLabel color={"neutral.700"} fontSize={'sm'}>
                {contents.hospital_address.label}
              </FormLabel>
              <Field as={CustomInput} name="address" placeholder={contents.hospital_address.placeholder} />
              <FormErrorMessage>{errors.address}</FormErrorMessage>
            </FormControl>
          </Flex>

          <Button type='submit'
            colorScheme="primary"
            size="lg"
            w={"full"}
            bg={"primary.700"}
            rounded={"2xl"}
            py={6} mt={8}
            fontSize={'md'}
            isDisabled={
              Object.keys(errors).length > 0 ||
              Object.keys(touched).length === 0
            }
            isLoading={registerHospitalLoading}
          >
            {button.label}
          </Button>
        </Form>
      )}
    </Formik>
  )
}