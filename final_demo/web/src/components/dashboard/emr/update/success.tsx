"use client"

import { assets } from "@/constants/assets";
import { emrSuccessModal } from "@/constants/contents/dashboard/emr/modal";
import { useEMRStore } from "@/store/patient-emr";
import { Button, Flex, Image, Modal, ModalBody, ModalContent, ModalFooter, ModalOverlay, Text } from "@chakra-ui/react";
import { useParams, useRouter } from "next/navigation";

export default function EMRUpdateSuccess({ isOpen }: { isOpen: boolean }) {
  const router = useRouter();
  const param = useParams();

  const id = param.id;

  const setUserHasEMR = useEMRStore(state => state.setUserHasEMR);

  const onClose = () => {
    setUserHasEMR(false);
    router.replace(`/dashboard/emr/${id}`)

    return;
  }

  const { title, description, button } = emrSuccessModal.update;

  return (
    <Modal
      isOpen={isOpen}
      onClose={onClose}
      size={"lg"}
      closeOnEsc={false}
      closeOnOverlayClick={false}
    >
      <ModalOverlay />
      <ModalContent rounded={"2xl"}>
        <ModalBody>
          <Flex direction={'column'} align={'center'}>
            <Image src={assets.emr_success_modal} alt="Success" w={48} />
            <Text textAlign="center" fontSize="2xl" fontWeight="bold" pt={5}>
              {title}
            </Text>
            <Text textAlign="center" fontSize="lg" color={'gray.500'}>
              {description}
            </Text>
          </Flex>
        </ModalBody>
        <ModalFooter pt={8}>
          <Button
            w={'full'}
            colorScheme="primary"
            bg={"primary.700"}
            rounded={"xl"}
            onClick={onClose}
          >
            {button.label}
          </Button>
        </ModalFooter>
      </ModalContent>
    </Modal>
  )
}