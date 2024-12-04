"use client"

import { RevokeGroupAccessRequest } from "@/declarations/patient_registry/patient_registry.did";
import { grantGroupAccessSchema } from "@/libs/yup/grant-group-access";
import { encodeHashNIK, usePatientMethod } from "@/services/patients";
import { useToast, useDisclosure, Button, Icon, Modal, ModalOverlay, ModalContent, ModalBody, Flex, Text, Input } from "@chakra-ui/react";
import { Formik, Form } from "formik";
import { useParams } from "next/navigation";
import { HiLockClosed } from "react-icons/hi2";

interface IRevokeAccessGroupModal {
}

export default function RevokeAccessGroupModal({ props }: { props: IRevokeAccessGroupModal }) {
  const toast = useToast();
  const { group_id } = useParams();
  const { isOpen, onOpen, onClose } = useDisclosure();

  const { call: revokeGroupAccess, loading: revokeGroupAccessLoading } = usePatientMethod({
    functionName: "revoke_group_access",
    refetchOnMount: false,
    onSuccess(data) {
       toast({
        title: "Successfully revoke group access",
        description: "You can now proceed",
        isClosable: true,
        duration: 5000,
        status: "success",
        position: "top-right",
      })
    },
    onError(err) {
      if (err instanceof Error) {
        toast({
          title: "Error!",
          description: "Failed to leave group",
          status: "error",
          duration: 5000,
          isClosable: true,
          position: "top-right"
        });
      } else {
        toast({
          title: "Error!",
          description: "Something went wrong!",
          isClosable: true,
          duration: 5000,
          position: "top-right",
          status: "error"
        })
      }

      throw err;
    },
  });

  const handleRevokeGroupAccess = async (nik: string) => {
    try {
      const data: RevokeGroupAccessRequest[] | any = [{
        grantee_nik: encodeHashNIK(nik)
      }];

      await revokeGroupAccess(data);
      onClose()
    } catch (error: unknown) {
      if (error instanceof Error) {
        console.log(error.message)
      }

      console.error(error)
    }
  };
  
  return (
    <>
      <Button
        colorScheme="danger"
        w={"full"}
        rounded={"2xl"}
        fontSize={'sm'}
        py={6}
        gap={2}
        mt={4}
        leftIcon={
          <Icon as={HiLockClosed} boxSize={5} />
        }
        onClick={onOpen}
      >
        Revoke EMR Access
      </Button>
      <Modal
        isOpen={isOpen}
        onClose={onClose}
        // size={{ base: 'full', md: 'md' }}
      >
        <ModalOverlay 
          onClick={onClose}
          background={"rgba(80, 80, 80, 0.6)"}
          backdropFilter="blur(8px)"
        />
        <ModalContent
          marginX={0}
          marginY={"auto"}
          w={"full"}
          h={"fit-content"}
          display={"flex"}
          background={"transparent"}
          boxShadow={"none"}
        >
          <ModalBody
            display={"flex"}
            flexDirection={"column"}
            rowGap={4}
            px={4}
          >
            <Flex
              display={"flex"}
              flexDirection={"column"}
              rowGap={3}
              bg={"white"}
              px={6}
              py={6}
              rounded={"lg"}
            >
              <Formik
                initialValues={{ grantee_nik: "" }}
                validationSchema={grantGroupAccessSchema}
                onSubmit={() => {}}
              >
                {({ errors, touched, isSubmitting, values, setFieldValue }) => (
                  <Form
                    onSubmit={(e) => {
                      e.preventDefault();
                      handleRevokeGroupAccess(values.grantee_nik);
                    }}
                  >
                    <Flex 
                      margin={"auto"}
                      flexDirection={"column"}
                      rowGap={6}
                      width={"full"}
                      px={8}
                    >
                      <Text
                        fontSize={'xl'}
                        fontWeight={'bold'}
                        textAlign={"center"}
                      >
                        NIK
                      </Text>
                      <Input
                        value={values.grantee_nik}
                        onChange={(e) => { setFieldValue('grantee_nik', e.target.value) }}
                        fontSize={'lg'}
                        fontWeight={'bold'}
                        textAlign={"center"}
                        borderColor={'#A1A2A6'}
                        background={"#DBDDF7"}
                        rounded={'xl'}
                        py={6}
                        focusBorderColor="transparent"
                        _placeholder={{ color: "rgba(93, 93, 93, 1)" }}
                      />
                      <Button
                        colorScheme="primary"
                        w={"full"}
                        bg={"primary.700"}
                        rounded={"2xl"}
                        fontSize={'lg'}
                        py={6}
                        gap={2}
                        type="submit"
                        isLoading={revokeGroupAccessLoading}
                        isDisabled={!values.grantee_nik}
                      >
                        Submit
                      </Button>
                    </Flex>
                  </Form>
                )}
              </Formik>
            </Flex>
          </ModalBody>
        </ModalContent>
      </Modal>
    </>
  )
}