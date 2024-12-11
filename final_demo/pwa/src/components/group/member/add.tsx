"use client"

import InputPIN from "@/components/pin/input";
import { ClaimConsentRequest, ClaimConsentResponse, Result_1 } from "@/declarations/patient_registry/patient_registry.did";
import { usePatientMethod } from "@/services/patients";
import { usePinStore } from "@/store/pin-store";
import { 
  Button,
  Flex,
  Icon,
  Modal, 
  ModalContent, 
  ModalOverlay, 
  Text, 
  Image,
  useDisclosure, 
  useToast
} from "@chakra-ui/react";
import { useState } from "react";
import { FaChevronLeft, FaUserPlus } from "react-icons/fa6";
import ChooseRelationModal from "./choose-relation";

interface IAddMemberModal {
  handleGetGroupDetails: () => void
}

export default function AddMemberModal({ props }: { props: IAddMemberModal }) {
  const toast = useToast();
  const { isOpen, onOpen, onClose } = useDisclosure();
  const { handleGetGroupDetails } = props

  const pin = usePinStore(state => state.pin);
  const setPin = usePinStore(state => state.setPin);

  const [claimConsentData, setClaimConsentData] = useState<ClaimConsentResponse | null | undefined>()
  const [showChooseRelationModal, setShowChooseRelationModal] = useState(false)

  const { call: claimConsentForGroup, loading: claimConsentForGroupLoading } = usePatientMethod({
    functionName: "claim_consent_for_group",
    refetchOnMount: false,
    onSuccess(data) {
      const result: Result_1 | undefined = data

      if (result && Object.keys(result)[0] === 'Ok') {
        setClaimConsentData(data)
        setShowChooseRelationModal(true)
        onClose()

        return toast({
          title: "Success Entered Pin",
          description: "You can now proceed",
          isClosable: true,
          duration: 5000,
          status: "success",
          position: "top-right",
        })
      } else if (result && Object.keys(result)[0] === 'Err') {
        const error = result['Err'] ?? "Something went wrong!"

        return toast({
          title: "Error!",
          description: error,
          status: "error",
          duration: 5000,
          isClosable: true,
          position: "top-right"
        })
      }
    },
    onError(err) {
      if (err instanceof Error) {
        toast({
          title: "Error!",
          description: "Pin does not match",
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

  const handleClaimConsentForGroup = async () => {
    try {
      const data: ClaimConsentRequest[] | any | undefined = [{
        code: pin,
      }];

      await claimConsentForGroup(data);
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
        rounded={"2xl"}
        fontSize={'sm'}
        py={6}
        gap={2}
        // mt={"auto"}
        leftIcon={
          <Icon as={FaUserPlus} boxSize={5} />
        }
        onClick={onOpen}
      >
        Add Member
      </Button>
      <Modal
        isOpen={isOpen}
        onClose={onClose}
        // size={{ base: 'full', md: 'md' }}
      >
        <ModalOverlay 
          onClick={onClose}
          background={"white"}
        />
        <ModalContent
          marginX={0}
          marginY={"auto"}
          w={"full"}
          h={"full"}
          display={"flex"}
          background={"transparent"}
          boxShadow={"none"}
        >
          <Flex
            align={'center'}
            justify={'space-between'}
            gap={3}
            px={1}
            py={3}
            w={"full"}
          >
            <Button onClick={onClose}>
              <Icon 
                as={FaChevronLeft} 
                boxSize={5} 
                color={'neutral.500'} 
              />
            </Button>
          </Flex>
          <Flex
            direction={"column"}
            h={"full"}
            w={"full"}
            alignItems={"center"}
            justifyContent={"center"}
          >
            <Image 
              src="/assets/female-doctor.png" 
              alt="Add Member" 
              width={"40%"}
              marginX={"auto"}
              marginBottom={12}
            />
            <Flex 
              flexDirection={"column"}
              rowGap={6}
              width={"full"}
              px={8}
            >
              <Flex
                direction={"column"}
              >
                <Text
                  fontSize={'2xl'}
                  fontWeight={'bold'}
                  textAlign={"center"}
                >
                  Add Your Group
                </Text>
                <Text
                  fontSize={'2xl'}
                  fontWeight={'bold'}
                  textAlign={"center"}
                >
                  Consent Code
                </Text>
                <Text
                  textAlign={"center"}
                  mt={2}
                >
                  To Add Member or Partner
                </Text>
              </Flex>
              <Flex>
                <InputPIN />
              </Flex>
              <Button
                colorScheme="primary"
                w={"full"}
                bg={"primary.700"}
                rounded={"2xl"}
                fontSize={'sm'}
                py={6}
                gap={2}
                mt={4}
                type="button"
                onClick={handleClaimConsentForGroup}
                isLoading={claimConsentForGroupLoading}
              >
                Submit
              </Button>
            </Flex>
          </Flex>
        </ModalContent>
      </Modal>
      <ChooseRelationModal 
        props={{
          data: claimConsentData,
          showChooseRelationModal,
          setShowChooseRelationModal,
          handleGetGroupDetails
        }}
      />
    </>
  )
}