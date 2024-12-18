"use client"

import { EmrListPatientResponse } from "@/declarations/patient_registry/patient_registry.did";
import { usePatientQuery } from "@/services/patients";
import { 
  Button,
  Icon,
  Text, 
  useDisclosure,
  Flex,
  Modal,
  ModalContent,
  ModalOverlay,
  Image,
  useToast
} from "@chakra-ui/react";
import { useEffect, useState } from "react";
import { HiOutlineChevronLeft, HiOutlineEye } from "react-icons/hi2";
import EMRMemberDetail from "./emr-detail";
import { FaChevronLeft, FaHospital } from "react-icons/fa6";
import { BiChevronLeft } from "react-icons/bi";

interface IEMRListModalProps {
  group_id: string
  nik: string
  name: string
}

export default function EMRListModal({ props }: { props: IEMRListModalProps }) {
  const toast = useToast()
  const { group_id, nik, name } = props
  const { isOpen, onOpen, onClose } = useDisclosure();

  const [emrGroupInformation, setEmrGroupInformation] = useState<
    EmrListPatientResponse | null | undefined
  >(undefined);
  
  const {
    call: getEmrGroupInformation,
    loading: loadingGetEmrGroupInformation,
  } = usePatientQuery({
    functionName: "view_group_member_emr_information",
    refetchOnMount: false,
    onSuccess(data) {
      console.log(data);
      const { Ok, Err }: any = data;
      if (Ok) {
        setEmrGroupInformation(Ok);
      } else if (Err) {
        setEmrGroupInformation(null);
      }
    },
    onError(error) {
      setEmrGroupInformation(null);
      console.error(error);
    },
  });

  useEffect(() => {
    if (isOpen) {
      getEmrGroupInformation([
        {
          page: BigInt(1),
          limit: BigInt(1),
          group_id: group_id,
          member_nik: nik,
        },
      ] as any);
    }
  }, [isOpen])
  
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
        onClick={onOpen}
        leftIcon={<Icon as={HiOutlineEye} boxSize={6} />}
      >
        <Text>See Member EMRs</Text>
      </Button>
      <Modal
        isOpen={isOpen}
        onClose={onClose}
        size={{ base: 'full' }}
      >
        <ModalOverlay 
          onClick={onClose}
          background={"transparent"}
        />
        <ModalContent
          border={"none"}
          shadow={"none"}
          marginX={0}
          marginY={"auto"}
          w={"full"}
          h={"full"}
          display={"flex"}
          onClick={onClose}
          background={"white"}
        >
          <Flex 
            marginTop={"auto"}
            flexDirection={"column"}
            rowGap={6}
            h={"full"}
            width={"full"}
            background={"white"}
            onClick={(e) => e.stopPropagation()}
          >
            <Flex 
              pt={6}
              w={"full"}
              alignItems={"center"}
              px={6}
              gap={3}
            >
              <Button
                type="button"
                bg={"white"}
                columnGap={3}
                fontWeight={400}
                color={"gray.500"}
                display={"flex"}
                justifyContent={"center"}
                w={"fit-content"}
                leftIcon={<Icon as={HiOutlineChevronLeft} boxSize={10} />}
                onClick={onClose}
                width={10}
                height={10}
              />
              <Text 
                textTransform={"capitalize"} 
                fontSize={"2xl"}
                fontWeight={"700"}
              >
                {name} EMR&apos;s
              </Text>
            </Flex>
            {emrGroupInformation === undefined && <Text px={8} fontSize={"xl"}>Please wait ..</Text> }
            {emrGroupInformation === null && <Text px={8} fontSize={"xl"}>Unable to access this user EMR</Text> }
            {emrGroupInformation && 
              <Flex direction={'column'} gap={4} bg={'white'} flex={1} px={8}>
                {
                  emrGroupInformation.emrs.map((emr, index) =>
                    <EMRMemberDetail props={{
                      emr,
                      nik,
                      group_id
                    }} key={index} />
                  )
                }
              </Flex>
            }
          </Flex>
        </ModalContent>
      </Modal>
    </>
  )
}