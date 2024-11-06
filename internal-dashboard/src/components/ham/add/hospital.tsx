"use client";

import { TextInput } from "@/components/input/text";
import { providerCanisterId } from "@/config/canisters/providers.canister";
import { uamAddHospitaldModal } from "@/constants/contents/uam/add";
import useProvider from "@/hooks/useProvider";
import { addHospitalSchema } from "@/libs/yup/ham";
import { ProviderActor } from "@/services/providers";
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
  Box
} from "@chakra-ui/react";
import { Formik, Form, Field } from "formik";
import { ReactElement } from "react";
import { FaPlus } from "react-icons/fa";

const AddHospitalModal = (): ReactElement => {
  const { isOpen, onOpen, onClose } = useDisclosure();
  const { registerHospitalLoading, handleRegisterHospital } = useProvider()

  const { title } = uamAddHospitaldModal;

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
        <Text>Add Hospital</Text>
      </Button>
      <Modal isOpen={isOpen} onClose={onClose} closeOnOverlayClick={true}>
        <ModalOverlay />
        <ModalContent rounded={"xl"}>
          <ModalHeader>{title}</ModalHeader>
          <ModalBody>
            <Formik
              initialValues={{ name: "", address: "", principal: "" }}
              validationSchema={addHospitalSchema}
              onSubmit={() => {}}
            >
              {({ errors, touched, isSubmitting, values }) => (
                <Form
                  onSubmit={(e) => {
                    e.preventDefault();
                    handleRegisterHospital(values, onClose);
                  }}
                >
                  <Stack w={"full"} spacing={5}>
                    <FormControl
                      isRequired
                      isInvalid={!!errors.name && touched.name}
                    >
                      <FormLabel htmlFor="name">Name</FormLabel>
                      <Field
                        as={TextInput}
                        type="name"
                        name="name"
                        placeholder=""
                      />
                      <FormErrorMessage>{errors.name}</FormErrorMessage>
                    </FormControl>
                    <FormControl
                      isRequired
                      isInvalid={!!errors.address && touched.address}
                    >
                      <FormLabel htmlFor="address">Address</FormLabel>
                      <Field
                        as={TextInput}
                        type="address"
                        name="address"
                        placeholder=""
                      />
                      <FormErrorMessage>{errors.address}</FormErrorMessage>
                    </FormControl>
                    <FormControl
                      isRequired
                      isInvalid={!!errors.principal && touched.principal}
                    >
                      <FormLabel htmlFor="principal">Principal</FormLabel>
                      <Field
                        as={TextInput}
                        type="principal"
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
                        isLoading={isSubmitting || registerHospitalLoading}
                        isDisabled={!values.name || !values.address}
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

export default function HAMAddHospitalModal() {
  return (
    <ProviderActor canisterId={providerCanisterId}>
      <AddHospitalModal />
    </ProviderActor>
  );
}
