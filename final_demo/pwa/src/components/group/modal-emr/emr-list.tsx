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
  Image
} from "@chakra-ui/react";
import { useEffect, useState } from "react";
import { HiOutlineEye } from "react-icons/hi2";
import EMRMemberDetail from "./emr-detail";

interface IEMRListModalProps {
  group_id: string
  nik: string
}

export default function EMRListModal({ props }: { props: IEMRListModalProps }) {
  const { group_id, nik } = props
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
      const { Ok }: any = data;
      if (Ok) setEmrGroupInformation(Ok);
      else setEmrGroupInformation(null);
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
        <Text>See Group EMRs</Text>
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
          background={"transparent"}
        >
          <Flex 
            marginTop={"auto"}
            flexDirection={"column"}
            rowGap={6}
            h={"full"}
            py={12}
            width={"full"}
            background={"rgba(230, 238, 252, 1)"}
            px={8}
            onClick={(e) => e.stopPropagation()}
          >
            <Button
              type="button"
              bg={"transparent"}
              display={"flex"}
              justifyContent={"items-start"}
              columnGap={3}
              fontWeight={400}
              color={"primary.700"}
              onClick={onClose}
            >
              <Text>Close</Text>
            </Button>
            {emrGroupInformation === undefined && <Text>Please wait ..</Text> }
            {emrGroupInformation && 
              <Flex>
                {
                  emrGroupInformation.emrs.map((emr, index) =>
                    <Flex key={index}>
                      <Text>{emr.hospital_name}</Text>
                      <EMRMemberDetail props={{
                        emr_id: emr.header.emr_id,
                        provider_id: emr.header.provider_id,
                        registry_id: emr.header.registry_id,
                        nik,
                        group_id
                       }} />
                    </Flex>
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