"use client"

import { AddGroupMemberRequest, ClaimConsentResponse, Relation } from "@/declarations/patient_registry/patient_registry.did";
import { usePatientMethod } from "@/services/patients";
import { usePinStore } from "@/store/pin-store";
import { 
  Button,
  Flex,
  Icon,
  Modal, 
  ModalContent, 
  ModalOverlay, 
  Select, 
  Text, 
  useToast
} from "@chakra-ui/react";
import { useParams } from "next/navigation";
import { useRouter } from "next/router";
import { Dispatch, SetStateAction, useState } from "react";
import { FaChevronLeft } from "react-icons/fa6";

interface IChooseRelationModal {
  data: ClaimConsentResponse | null | undefined
  showChooseRelationModal: boolean
  setShowChooseRelationModal: Dispatch<SetStateAction<boolean>>
  handleGetGroupDetails: () => void
}

export default function ChooseRelationModal({ props }: { props: IChooseRelationModal }) {
  const {
    data,
    showChooseRelationModal,
    setShowChooseRelationModal,
    handleGetGroupDetails
  } = props

  type RelationKeys =
  "Spouse" |
  "Parent" |
  "Sibling" |
  "Child" |
  "Other"

  const relationOptions: RelationKeys[] = [
   "Spouse",
   "Parent",
   "Sibling",
   "Child",
   "Other",
  ]

  const toast = useToast()
  const { group_id } = useParams();
  const pin = usePinStore(state => state.pin);
  const [relation, setRelation] = useState<RelationKeys>(relationOptions[0])

  const { call: addGroupMember, loading: addGroupMemberLoading } = usePatientMethod({
    functionName: "add_group_member",
    refetchOnMount: false,
    onSuccess() {
      return toast({
        title: "Success Add Member",
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

  const getRelation = (): Relation => {
    const object: any = {
      [relation as RelationKeys]: null
    }
    
    return object
  }

  const handleAddGroupMember = async () => {
    try {
      const data: AddGroupMemberRequest = {
        relation : getRelation(),
        consent_code : pin,
        group_id : BigInt(Number(group_id)),
      };

      await addGroupMember([data] as any);
      handleGetGroupDetails()
      setShowChooseRelationModal(false)
    } catch (error: unknown) {
      if (error instanceof Error) {
        console.log(error.message)
      }

      console.error(error)
    }
  };

  return (
    <Modal
      isOpen={showChooseRelationModal}
      onClose={() => { setShowChooseRelationModal(false) }}
      // size={{ base: 'full', md: 'md' }}
    >
      <ModalOverlay 
        onClick={() => { setShowChooseRelationModal(false) }}
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
          <Button onClick={() => { setShowChooseRelationModal(false) }}>
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
          alignItems={"stretch"}
          justifyContent={"center"}
        >
          {
              data
                ? <Flex 
                  flexDirection={"column"}
                  rowGap={6}
                  w={"full"}
                  px={8}
                  alignItems={"center"}
                >
                  <Text
                    fontWeight={'bold'}
                    fontSize={'lg'}
                  >
                    Your Relationship with
                  </Text>
                  <Flex
                    w={"30%"}
                    aspectRatio={1/1}
                    background={"rgb(217, 217, 217)"}
                    display={"block"}
                    rounded={"xl"}
                  />
                  <Text
                    fontSize={'xl'}
                    fontWeight={'bold'}
                    textAlign={"center"}
                    px={8}
                  >
                    {data.name}
                  </Text>
                  <Flex
                    flexDirection={"column"}
                    w={"full"}
                    rowGap={3}
                  >
                    <Flex
                      w={"full"}
                      background={"rgba(219, 221, 247, 1)"}
                      rounded={"2xl"}
                      py={1}
                    >
                      <Select 
                        w={"full"}
                        value={relation}
                        onChange={(e) => { setRelation(e.target.value as RelationKeys) }}
                        textAlign={"center"}
                        outline={"none"}
                        border={0}
                      >
                        {
                          relationOptions.map((option, index) =>
                            <option key={index} value={option}>{option}</option>
                          )
                        }
                      </Select>
                    </Flex>
                    <Button
                      colorScheme="primary"
                      w={"full"}
                      bg={"primary.700"}
                      rounded={"2xl"}
                      fontSize={'sm'}
                      py={6}
                      gap={2}
                      type="button"
                      onClick={handleAddGroupMember}
                    >
                      Submit
                    </Button>
                  </Flex>
                </Flex>
                : data === undefined
                  ? <Text>Loading</Text>
                  : <></>
            }
        </Flex>
      </ModalContent>
    </Modal>
  )
}