"use client"

import { RevokeGroupAccessRequest } from "@/declarations/patient_registry/patient_registry.did";
import { usePatientMethod } from "@/services/patients";
import { useToast, useDisclosure, Button, Icon, Modal, ModalOverlay, ModalContent, ModalBody, Flex, Text } from "@chakra-ui/react";
import { useParams } from "next/navigation";
import { FaExclamationTriangle } from "react-icons/fa";
import { HiLockClosed } from "react-icons/hi2";

interface IRevokeAccessGroupModal {
  nik: string
}

export default function RevokeAccessGroupModal({ props }: { props: IRevokeAccessGroupModal }) {
  const { nik } = props

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

  const handleRevokeGroupAccess = async () => {
    try {
      const data: RevokeGroupAccessRequest[] | any = [{
        grantee_nik: nik
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
              <Icon 
                as={FaExclamationTriangle} 
                color={"red.500"} 
                boxSize={16} 
                mx={"auto"}
              />
              <Text
                fontSize={'lg'}
                textAlign={"center"}
                px={4}
              >
                Continue to revoke EMR access to this group?
              </Text>
              <Flex
                mt={3}
                columnGap={3}
              >
                <Button 
                  variant={'outline'} 
                  w={'full'} 
                  bg={"white"}
                  onClick={onClose} 
                  disabled={revokeGroupAccessLoading}
                >
                  Cancel
                </Button>
                <Button 
                  colorScheme="danger" 
                  w={'full'} 
                  onClick={handleRevokeGroupAccess} 
                  isLoading={revokeGroupAccessLoading}
                >
                  Revoke
                </Button>
              </Flex>
            </Flex>
          </ModalBody>
        </ModalContent>
      </Modal>
    </>
  )
}