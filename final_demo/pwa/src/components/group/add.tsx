"use client"

import { CreateGroupRequest } from "@/declarations/patient_registry/patient_registry.did";
import { addPatientGroupSchema } from "@/libs/yup/group-registration";
import { usePatientMethod } from "@/services/patients";
import { 
  Button,
  Flex,
  Icon,
  Input,
  Modal, 
  ModalBody, 
  ModalContent, 
  ModalOverlay, 
  Text, 
  useDisclosure, 
  useToast
} from "@chakra-ui/react";
import { Formik, Form, Field } from "formik";
import { useState } from "react";
import { FaUserPlus } from "react-icons/fa6";

interface IAddGroupModalProps {
  getUserGroups: () => Promise<void>
}

export default function AddGroupModal ({ props }: { props: IAddGroupModalProps }) {
  const toast = useToast();
  const { isOpen, onOpen, onClose } = useDisclosure();

  const { call: createGroup, loading: createGroupLoading } = usePatientMethod({
    functionName: "create_group",
    refetchOnMount: false,
    onSuccess() {
      toast({
        title: "",
        description: "Success create new group.",
        isClosable: true,
        duration: 5000,
        position: "top-right",
        status: "success"
      })

      return;
    },
    onError(err) {
      if (err instanceof Error) {
        toast({
          title: "Error!",
          description: err.message,
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

  const handleCreateGroup = async (values: {
    name: string;
  }) => {
    try {
      const data: CreateGroupRequest[] | any | undefined = [{
        name: values.name,
      }];

      await createGroup(data);
      await props.getUserGroups()
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
        colorScheme="primary"
        w={"full"}
        bg={"primary.700"}
        rounded={"xl"}
        fontSize={'sm'}
        py={6}
        gap={2}
        leftIcon={
          <Icon as={FaUserPlus} boxSize={4} />
        }
        onClick={onOpen}
      >
        Create Group
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
          h={40}
          display={"flex"}
          background={"transparent"}
          boxShadow={"none"}
        >
          <ModalBody>
            <Formik
              initialValues={{ name: "" }}
              validationSchema={addPatientGroupSchema}
              onSubmit={() => {}}
            >
              {({ errors, touched, isSubmitting, values, setFieldValue }) => (
                <Form
                  onSubmit={(e) => {
                    e.preventDefault();
                    handleCreateGroup(values);
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
                      Name Your Group
                    </Text>
                    <Input
                      value={values.name}
                      onChange={(e) => { setFieldValue('name', e.target.value) }}
                      placeholder={'Doe Group'}
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
                      isLoading={createGroupLoading}
                      isDisabled={!values.name}
                    >
                      Submit
                    </Button>
                  </Flex>
                </Form>
              )}
            </Formik>
          </ModalBody>
        </ModalContent>
      </Modal>
    </>
  )
}