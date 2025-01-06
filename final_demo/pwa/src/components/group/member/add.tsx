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

  const [claimConsentData, setClaimConsentData] = useState<ClaimConsentResponse | null | undefined>()
  const [showChooseRelationModal, setShowChooseRelationModal] = useState(false)

  const handleClaimConsentForGroup = () => {
    setShowChooseRelationModal(true)
    onClose()
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
              >
                Submit
              </Button>
            </Flex>
          </Flex>
        </ModalContent>
      </Modal>
      <ChooseRelationModal 
        props={{
          showChooseRelationModal,
          setShowChooseRelationModal,
          handleGetGroupDetails
        }}
      />
    </>
  )
}