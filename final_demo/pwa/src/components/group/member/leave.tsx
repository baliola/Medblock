"use client"

import { LeaveGroupRequest } from "@/declarations/patient_registry/patient_registry.did";
import { usePatientMethod } from "@/services/patients";
import { 
  Button,
  Icon,
  Modal, 
  ModalContent, 
  ModalOverlay, 
  useDisclosure, 
  useToast,
  ModalBody,
  Flex,
  Text
} from "@chakra-ui/react";
import { useParams } from "next/navigation";
import { useRouter } from "next/navigation";
import { FaExclamationTriangle } from "react-icons/fa";
import { HiOutlineArrowRightOnRectangle } from "react-icons/hi2";

interface ILeaveGroupModal {
  isLeader: boolean
}

export default function LeaveGroupModal({ props }: { props: ILeaveGroupModal }) {
  const toast = useToast();
  const router = useRouter()
  const { group_id } = useParams();
  const { isOpen, onOpen, onClose } = useDisclosure();
  const { isLeader } = props

  const { call: leaveGroup, loading: leaveGroupLoading } = usePatientMethod({
    functionName: "leave_group",
    refetchOnMount: false,
    onSuccess(data) {
       toast({
        title: "Successfully left the group",
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

  const handleLeaveGroup = async () => {
    try {
      const data: LeaveGroupRequest[] | any = [{
        group_id: BigInt(Number(group_id)),
      }];

      await leaveGroup(data);
      onClose()
      router.push("/group")
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
          <Icon as={HiOutlineArrowRightOnRectangle} boxSize={6} />
        }
        onClick={onOpen}
      >
        <Text>
          Leave Group
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
                When you leave, {isLeader ? 'this group is deleted and all the member will be removed' : 'you must be re-invited to join this group'},<br></br>Are you sure?
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
                  disabled={leaveGroupLoading}
                >
                  Cancel
                </Button>
                <Button 
                  colorScheme="red" 
                  w={'full'} 
                  onClick={handleLeaveGroup} 
                  isLoading={leaveGroupLoading}
                >
                  Leave
                </Button>
              </Flex>
            </Flex>
          </ModalBody>
        </ModalContent>
      </Modal>
    </>
  )
}