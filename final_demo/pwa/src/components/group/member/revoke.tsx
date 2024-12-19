"use client"

import { RevokeGroupAccessRequest } from "@/declarations/patient_registry/patient_registry.did";
import { encodeHashNIK, usePatientMethod } from "@/services/patients";
import { useToast, useDisclosure, Button, Icon, Modal, ModalOverlay, ModalContent, ModalBody, Flex, Text } from "@chakra-ui/react";
import { useParams } from "next/navigation";
import { FaExclamationTriangle } from "react-icons/fa";
import { HiOutlineLockClosed } from "react-icons/hi2";

interface IRevokeAccessGroupModal {
  nik: string
  onCloseModalDetail: () => void
}

export default function RevokeAccessGroupModal({ props }: { props: IRevokeAccessGroupModal }) {
  const toast = useToast();
  const { nik, onCloseModalDetail } = props
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
          description: "Failed to revoke access",
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
        revokee_nik: encodeHashNIK(nik),
        group_id
      }];

      await revokeGroupAccess(data);
      const grantedList = JSON.parse(localStorage.getItem('grantedList') ?? '')
      const newGrantedList = grantedList.list.filter((item: string) => item !== nik)
      localStorage.setItem('grantedList', JSON.stringify({ ...grantedList, list: newGrantedList }))

      onClose()
      onCloseModalDetail()
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
        type="button"
        bg={"transparent"}
        display={"flex"}
        justifyContent={"items-start"}
        columnGap={3}
        fontWeight={400}
        color={"danger.700"}
        leftIcon={
          <Icon as={HiOutlineLockClosed} boxSize={6} />
        }
        onClick={onOpen}
      >
        <Text>
          Revoke EMR Access
        </Text>
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
                After revoke, this account would not be allowed to access your EMR&apos;s<br></br>
                Are you sure?
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
                  colorScheme="red" 
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