"use client";

import { GrantGroupAccessRequest } from "@/declarations/patient_registry/patient_registry.did";
import { encodeHashNIK, usePatientMethod } from "@/services/patients";
import {
  useToast,
  useDisclosure,
  Button,
  Icon,
  Modal,
  ModalOverlay,
  ModalContent,
  ModalBody,
  Flex,
  Text,
} from "@chakra-ui/react";
import { useParams } from "next/navigation";
import { FaExclamationTriangle } from "react-icons/fa";
import { HiOutlineLockOpen } from "react-icons/hi2";

interface IGrantAccessGroupModal {
  nik: string;
}

export default function GrantAccessGroupModal({
  props,
}: {
  props: IGrantAccessGroupModal;
}) {
  const { nik } = props;
  const toast = useToast();
  const { group_id } = useParams();
  const { isOpen, onOpen, onClose } = useDisclosure();

  const { call: grantGroupAccess, loading: grantGroupAccessLoading } =
    usePatientMethod({
      functionName: "grant_group_access",
      refetchOnMount: false,
      onSuccess(data) {
        toast({
          title: "Successfully grant group access",
          description: "You can now proceed",
          isClosable: true,
          duration: 5000,
          status: "success",
          position: "top-right",
        });
      },
      onError(err) {
        if (err instanceof Error) {
          toast({
            title: "Error!",
            description: "Failed to grant access",
            status: "error",
            duration: 5000,
            isClosable: true,
            position: "top-right",
          });
        } else {
          toast({
            title: "Error!",
            description: "Something went wrong!",
            isClosable: true,
            duration: 5000,
            position: "top-right",
            status: "error",
          });
        }

        throw err;
      },
    });

  const handleGrantGroupAccess = async () => {
    try {
      const data: GrantGroupAccessRequest[] | any = [
        {
          group_id: group_id,
          grantee_nik: nik,
        },
      ];

      console.log("access data group emr", data);

      await grantGroupAccess(data);
      onClose();
    } catch (error: unknown) {
      if (error instanceof Error) {
        console.log(error.message);
      }

      console.error(error);
    }
  };

  return (
    <>
      <Button
        type="button"
        bg={"transparent"}
        display={"flex"}
        justifyContent={"items-start"}
        columnGap={3}
        fontWeight={400}
        color={"primary.700"}
        leftIcon={<Icon as={HiOutlineLockOpen} boxSize={6} />}
        onClick={onOpen}
      >
        <Text>Grant EMR Access</Text>
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
              <Icon
                as={FaExclamationTriangle}
                color={"red.500"}
                boxSize={16}
                mx={"auto"}
              />
              <Text fontSize={"lg"} textAlign={"center"} px={4}>
                Once you give access to this account, it will be allowed to
                access your EMR&apos;s<br></br>
                Are you sure?
              </Text>
              <Flex mt={3} columnGap={3}>
                <Button
                  variant={"outline"}
                  w={"full"}
                  bg={"white"}
                  onClick={onClose}
                  disabled={grantGroupAccessLoading}
                >
                  Cancel
                </Button>
                <Button
                  colorScheme="primary"
                  w={"full"}
                  onClick={handleGrantGroupAccess}
                  isLoading={grantGroupAccessLoading}
                >
                  Grant
                </Button>
              </Flex>
            </Flex>
          </ModalBody>
        </ModalContent>
      </Modal>
    </>
  );
}
