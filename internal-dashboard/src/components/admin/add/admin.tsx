"use client";

import { TextInput } from "@/components/input/text";
import { patientCanisterId } from "@/config/canisters/patient.canister";
import { uamAddAdmindModal } from "@/constants/contents/admin/add";
import useAdmin from "@/hooks/useAdmin";
import { addAdminSchema } from "@/libs/yup/admin";
import { PatientActor } from "@/services/patients";
import {
  Button,
  FormControl,
  FormErrorMessage,
  FormLabel,
  Modal,
  ModalBody,
  ModalContent,
  ModalFooter,
  ModalHeader,
  ModalOverlay,
  Stack,
  useDisclosure,
  Text,
  Icon,
  Box,
} from "@chakra-ui/react";
import { Formik, Form, Field } from "formik";
import { ReactElement } from "react";
import { FaPlus } from "react-icons/fa";

const AddAdminModalForm = (): ReactElement => {
  const { isOpen, onOpen, onClose } = useDisclosure();
  const { bindAdminLoading, handleBindAdmin } = useAdmin()

  const { title } = uamAddAdmindModal;

  return (
    <>
      <Button
        type="button"
        onClick={onOpen}
        bg={'primary.700'}
        colorScheme="primary"
        color={'white'}
        flex={1}
        alignItems={"center"}
        gap={3} 
      >
        <Box 
          p={0.5}
          borderRadius={"100%"}
          bg={"rgba(255, 255, 255, 0.5)"}
          display={"flex"}
          w={5}
          h={5}
        >
          <Icon
            m={"auto"}
            as={FaPlus}
            w={2}
            h={2}
          />
        </Box>
        <Text>Add Admin</Text>
      </Button>
      <Modal isOpen={isOpen} onClose={onClose} closeOnOverlayClick={true}>
        <ModalOverlay />
        <ModalContent rounded={"xl"}>
          <ModalHeader>{title}</ModalHeader>
          <ModalBody>
            <Formik
              initialValues={{ nik: "", principal: "" }}
              validationSchema={addAdminSchema}
              onSubmit={() => {}}
            >
              {({ errors, touched, isSubmitting, values }) => (
                <Form
                  onSubmit={(e) => {
                    e.preventDefault();
                    handleBindAdmin(values, onClose);
                  }}
                >
                  <Stack w={"full"} spacing={5}>
                    <FormControl
                      isRequired
                      isInvalid={!!errors.nik && touched.nik}
                    >
                      <FormLabel htmlFor="nik">NIK</FormLabel>
                      <Field
                        as={TextInput}
                        type="text"
                        name="nik"
                        placeholder=""
                      />
                      <FormErrorMessage>{errors.nik}</FormErrorMessage>
                    </FormControl>
                    <FormControl
                      isRequired
                      isInvalid={!!errors.principal && touched.principal}
                    >
                      <FormLabel htmlFor="principal">Principal</FormLabel>
                      <Field
                        as={TextInput}
                        type="text"
                        name="principal"
                        placeholder=""
                      />
                      <FormErrorMessage>{errors.principal}</FormErrorMessage>
                    </FormControl>
                    <ModalFooter gap={3}>
                      <Button
                        colorScheme="red"
                        variant={"ghost"}
                        w={"full"}
                        onClick={onClose}
                        isDisabled={isSubmitting}
                        type="button"
                      >
                        Cancel
                      </Button>
                      <Button
                        type="submit"
                        colorScheme="primary"
                        bg={"primary.700"}
                        w={"full"}
                        isLoading={isSubmitting || bindAdminLoading}
                        isDisabled={!values.nik || !values.principal}
                      >
                        Submit
                      </Button>
                    </ModalFooter>
                  </Stack>
                </Form>
              )}
            </Formik>
          </ModalBody>
        </ModalContent>
      </Modal>
    </>
  );
};

export default function AddAdminModal() {
  return (
    <PatientActor canisterId={patientCanisterId}>
      <AddAdminModalForm />
    </PatientActor>
  );
}
