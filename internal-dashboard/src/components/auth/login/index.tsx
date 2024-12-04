"use client"

import { loginSchema } from "@/libs/yup/auth";
import { Button, FormControl, FormErrorMessage, FormLabel, Stack, useToast } from "@chakra-ui/react";
import { Formik, Form, Field } from "formik";
import { useState } from "react";
import { signIn } from "next-auth/react";
import { useRouter } from "next/navigation";
import { PasswordInput } from "@/components/input/password";
import { TextInput } from "@/components/input/text";

export default function FormLogin() {
  const toast = useToast();
  const router = useRouter();
  const [loading, setLoading] = useState<boolean>(false);

  const onSubmit = async (values: { email: string; password: string }) => {
    setLoading(true);
    try {
      const response = await signIn('credentials', {
        email: values.email,
        password: values.password,
        redirect: false,
      });

      if (response?.ok) {
        router.replace("/");
        return toast({
          title: "Sukses Login!",
          description: "Login telah berhasil.",
          status: "success",
        });
      }
      throw new Error("Email atau Password tidak sama.")
    } catch (error) {
      if (error instanceof Error) {
        toast({
          title: "Login Gagal!",
          description: error.message,
          status: "error",
          position: 'top',
          duration: 5000
        });
      }
    } finally {
      setLoading(false);
    }
  }

  return (
    <Formik
      initialValues={{ email: '', password: '' }}
      validationSchema={loginSchema}
      onSubmit={(values) => onSubmit(values)}
    >
      {({ handleSubmit, errors, touched }) => (
        <Form onSubmit={handleSubmit}>
          <Stack w={'sm'} spacing={5}>
            <FormControl
              isRequired
              isInvalid={!!errors.email && touched.email}
            >
              <FormLabel htmlFor="email">Email</FormLabel>
              <Field as={TextInput} type="email" name="email" placeholder="Email" />
              <FormErrorMessage>{errors.email}</FormErrorMessage>
            </FormControl>
            <FormControl
              isRequired
              isInvalid={!!errors.password && touched.password}
            >
              <FormLabel htmlFor="password">Password</FormLabel>
              <Field as={PasswordInput} type="password" name="password" placeholder="Password" />
              <FormErrorMessage>{errors.password}</FormErrorMessage>
            </FormControl>
            <Button type="submit"
              colorScheme="primary"
              bg={'primary.700'}
              py={6}
              isLoading={loading}
              mt={4}
            >
              Login
            </Button>
          </Stack>
        </Form>
      )}
    </Formik>
  )
}